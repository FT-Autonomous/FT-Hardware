//Actuator test programme

import processing.net.*;//library for server stuff

Client R4 = new Client(this, "172.20.10.2", 5200);  //establish the buggy as a place to send and receive things

char data;

int count = 77;

void setup() {
  size(800, 500);
  background(200);
  
  analogNumberDisplay(10*buttonW, sliderY, count);
}

void draw() {
  showUI();
  
  analogNumberDisplay(10*buttonW, sliderY, count);
  
  data = R4.readChar();  //read what the buggy sent and store it in "data"
  if(data == 'c'){
    count = getInteger();
  }
}

void mousePressed() {
  clickUI();
}


void keyPressed() {
  typeUI();
}

int getInteger() {
  //if we try to read in too early it'll save as -1, so keep checking until we get the actual signal
  int NUM = -1;
  while (NUM == -1) { //wait till count != -1
    NUM = R4.read();
  }
  return NUM;
}
