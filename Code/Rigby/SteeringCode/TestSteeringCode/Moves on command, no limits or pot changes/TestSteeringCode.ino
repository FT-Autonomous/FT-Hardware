#include <FTSerial.h>

FTSerial ftSerial(Serial);

int motorL = 5;
int motorR = 6;

int high = 255;
int low = 0;

void setup() {
  Serial.begin(9600); //we could speed up if lagging? - 115200
  pinMode(motorL, OUTPUT);
  pinMode(motorR, OUTPUT);

  steer('s');  //stopped by default
}

void loop() {
  String line = ftSerial.readUntilNewline();
  if (line.length() > 0) {
    char received = line.charAt(0);
    Serial.print("Got: ");
    Serial.print(received);
    Serial.print(" ASCII=");
    Serial.println((int)received);
    steer(received);
  }
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