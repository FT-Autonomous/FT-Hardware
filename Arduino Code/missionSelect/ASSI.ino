
void ASSI_Setup() {
  //initalise default/starting states
  asOff = false;
  asReady = false;
  asDriving = true;
  asFinished = false;
  asEmergency = false;
  manualD = false;

  //intilise pins

  yellow = 4;
  blue = 5;
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
