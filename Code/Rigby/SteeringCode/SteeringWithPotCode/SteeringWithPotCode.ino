#include <FTSerial.h>

FTSerial ftSerial(Serial, 24);

//Hard limits (to not break mechanically, only change if range is tested/safe)
const float ANGLE_LIMIT_DEG = 80.0;
const float STOP_BAND_DEG = 1.0; //How close is it to required angle before stopping (tolerance)

  //make sure adc left and right are never the same, make left < right at all times (avoids dividing by 0 later) - should be solved when actual values are added anyways
const int POT_ADC_LEFT = 150; //CHANGE THIS, TEST IT BEFORE RUNNING CODE, PUT AT LEFT EXTREME AND PUT THAT ANGLE FROM POTENTIOMETER IN HERE/////////////////////
const int POT_ADC_RIGHT = 870; //CHANGE THIS, TEST IT BEFORE RUNNING CODE, PUT AT RIGHT EXTREME AND PUT THAT ANGLE FROM POTENTIOMETER IN HERE/////////////////////

const float GAIN = 3.0; //proportional gain for how far out from PWM value (error)
const int PWM_MIN = 35;
const int PWM_MAX = 200;

const float ANGLE_SMOOTHING = 0.20; //change between 0.000...1 and 1, 1 being no smoothing as heading towards angle. (it will not move if 0)

//Limits in testing
const bool HOLD_LAST_VALUE = true;
const unsigned long SERIAL_TIMEOUT_MS = 1500;

//Pins
//these pins should be right. if the steering is turning the wrong way flip the wires or change the pwm pins here ///////////////////////////
const int POT_PIN = A3;

const int RPWM_PIN = 5;
const int LPWM_PIN = 6;
const int REN_PIN = 7;
const int LEN_PIN = 8;

//Starting state
float targetDeg = 0.0;
float angleDegFiltered = 0.0;
unsigned long lastCmdMs = 0;

//Setup/loop (main code)

void setup() {
  Serial.begin(115200); //maybe we can make this lower, Ahmed had 9500 but I wasn't sure if that was arbitrary
                        // re above: higher baudrate = faster time to react to commands, i don't see a problem with this

 pinMode(RPWM_PIN, OUTPUT); //in testing we generally just kept these hooked up to 5V
 pinMode(LPWM_PIN, OUTPUT); //keeping as may be useful for emergency stop in future
 pinMode(REN_PIN, OUTPUT);
 pinMode(LEN_PIN, OUTPUT);

 driverEnable(true);
 coastStop();

 //initialise the filtered angle to current reading
 int adc = analogRead(POT_PIN);
 angleDegFiltered = mapPotToDeg(adc);

 targetDeg = 0.0;
 lastCmdMs = millis();

 Serial.println("Steering controller ready");
}

void loop() {
  //Update target from serial
  float cmd;
  if (ftSerial.readFloat(cmd)) {
    targetDeg = directionCheck(cmd, -ANGLE_LIMIT_DEG, +ANGLE_LIMIT_DEG);
    lastCmdMs = millis();

    Serial.print("Target: ");
    Serial.println(targetDeg, 2);

  }

  //If it times out, choose behaviour (timeout behaviour is not receiving input)
  if (!HOLD_LAST_VALUE) {
    if (millis() - lastCmdMs > SERIAL_TIMEOUT_MS) {
    targetDeg = 0.0;
  }}

  //Read current angle from pot and filter it
  int adc = analogRead(POT_PIN);
  float angleNow = mapPotToDeg(adc);
  angleDegFiltered = (1.0 - ANGLE_SMOOTHING) * angleDegFiltered + ANGLE_SMOOTHING * angleNow;

  //compute the error (distance needed to travel)
  float err = targetDeg - angleDegFiltered;

  //control
  if (abs(err) <= STOP_BAND_DEG) {
    coastStop();
  } else {
    //proportional speed
    float pwmFloat = GAIN * abs(err);
    int pwm = (int)pwmFloat;

    pwm = constrain(pwm, 0, PWM_MAX);
    if (pwm < PWM_MIN) pwm = PWM_MIN;

    int pwmSigned = (err > 0) ? pwm : -pwm; //going left or right? neg or positive change check
    drive(pwmSigned);
  }

  static unsigned long lastPrint = 0;
  if (millis() - lastPrint > 50) {
    lastPrint = millis();
    Serial.print("angle= ");
    Serial.print(angleDegFiltered, 2);
    Serial.print(" target= ");
    Serial.print(targetDeg, 2);
    Serial.print(" error= ");
    Serial.println(err, 2);
  }


}



//Other functions

float directionCheck (float x, float a, float b) {
  if (x < a) return a;
  if (x > b) return b;
  return x;
}

float mapPotToDeg (int adc) {
  adc = constrain(adc, min(POT_ADC_LEFT, POT_ADC_RIGHT), max(POT_ADC_LEFT, POT_ADC_RIGHT)); //limits input angles to valid/safe range

  float t = (float)(adc - POT_ADC_LEFT) / (float)(POT_ADC_RIGHT - POT_ADC_LEFT); //make sure left < right when calibrating (to make sure this is positive)
  float deg = ((-ANGLE_LIMIT_DEG) + t * (2.0 * ANGLE_LIMIT_DEG));

  return deg;
}

void driverEnable(bool en) {
  digitalWrite(REN_PIN, en ? HIGH : LOW);
  digitalWrite(LEN_PIN, en ? HIGH : LOW);
}

void coastStop() {
  //idk if this is really necessary, I noticed the wheel continued moving a bit when there was power but no signal going to it... (19/02/2026)
  //also being used as the general stop moving command
  analogWrite(RPWM_PIN, 0);
  analogWrite(LPWM_PIN, 0);
}

void drive (int pwmSigned) {
  // > 0 steer right, <0 steer left
  int pwm = abs(pwmSigned);
  pwm = constrain(pwm, 0, 255);

  if (pwmSigned > 0) {
    analogWrite(RPWM_PIN, pwm);
    analogWrite(LPWM_PIN, 0);
  } else if (pwmSigned < 0) {
    analogWrite(RPWM_PIN, 0);
    analogWrite(LPWM_PIN, pwm);
  } else {
    coastStop();
  }
}

