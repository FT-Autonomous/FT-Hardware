
bool asOff, asReady, asDriving, asFinished, asEmergency, manualD;

bool notReady;  // this means timer elapsed without entering go.

bool EBS, done, moving, go;  //Serial variables: Emergency Break system, Mission done and currently moving

int yellow, blue; //yellow and blue being the colors of the corresponding LEDs
double initialT;

//run this function in setup to ready everything needed for the ASSI functionw
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

  yellow = 4;
  blue = 5;
  pinMode(yellow, OUTPUT);
  pinMode(blue, OUTPUT);
}

// Set the ASSI LEDs according to global booleans
void ASSI_LED() {
  if (asOff) {
    digitalWrite(yellow, LOW);
    digitalWrite(blue, LOW);
  }

  if (asReady) {
    digitalWrite(yellow, HIGH);
    digitalWrite(blue, LOW);
  }

  if (asDriving) {
    blink2(yellow);
    digitalWrite(blue, LOW);
  }

  if (asFinished) {
    digitalWrite(yellow, LOW);
    digitalWrite(blue, HIGH);
  }

  if (asEmergency) {
    blink2(blue);
    digitalWrite(yellow, LOW);
  }

  if (manualD) {
    digitalWrite(yellow, HIGH);
    digitalWrite(blue, HIGH);
  }
}

void ASSI() {
  double timer;  //current T for asReady Timer
  asOff = false;

  if (asEmergency == false) {

    if (EBS) {            //if EBS is engaged
      asDriving = false;  //couldnt possibly be driving if break is pulled
      asReady = false;
      if (!moving && done) {
        asFinished = true;
      } else {
        asEmergency = true;
      }
    } else {  // if EBS isnt engaged check if mission has been set

      if (selected && !notReady) {

        if (!asReady) {
          initialT = (millis() / 1000);  //start the timer for asReady
        }

        asReady = true;

      } else {
        asReady = false;
        asOff = true;
      }
    }

    if (asReady && !asDriving) {
      timer = (millis() / 1000) - initialT;  //update timer

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