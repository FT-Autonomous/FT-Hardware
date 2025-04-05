int received;

void checkSerial() {
  if (Serial.available() > 0) {
    received = Serial.read();
    Serial.println(received);
    decode(received);
  }
}

void decode(int val) {
  /*
    val is a decimal integer recieved over serial
    The 4bit binary of val represents the desired state for EBS, done, moving & go;
    we need to convert val to binary and then set the booleans to match those ones and zeros
    we assume that the value fits 4 bit binary
  */

  go = val & 0b0001;             // LSB (bit 0)
  moving = (val >> 1) & 0b0001;  // bit 1
  done = (val >> 2) & 0b0001;    // bit 2
  EBS = (val >> 3) & 0b0001;     // MSB (bit 3)

}  //set serial booleans according to integer recieved over serial
