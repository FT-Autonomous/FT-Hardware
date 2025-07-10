
bool firstBlink = true;
double prevT, currT;

bool firstBlink2 = true;
double prevT2, currT2;

//************************************

const int ledPinMax = 12;
const int ledPinMin = 6;  //pin range of LED array mission select

int mode = ledPinMin;  //test
int prevMode = mode;

const int cyclePin = 2;
const int selectPin = 3;

bool selected = false;
bool interrupted = false;

//************************************

void setup() {
  Serial.begin(9600);

  ASSI_Setup();

  for (int i = ledPinMin; i < ledPinMax; i++) {
    pinMode(i, OUTPUT);
  }  //set pinmode for missionSelect LED range

  pinMode(cyclePin, INPUT);
  pinMode(selectPin, INPUT);

  attachInterrupt(digitalPinToInterrupt(cyclePin), cycleButton, RISING);
  attachInterrupt(digitalPinToInterrupt(selectPin), selectButton, RISING);
}

void cycleButton() {
  if (!interrupted && !selected) {  //this was set as while for some reason, I see no reason for this and dont remember it having caused an issue before so now using IF instead
    mode++;
    interrupted = true;
  }
}  // code that cycles mode when appropriate button pressed

void selectButton() {
  selected = true;
}

void loop() {

  if (!selected) {  // if no mode has been selected yet
    blink(mode);    //blink the LED corresponding to the current mode being conidered
    Serial.println(mode);
  } else {
    digitalWrite(mode, HIGH);  //display chosen mode
  }

  if (mode > ledPinMax) {
    mode = ledPinMin;
  }  //loop back around if we have cycled out of bounds

  if (prevMode > ledPinMax) {
    prevMode = ledPinMin;
    interrupted = false;
  }

  if (mode != prevMode) {
    digitalWrite(prevMode, LOW);
    prevMode = mode;
    firstBlink = true;
    delay(250);
    interrupted = false;
  }
  if (selected)
    checkSerial();
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