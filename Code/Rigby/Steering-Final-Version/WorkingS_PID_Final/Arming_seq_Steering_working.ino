const int POT_PIN = 34;

// Motor pins
const int LPWM = 18;
const int RPWM = 19; //idk i copied the ones from the ArmingSeq code
const int L_EN = 25;
const int R_EN = 26;
const int LED = 14;

// setup
bool isArmed = false;

float angle_target = 0.0;
float angle_current = 0.0;
float error = 0.0;

float kp = 1.2;
float angle_max = 10.0;
float range = 1.0;

unsigned long lastTime = 0;
unsigned long sample = 20;

const float ADC_MAX = 4095.0; //Digital Value


// Motor inputs
void motorStop() {
  analogWrite(LPWM, 0); //Default pos
  analogWrite(RPWM, 0);
}

void driverDisable() {
  motorStop();
  digitalWrite(L_EN, LOW);  //Should it be 0 or low? we'll see
  digitalWrite(R_EN, LOW);
  digitalWrite(LED, LOW);
  isArmed = false;
}

void driverEnable() {
  motorStop();
  digitalWrite(L_EN, HIGH);
  digitalWrite(R_EN, HIGH); //Keep the engine active
  digitalWrite(LED, HIGH);
  isArmed = true;
}

void driveMotor(float error, int pwm) {
  if (!isArmed) {
    motorStop();  //Call the Function to set the motor to Low
    return;
  }

  if (error >= range) {
    // Go right
    analogWrite(LPWM, 0);
    analogWrite(RPWM, pwm);
  }
  else if (error <= -range) {
    // Go left
    analogWrite(LPWM, pwm);
    analogWrite(RPWM, 0);
  }
  else {
    motorStop();
  }
}


// Setup
void setup() {
  Serial.begin(115200); //baud rate
  Serial.setTimeout(100);

  pinMode(POT_PIN, INPUT);

  pinMode(LPWM, OUTPUT);
  pinMode(RPWM, OUTPUT);
  pinMode(L_EN, OUTPUT);
  pinMode(R_EN, OUTPUT);
  //pinMode(LED, OUTPUT); unless you want to do some testing.

  driverDisable();

  Serial.println("OFF");
  Serial.println("Command A to enable motors.");
  Serial.println("Command F to disable motors.");
  Serial.println("Arming Done,target angle: ");
}


// Main loop
void loop() {
  if (Serial.available() > 0) {
    String input = Serial.readStringUntil('\n');
    input.trim();

    if (input == "A") {
      driverEnable();
      Serial.println("SYS ARMED & ONLINE");
    }
    else if (input == "F") {
      driverDisable();
      Serial.println("SYS DISARMED & MOTORS OFF");
    }
    else {
      float angle_input = input.toFloat();
      angle_target = constrain(angle_input, -angle_max, angle_max);

      Serial.print("New angle: ");
      Serial.println(angle_target);
    }
  }

  unsigned long currentTime = millis();

  if (currentTime - lastTime >= sample) {
    lastTime = currentTime;

    int pot_raw = analogRead(POT_PIN);

    // ADC to angle
    angle_current = ((float)pot_raw / ADC_MAX) * (2.0 * angle_max) - angle_max;

    // Round it (or not up to you)
    angle_current = round(angle_current * 10.0) / 10.0;

    error = angle_target - angle_current;

    int pwm = abs(error * kp * (255.0 / angle_max));
    pwm = constrain(pwm, 0, 255);

    driveMotor(error, pwm);

    Serial.print("Armed: ");
    Serial.print(isArmed ? "Y" : "N");

    Serial.print(" | Target: ");
    Serial.print(angle_target);

    Serial.print(" | Current: "); //To debug
    Serial.print(angle_current);

    Serial.print(" | Raw pot: ");
    Serial.print(pot_raw);

    Serial.print(" | Err: ");
    Serial.print(error);

    Serial.print(" | PWM: ");
    Serial.println(pwm);
  }
}