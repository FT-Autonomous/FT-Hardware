//23/06: Test ASSI
// sources: https://newbiely.com/tutorials/arduino-nano-iot/arduino-nano-33-iot-button-led
// double check the pin


int redPin = D3;
int greenPin = D5;
int bluePin = D6;

int redPin2 = D9;
int greenPin2 = D10;
int bluePin2 = D11;

int btn1 = D2;
int btn2 = D4;

void setup() {
  pinMode(redPin, OUTPUT);
  pinMode(greenPin, OUTPUT);
  pinMode(bluePin, OUTPUT);

  pinMode(redPin2, OUTPUT);
  pinMode(greenPin2, OUTPUT);
  pinMode(bluePin2, OUTPUT);

  pinMode(btn1, INPUT_PULLUP);
  pinMode(btn2, INPUT_PULLUP);
}

void loop() {


  // Btn 1: Flashing state
  if (digitalRead(btn1) == LOW) {
    setColor1(255, 0, 0); // flashing (idk what colour should it be)
    delay(300);
    setColor1(0, 0, 0);
    delay(300);
  } else {
    setColor1(0, 0, 0);
  }

  // Btn 2: solid colour 
  if (digitalRead(btn2) == LOW) {
    setColor2(0, 0, 255);   // LED 2 blue
  } else {
    setColor2(0, 0, 0);     // LED 2 off
  }
}

void setColor1(int redValue, int greenValue, int blueValue) {
  analogWrite(redPin, redValue);
  analogWrite(greenPin, greenValue);
  analogWrite(bluePin, blueValue);
}

void setColor2(int redValue, int greenValue, int blueValue) {
  analogWrite(redPin2, redValue);
  analogWrite(greenPin2, greenValue);
  analogWrite(bluePin2, blueValue);
}