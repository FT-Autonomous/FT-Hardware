char received;
int motorL = 5;
int motorR = 6;

int high = 255;
int low = 0;

void setup() {
  Serial.begin(9600);
  pinMode(motorL, OUTPUT);
  pinMode(motorR, OUTPUT);

  steer('S');  //stopped by default
}

void loop() {
  checkSerial();
  steer(received);
}

void checkSerial() {
  if (Serial.available() > 0) {
    received = Serial.read();
    Serial.print(received);
    Serial.println("---");
  }
}

void steer(char temp) {

  if (temp == 'L') {  //Move left
    analogWrite(motorL, high);
    analogWrite(motorR, low);
  } else if (temp == 'R') {  //Move right
    analogWrite(motorR, high);
    analogWrite(motorL, low);
  } else if (temp == 'S') {  //Stop command
    analogWrite(motorL, low);
    analogWrite(motorR, low);
  }
}