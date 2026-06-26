const int potentiometerPin = A3;
int pinA = 10;  // 2 Pins for actuator control
int pinB = 11;


// Global current position estimate in millimeters
float currentSliderMM = 0.0;
const float sliderTravelMM = 100.0;  // max range of slider

int targetMM = 0;
int margin = 2;  // TODO calibrate tolerance

void setup() {
  Serial.begin(9600);
  //Serial.println("----------------------");


  pinMode(pinA, OUTPUT);  //2 analog output pins
  pinMode(pinB, OUTPUT);
}

void loop() {
  updateTarget();            // update global var targetMM from serial
  int curr = getSliderMM();  // update global var currentSliderMM

  int difference = targetMM - curr;

  if (abs(difference) >= margin) {
    if (difference < 0) {
      extend();  //start extending
    } else {
      retract(); //start retracting
    }// TODO Calibrate direction response

    while (abs(difference) >= margin) {
      difference = targetMM - getSliderMM();
    }//wait till we get to the desired margin

    stop();  // stop actuating at target
  }
}

int updateTarget() { //des
  if (Serial.available() > 0) {
    int newTarget = Serial.parseInt();

    if (newTarget >= 0 && newTarget <= sliderTravelMM) {
      targetMM = newTarget;
    } else {
      targetMM = constrain(newTarget, 0, (int)sliderTravelMM);
    }

    //Serial.print("MM target set to: ");
    Serial.println(targetMM);

    while (Serial.available()) {
      Serial.read();  // clear leftover newline/characters
    }
  }
  return targetMM;
}

int getSliderMM() {
  int rawValue = analogRead(potentiometerPin);  //pwm 0 - 1023 from sensor

  float percentage = rawValue / 1023.0;
  currentSliderMM = percentage * sliderTravelMM;

  /*
    Serial.print("  Approx Position: ");
    Serial.print(currentSliderMM, 2);
    Serial.println(" mm");
  */
  return currentSliderMM;
}

void extend() {
  analogWrite(pinB, 0);
  analogWrite(pinA, 255);
  //Serial.println("A high: Extend");
}

void retract() {
  analogWrite(pinA, 0);
  analogWrite(pinB, 255);
  //Serial.println("b high: Retract");
}

void stop() {
  analogWrite(pinA, 0);
  analogWrite(pinB, 0);
  //Serial.println("both low: Stop");
}
