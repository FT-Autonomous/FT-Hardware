int pinA = 10; // 2 Pins
int pinB = 11;

void setup() {
  Serial.begin(9600);
  Serial.println("----------------------");


  pinMode(pinA, OUTPUT);    //2 analog output pins
  pinMode(pinB, OUTPUT);
}

int wait = 2000;  // delay for demo purposes

void loop() {

  extend();

  delay(wait);

  retract();

  delay(wait);

  //Serial.println("----------------------"); //1 full cycle complete indicator
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

/* FAQ NOTES FOR CONTROL: YES YOU!

  naturally: PWM for motor control runs from 0 - 255
  
  1:There is no increasing or decreasing the speed at which the actuation happens
  
  2: 0 is complete stop, anything less than 26(PWM) will cause the motor to try and spin but to no avail.

  3: anything above 26PWM will behave, measurably, no different from PWM 255
    Thus: given the lack of granualar control I HIGHLY recommend you operate in absolutes
          send either 0 or 255. 
  -
  
  4: I anticapate that if the actuator is in a prolonged state of [ 0 < PWM >= 26 ], it will cause undue stress that could lead to malfunction
    hence again refrain from sending singals in that range. 0 is fine ofc but the next lowest signal I would allow is prolly 50. for safety.
  
*/
