// RGB LED pin colour mapper test
// Hold Button 1 to test LED 1
// Hold Button 2 to test LED 2
// It will flash each pin/channel one at a time.

const int led1_pinA = 6;
const int led1_pinB = 5; // assumed dead, continuity testing passed
const int led1_pinC = 3;

const int led2_pinA = 9;
const int led2_pinB = 10;
const int led2_pinC = 11;

const int btn1 = 2;
const int btn2 = 4;

void setup() {
  pinMode(led1_pinA, OUTPUT);
  pinMode(led1_pinB, OUTPUT);
  pinMode(led1_pinC, OUTPUT);

  pinMode(led2_pinA, OUTPUT);
  pinMode(led2_pinB, OUTPUT);
  pinMode(led2_pinC, OUTPUT);

  pinMode(btn1, INPUT_PULLUP);
  pinMode(btn2, INPUT_PULLUP);

  allOff();
}

void loop() {
  if (digitalRead(btn1) == LOW) {
    testLED1();
  } 
  else if (digitalRead(btn2) == LOW) {
    testLED2();
  } 
  else {
    allOff();
  }
}

void testLED1() {
  // LED 1, pin 3
  flashSingle(led1_pinA);
  delay(700);

  // LED 1, pin 5
  flashSingle(led1_pinB);
  delay(700);

  // LED 1, pin 6
  flashSingle(led1_pinC);
  delay(1200);
}

void testLED2() {
  // LED 2, pin 9
  flashSingle(led2_pinA);
  delay(700);

  // LED 2, pin 10
  flashSingle(led2_pinB);
  delay(700);

  // LED 2, pin 11
  flashSingle(led2_pinC);
  delay(1200);
}

void flashSingle(int pinToFlash) {
  allOff();

  analogWrite(pinToFlash, 255);
  delay(300);

  allOff();
  delay(300);

  analogWrite(pinToFlash, 255);
  delay(300);

  allOff();
}

void allOff() {
  analogWrite(led1_pinA, 0);
  analogWrite(led1_pinB, 0);
  analogWrite(led1_pinC, 0);

  analogWrite(led2_pinA, 0);
  analogWrite(led2_pinB, 0);
  analogWrite(led2_pinC, 0);
}