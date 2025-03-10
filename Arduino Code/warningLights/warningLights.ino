// as = autonomous system

bool asOff, asReady, asDriving, asFinished, asEmergency, manualD;
int yellow, blue;



void setup() {
  ASSI_Setup();
}

void loop() {
  ASSI();
}


void ASSI_Setup() {
  //initalise default/starting states
  asOff = true;
  asReady = false;
  asDriving = false;
  asFinished = false;
  asEmergency = false;
  manualD = false;

  //intilise pins

  yellow = 5;
  blue = 6;
  pinMode(yellow, OUTPUT);
  pinMode(blue, OUTPUT);
}
void ASSI() {
  if (asOff) {
    digitalWrite(yellow, LOW);
    digitalWrite(blue, LOW);
  }

  if (asReady) {
    digitalWrite(yellow, HIGH);
    digitalWrite(blue, LOW);
  }

  if (asDriving) {
    blink(yellow);
    digitalWrite(blue, LOW);
  }

  if (asFinished) {
    digitalWrite(yellow, LOW);
    digitalWrite(blue, HIGH);
  }

  if (asEmergency) {
    blink(blue);
    digitalWrite(yellow, LOW);
  }

  if (manualD) {
    digitalWrite(yellow, HIGH);
    digitalWrite(blue, HIGH);
  }
}

void blink(int Pin) {
  digitalWrite(Pin, HIGH);
  delay(500);
  digitalWrite(Pin, LOW);
  delay(500);
}
