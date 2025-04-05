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

  double remainder;
  //we use a pretty standard conversion method I found on khan academy
  // only differencs is that instead of storing the ones and zeors found we assign the booleans accordingly

  for (int i = 0; i < 4; i++) {

    remainder = val / 2;          //remainder is half of the value, with a decimal place of 0.5 or 0
    val = floor(remainder);       //set val as truncated remainder (exmpl: 6.5 becomes 6, 6.0 stays 6)
    remainder = remainder - val;  //remainder is now either 0.5 or 0. from here we WOULD assign the relevant array position with 0 or 1

    //setting a bool = to a double will return false if the double is 0 and true otherwise
    if (i == 0) {
      go = remainder;
    } else if (i == 1) {
      moving = remainder;
    } else if (i == 2) {
      done = remainder;
    } else {
      EBS = remainder;
    }
  }
}  //set serial booleans according to integer recieved over serial
