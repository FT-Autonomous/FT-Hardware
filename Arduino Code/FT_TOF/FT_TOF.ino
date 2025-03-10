#include <math.h>

const int trigPin = 8; 
const int echoPin= 9;    // Ultrasonics Pins

const int(dmotorf) = 10;
const int(dmotorb) = 11;   // Drive Motor forward back
const int(smotor1) = 3;   
const int(smotor2) = 2;     // Steering Motor 1 2

char ch, dholdcmd, sholdcmd, steercmd, drivecmd;
bool dupdate, supdate;
float duration;
float distance;
float distrd;

void setup() {  
	pinMode(trigPin, OUTPUT); // trigger, output
	pinMode(echoPin, INPUT);  // echo, input

  pinMode(dmotorf, OUTPUT); 
  pinMode(dmotorb, OUTPUT);  // 
  pinMode(smotor1, OUTPUT);
  pinMode(smotor2, OUTPUT);
  dupdate = false;
  supdate = false;
  digitalWrite(dmotorf, LOW);
  digitalWrite(dmotorb, LOW); 
  digitalWrite(smotor1, LOW);
  digitalWrite(smotor2, LOW);
	Serial.begin(9600);  
} 

void loop() { 


	digitalWrite(trigPin, LOW);  
	delayMicroseconds(2);  
	digitalWrite(trigPin, HIGH);  
	delayMicroseconds(10);  
	digitalWrite(trigPin, LOW);  
  duration = pulseIn(echoPin, HIGH);
  distance = (duration*.343)/2;
  distrd = (round(distance)/10);
  Serial.print("Distance: ");  
	Serial.println(distrd);


  charget();
  charset();
  cmdset();
  if (distrd >= 16){
    Serial.println("max lock left");
  }
  if (distrd <= 9.9){
    Serial.println("max lock right");
  }
	delay(100);
}

void charget() {
 if (Serial.available() > 0) {
 ch = Serial.read();                /// reads charachter set by laptop controller
 }
}

void charset(){
  if((ch == 'w' || ch == 's' || ch == 'n') && (dupdate == false)){
    drivecmd = ch; 
    dupdate = true;


  }
  if((ch == 'a' || ch == 'd' || ch == 'm' || ch == 'b') && (supdate == false)){      // decipher command to steering or drive
    steercmd = ch; 
    supdate = true;
  }
}
void cmdset(){
  if(drivecmd == 'w' && dupdate == ltrue){
    analogWrite(dmotorf, 50);
    analogWrite(dmotorb, 0);
    Serial.println("forwards");     // drive forwards if w
    dupdate = false;
  }
  if(drivecmd == 's' && dupdate == true){
    analogWrite(dmotorf, 0);   // drive backwards if s
    analogWrite(dmotorb, 50);
    Serial.println("reverse");
    dupdate = false;
  }
  if(drivecmd == 'n' && dupdate == true){
    analogWrite(dmotorf, 0);   // no power to drive motor n
    analogWrite(dmotorb, 0);
    Serial.println("neutral");
    dupdate = false;
  }
  if(steercmd == 'a' && supdate == true){ 
     if(distrd >= 15){
       digitalWrite(smotor1, LOW);   // max lock left
       digitalWrite(smotor2, LOW);  
       supdate = false;
  }
    else if (supdate == true){
      digitalWrite(smotor1, HIGH);
      digitalWrite(smotor2, LOW);    // steer right d
      Serial.println("left");
      supdate = false;
    }
  }

  if(steercmd == 'd'){
     if(distrd <= 9.9){
       digitalWrite(smotor1, LOW);   // max lock right
       digitalWrite(smotor2, LOW);
       supdate = false;
     }
     else if (supdate == true){
      digitalWrite(smotor1, LOW);
      digitalWrite(smotor2, HIGH);    // steer right d
      Serial.println("right");
      supdate = false;
    }
  }
  if(steercmd == 'm' && supdate == true){
    digitalWrite(smotor1, LOW);
    digitalWrite(smotor2, LOW);     // hold current steering angle m
    Serial.println("hold steering angle");
    supdate = false;
  }
}
//   if(steercmd == 'b' && supdate == true){
//   Serial.println("centering");
//     if(distrd > 12){}
//       digitalWrite(smotor1, LOW);
//       digitalWrite(smotor2, LOW);     //steer right
//       supdate = false;
//     }
//     if(distrd < 12){
//       digitalWrite(smotor1, LOW);
//       digitalWrite(smotor2, LOW);     //steer left
//       supdate = false;
//     }
//     else{
//       digitalWrite(smotor1, LOW);
//       digitalWrite(smotor2, LOW);     // center and hold
//       supdate = false;
//     } 
// }

