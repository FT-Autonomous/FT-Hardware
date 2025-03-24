
bool firstBlink = true;
double prevT, currT;

bool firstBlink2 = true;
double prevT2, currT2;



//************************************


int LedPinMax = 12;
int LedPinMin = 6;

int mode = LedPinMin;
int prevMode = mode;

const int cyclePin = 2;
const int selectPin = 3;


bool selected = false;
bool interrupt = false;

void setup() {
  Serial.begin(9600);

  ASSI_Setup();

  /*
    pinMode(6, OUTPUT);
    pinMode(7, OUTPUT);
    pinMode(8, OUTPUT);
    pinMode(9, OUTPUT);
    pinMode(10, OUTPUT);
    pinMode(11, OUTPUT);
    pinMode(12, OUTPUT);
  //*/

  for (int i = LedPinMin; i < LedPinMax; i++) {
    pinMode(i, OUTPUT);
  }

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

  if (mode > LedPinMax) {
    mode = LedPinMin;
  }

  if (prevMode > LedPinMax) {
    prevMode = LedPinMin;
    interrupt = false;
  }

  if (mode != prevMode) {
    digitalWrite(prevMode, LOW);
    prevMode = mode;
    firstBlink = true;
    delay(250);
    interrupt = false;
  }

  ASSI();
}
// Cleavon was here
void blink(int Pin) {
  currT = (millis() / 1000);  //get time in integer seconds

  if (firstBlink) {           //if we only just started blinking
    digitalWrite(Pin, HIGH);  //on
    prevT = currT;            //save current time

    firstBlink = false;                //set false
  } else if ((currT - prevT) > 0.5) {  //if firstblink is false and the current time is 2 seconds greater than previous time
    digitalWrite(Pin, LOW);            //set off

    if ((currT - prevT) > 1) {  //delay another 2 seconds before changing states
      firstBlink = true;
    }
  }
}

void blink2(int Pin) {
  currT2 = (millis() / 1000);  //get time in integer seconds

  if (firstBlink2) {          //if we only just started blinking
    digitalWrite(Pin, HIGH);  //on
    prevT2 = currT2;          //save current time

    firstBlink2 = false;                 //set false
  } else if ((currT2 - prevT2) > 0.5) {  //if firstblink is false and the current time is 2 seconds greater than previous time
    digitalWrite(Pin, LOW);              //set off

    if ((currT2 - prevT2) > 1) {  //delay another 2 seconds before changing states
      firstBlink2 = true;
    }
  }
}