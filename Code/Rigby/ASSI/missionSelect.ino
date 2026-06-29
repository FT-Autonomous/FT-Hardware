// Mission select for new 2x 3-channel LED hardware
// LED 1 channels are used to display missions 1-7 as binary combinations.

bool firstBlink = true;
double prevT, currT;

bool firstBlink2 = true;
double prevT2, currT2;

//************************************

const int missionMin = 1;
const int missionMax = 7;

// New hardware: LED 1 is the 3-channel mission-select LED
const int missionPinA = 9;
const int missionPinB = 10;
const int missionPinC = 11;

int mode = missionMin;
int prevMode = mode;

const int cyclePin = 2;
const int selectPin = 4;

bool selected = false;

// Button polling, because pin 4 is not an interrupt pin on Arduino Uno/Nano
bool lastCycleState = HIGH;
bool lastSelectState = HIGH;
unsigned long lastCycleT = 0;
unsigned long lastSelectT = 0;
const unsigned long debounceDelay = 50;

//************************************

void setup() {
  Serial.begin(9600);

  ASSI_Setup();

  pinMode(missionPinA, OUTPUT);
  pinMode(missionPinB, OUTPUT);
  pinMode(missionPinC, OUTPUT);

  pinMode(cyclePin, INPUT_PULLUP);
  pinMode(selectPin, INPUT_PULLUP);

  missionLEDOff();
}

void cycleButton() {
  if (!selected) {
    mode++;

    if (mode > missionMax) {
      mode = missionMin;
    }

    missionLEDOff();
    prevMode = mode;
    firstBlink = true;
  }
}

void selectButton() {
  selected = true;
  setMissionLED(mode);
}

void checkButtons() {
  bool cycleState = digitalRead(cyclePin);
  bool selectState = digitalRead(selectPin);
  unsigned long now = millis();

  if (lastCycleState == HIGH && cycleState == LOW && (now - lastCycleT) > debounceDelay) {
    cycleButton();
    lastCycleT = now;
  }

  if (lastSelectState == HIGH && selectState == LOW && (now - lastSelectT) > debounceDelay) {
    selectButton();
    lastSelectT = now;
  }

  lastCycleState = cycleState;
  lastSelectState = selectState;
}

void loop() {
  checkButtons();

  if (!selected) {
    blink(mode);
  } else {
    setMissionLED(mode);
    checkSerial();
  }

  ASSI();
}

void setMissionLED(int mission) {
  // Mission 1-7 are the 7 non-zero binary combinations of the 3 LED channels.
  analogWrite(missionPinA, (mission & 0b001) ? 255 : 0);
  analogWrite(missionPinB, (mission & 0b010) ? 255 : 0);
  analogWrite(missionPinC, (mission & 0b100) ? 255 : 0);
}

void missionLEDOff() {
  analogWrite(missionPinA, 0);
  analogWrite(missionPinB, 0);
  analogWrite(missionPinC, 0);
}

void blink(int mission) {
  currT = (millis() / 1000.0);

  if (firstBlink) {
    setMissionLED(mission);
    prevT = currT;
    firstBlink = false;
  } else if ((currT - prevT) > 0.5) {
    missionLEDOff();

    if ((currT - prevT) > 1) {
      firstBlink = true;
    }
  }
}

void blink2(int Pin) {
  currT2 = (millis() / 1000.0);

  if (firstBlink2) {
    analogWrite(Pin, 255);
    prevT2 = currT2;
    firstBlink2 = false;
  } else if ((currT2 - prevT2) > 0.5) {
    analogWrite(Pin, 0);

    if ((currT2 - prevT2) > 1) {
      firstBlink2 = true;
    }
  }
}
