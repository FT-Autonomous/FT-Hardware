/*
  ahmed has trouble running missionSelect on his linux machine the way I have the code organised across 3 bitesized files
  So I am putting it all together for him
//*/

//Primary global variables

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

// ASSI specific Global variables

bool asOff, asReady, asDriving, asFinished, asEmergency, manualD;

bool notReady;  // this means timer elapsed without entering go.

bool EBS, done, moving, go;  //Serial variables: Emergency Break system, Mission done and currently moving

int yellow, blue; //yellow and blue being the colors of the corresponding LEDs
double initialT;

//************************************

int received; // for serial communication

//************************************

void setup() {
  Serial.begin(9600);

  ASSI_Setup();
  //initialise all the pins needed for the ASSI LEDs and buttons 
  
  for (int i = ledPinMin; i < ledPinMax; i++) {
    pinMode(i, OUTPUT);
  }  //set pinmode for missionSelect LED range

  pinMode(cyclePin, INPUT);
  pinMode(selectPin, INPUT);

  attachInterrupt(digitalPinToInterrupt(cyclePin), cycleButton, RISING);
  attachInterrupt(digitalPinToInterrupt(selectPin), selectButton, RISING);
}

//************************************ Interrupt functions

void cycleButton() {
  if (!interrupted && !selected) {  //this was set as while for some reason, I see no reason for this and dont remember it having caused an issue before so now using IF instead
    mode++;
    interrupted = true;
  }
}  // code that cycles mode when appropriate button pressed

void selectButton() {
  selected = true;
}

//************************************

void loop() {

  if (!selected) {  // if no mode has been selected yet
    blink(mode);    //blink the LED corresponding to the current mode being conidered
    //Serial.println(mode);
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

//************************************ blink functions

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

//************************************ ASSI Specific functions

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
}//run this function in setup to ready everything needed for the ASSI function

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

//************************************ Serial Specific functions

void checkSerial() {
  if (Serial.available() > 0) {
    received = Serial.read();  // Serial.read() retruns a char NOT an int. so storing it this way wont store what was entered but instead the ASCII of what was entered.

    //https://theasciicode.com.ar/ascii-printable-characters/capital-letter-a-uppercase-ascii-code-65.html
    //this link displays what value which chars are stored as

    if (received != 10) {  //10 corresponds to the Enter Key which needs to be ignored
      if (65 <= received && received <= 70) { //if we got something between A and F (HEX INPUT)
        received = received - 55;
      } else
        received = received - 48;  //the ascii for 0 is 48
      Serial.println(received);
      Serial.println("---");
      //sendMode();
      decode(received);
    }
  }
}

void decode(int val) {
  /*
    val is a decimal integer recieved over serial
    The 4bit binary of val represents the desired state for EBS, done, moving & go;
    we need to convert val to binary and then set the booleans to match those ones and zeros
    we assume that the value fits 4 bit binary
  */

  //I cant remember what these are meant to look like for testing but I dont think it's working

  go = val & 0b0001;             // LSB (bit 0)
  moving = (val >> 1) & 0b0001;  // bit 1
  done = (val >> 2) & 0b0001;    // bit 2
  EBS = (val >> 3) & 0b0001;     // MSB (bit 3)

}  //set serial booleans according to integer recieved over serial

void sendMode(){
  //bool asOff, asReady, asDriving, asFinished, asEmergency, manualD;
  if( asOff )
    send("O");
  else if ( asReady )
    send("R");
  else if ( asDriving )
    send("D");
  else if ( asFinished ){
    send( "E" );
  }
}

void send(String temp){
  Serial.println(temp);
}

