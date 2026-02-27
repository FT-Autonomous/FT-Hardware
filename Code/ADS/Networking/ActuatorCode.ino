#include <WiFiS3.h>

char ssid[] = "Test";
char pass[] = "wordpass";

WiFiServer R4(5200);  // establish an instance of class WiFiServer called R4

bool paused = true;
bool cycleTest = false;


int pinA = 10;
int pinB = 11;

const int hallPinR = 3;
volatile long countR = 0;
int count = countR;
void rightCounterInterrupt() {
  countR++;
}

void setup() {
  // put your setup code here, to run once:
  Serial.begin(9600);
  Serial.println("----------------------");
  pinMode(pinA, OUTPUT);
  pinMode(pinB, OUTPUT);

  pinMode(hallPinR, INPUT_PULLUP);
  attachInterrupt(digitalPinToInterrupt(hallPinR), rightCounterInterrupt, CHANGE);  //rightEncoderInterrupt will run when the pin CHANGES VALUE
  wifiSetup();
}

int st = 2000; // delay
int aVal = 100;
int bVal = 100;

void loop() {
  // put your main code here, to run repeatedly:
  wifiLoop();

  if(!paused){ 
    a(0);
    b(0);
    Serial.println("both low");

    hallUpdate();
    delay(st);

    a(aVal);
    Serial.println("A high: Extend");

    hallUpdate();
    delay(st);

    a(0);
    Serial.println("both low");

    hallUpdate();
    delay(st);
    
    b(bVal);
    Serial.println("b high: Retract");

    hallUpdate();
    delay(st);

    Serial.println("----------------------");
  }
}

void a(int state){
  state = floor(255*state/10);
  analogWrite(pinA, state);
}//set A to state percent power

void b(int state){
  state = floor(255*state/10);
  analogWrite(pinB, state);
}

void hallUpdate(){
  if(count != countR){
    Serial.println(countR);
    count = countR;
    sendUpdate();
  }
}


