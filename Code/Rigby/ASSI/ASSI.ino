// ASSI for new 2x 3-channel LED hardware
// LED 2 channels are used for ASSI output.

extern bool selected;

bool asOff, asReady, asDriving, asFinished, asEmergency, manualD;

bool notReady;  // this means timer elapsed without entering go.

bool EBS, done, moving, go;  // Serial variables: Emergency Brake System, mission done and currently moving

double initialT;

// New hardware: LED 2 is the 3-channel ASSI LED
const int assiPinA = 6;
const int assiPinB = 5;
const int assiPinC = 3;

//run this function in setup to ready everything needed for the ASSI function
void ASSI_Setup() {
  //initalise default/starting states
  asOff = false;
  asReady = false;
  asDriving = false;
  asFinished = false;
  asEmergency = false;
  manualD = false;
  notReady = false;

  //intilise pins
  pinMode(assiPinA, OUTPUT);
  pinMode(assiPinB, OUTPUT);
  pinMode(assiPinC, OUTPUT);

  ASSI_LED_Off();
}

void ASSI_LED_Off() {
  analogWrite(assiPinA, 0);
  analogWrite(assiPinB, 0);
  analogWrite(assiPinC, 0);
}

void ASSI_LED_Set(bool A, bool B, bool C) {
  analogWrite(assiPinA, A ? 255 : 0);
  analogWrite(assiPinB, B ? 255 : 0);
  analogWrite(assiPinC, C ? 255 : 0);
}

// Set the ASSI LEDs according to global booleans
void ASSI_LED() {
  if (asOff) {
    ASSI_LED_Off();
  }

  if (asReady) {
    ASSI_LED_Set(true, true, false);
  }

  if (asDriving) {
    blink2(assiPinA);
    analogWrite(assiPinB, 255);
    analogWrite(assiPinC, 0);
  }

  if (asFinished) {
    ASSI_LED_Set(false, false, true);
  }

  if (asEmergency) {
    blink2(assiPinC);
    analogWrite(assiPinA, 0);
    analogWrite(assiPinB, 0);
  }

  if (manualD) {
    ASSI_LED_Set(true, true, true);
  }
}

void ASSI() {
  double timer;  //current T for asReady Timer
  asOff = false;

  if (asEmergency == false) {

    if (EBS) {            //if EBS is engaged
      asDriving = false;  //couldnt possibly be driving if brake is pulled
      asReady = false;
      if (!moving && done) {
        asFinished = true;
      } else {
        asEmergency = true;
      }
    } else {  // if EBS isnt engaged check if mission has been set

      if (selected && !notReady) {

        if (!asReady) {
          initialT = (millis() / 1000.0);  //start the timer for asReady
        }

        asReady = true;

      } else {
        asReady = false;
        asOff = true;
      }
    }

    if (asReady && !asDriving) {
      timer = (millis() / 1000.0) - initialT;  //update timer

      if ((timer >= 5) && (timer < 30) && go) {
        asReady = false;
        asDriving = true;
      }

      if (timer > 30) {
        asEmergency = true;
        asReady = false;
      }
    }
  }
  sendMode();
  ASSI_LED();  //set LEDs based on booleans
}

void reportAS(){
  //TODO: send the AS status over serial after the state machine has decided it.
}
