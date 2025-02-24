
int mode = 7;
int prevMode = mode;
const int cyclePin = 2;
const int selectPin = 3;
bool selected = false;
bool interrupt = false;

void setup() {
  Serial.begin(9600);

  pinMode(7, OUTPUT);
  pinMode(8, OUTPUT);
  pinMode(9, OUTPUT);
  pinMode(10, OUTPUT);
  pinMode(11, OUTPUT);
  pinMode(12, OUTPUT);
  pinMode(13, OUTPUT);

  pinMode(cyclePin, INPUT);
  pinMode(selectPin, INPUT);

  attachInterrupt(digitalPinToInterrupt(cyclePin), cycleButton, RISING);  //rightEncoderInterrupt will run when the pin CHANGES VALUE
  attachInterrupt(digitalPinToInterrupt(selectPin), selectButton, RISING);
}

void cycleButton() {
  while (!interrupt) {
    mode++;
    interrupt = true;
  }
}  // code that cycles mode when appropriate button pressed

void selectButton() {
  selected = true;
}

void loop() {
  Serial.println(mode);

  if (!selected) {  // code that blinks not selected LED
    blink(mode);
  } else {
    digitalWrite(mode, HIGH);
  }

  if (mode > 13) {
    mode = 7;
  }

  if (prevMode > 13) {
    prevMode = 7;
    interrupt = false;
  }

  if (mode != prevMode) {
    interrupt = false;
    prevMode = mode;
  }
}

void blink(int Pin) {
  digitalWrite(Pin, HIGH);
  delay(500);
  digitalWrite(Pin, LOW);
  delay(500);
}