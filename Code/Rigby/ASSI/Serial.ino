int received;

/*
  TODO
  Need to figure out how this information is going to arrive (sentinal controlled strings or Hex or shannon fano style codes )
    so that a processing scheme can be made accordingly (this should be simple since atm we can enter chars on the serial monitor to be stored in "recieved")

    need to figure out how its meant to look when working then test and fix the "decode()" function
    good postion, need to decide on direction to proceed
*/

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
      //Serial.println(received);
      //Serial.println("---");
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

