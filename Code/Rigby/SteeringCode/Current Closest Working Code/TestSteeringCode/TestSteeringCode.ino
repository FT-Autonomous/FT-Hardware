char received = 's';

int motorL = 5;
int motorR = 6;

int high = 255;
int low = 0;
/*
//timing control to avoid flooding arduino
unsigned long msSinceCmd = 0;
unsigned long msTimeout = 200;
*/

void setup() {
  Serial.begin(9600); //we could speed up if lagging? - 115200
  pinMode(motorL, OUTPUT);
  pinMode(motorR, OUTPUT);

  steer('s');  //stopped by default
  //msSinceCmd = millis();
}

void loop() {
  if (checkSerial()) {
    steer(received);
  }
}


bool checkSerial() {
  bool got = false;

  while(Serial.available() > 0) {
    char c = Serial.read();

    if (c == '\n' || c == '\r' || c == ' ') continue;

    received = c;
    //msSinceCmd = millis();
    got = true;
  }

  if (got) {
    Serial.print("Got: ");
    Serial.print(received);
    Serial.print(" ASCII=");
    Serial.println((int)received);
  }

  return got;
}


void steer(char temp) {
  
  if (temp == 'L') {  //Move left
    analogWrite(motorL, high);
    analogWrite(motorR, low);
  } else if (temp == 'R') {  //Move right
    analogWrite(motorR, high);
    analogWrite(motorL, low);
  } else if (temp == 's') {  //Stop if not explicitly left or right
    analogWrite(motorL, low);
    analogWrite(motorR, low);
  }
}