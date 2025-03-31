int received;

void checkSerial() {
  if (Serial.available() > 0) {
    received = Serial.read();
    Serial.println(received);
    decode(received);
  }
}

void decode(int val) {
  double remainder;

  for (int i = 0; i < 3; i++) {
    
    remainder = val / 2;
    val = floor(remainder);
    remainder = remainder - val; 

    if (remainder != 0) {
      if (i == 0) {
        go = true;
      } else if (i == 1) {
        moving = true;
      } else if (i == 2) {
        done = true;
      } else {
        EBS = true;
      }
    }
  }
}
