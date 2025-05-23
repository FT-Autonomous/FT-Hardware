//#include "Arduino.h"

//ArduPID C1;

#define ENCODER_A 2 // Channel A pin
#define ENCODER_B 3 // Channel B pin

#define MOTOR_F 10 // For motor stuff
#define MOTOR_B 11




//double setpoint = 0.1
//double input;
//double output;



volatile int pulseCount = 0; // Number of pulses (Volatile int to load var from RAM and not overload the compiler as the encoder speed and direction change is based on the command driving it)
unsigned long lastTime = 0; // Time for RPM calculation
const int pulsesPerRevolution = 600; // Based on the encoder PPR

float v_target = 0.0;
float v_current = 0.0;
float error = 0;  //error is dynamic
float kp = 1; // kp val @ 0.5 works solo for now
float ki = 0;
float kd = 0;

int pwm = 0;
int correction = 0;
float v_max = 0.8; // the max speed m/s
float v_input = 0.0;


bool hold = false;



void encoderISR() {
  // Increment pulse count on every rising edge of Channel A
  if (digitalRead(ENCODER_A) == HIGH) {
    if (digitalRead(ENCODER_B) == LOW) {
      pulseCount++; // GO CLOCKWISE
    } else {
      pulseCount--; // GO ANTICLOCKWISE (Could change since the wheel placment would mean the shaft goes ANTICLOCKWISE)
    }
  }
}

void setup() {
  Serial.begin(9600);
  Serial.setTimeout(1000);
  
  // Set up encoder pins
  pinMode(ENCODER_A, INPUT_PULLUP);
  pinMode(ENCODER_B, INPUT_PULLUP); //encoder pins

  pinMode(MOTOR_F,OUTPUT);
  pinMode(MOTOR_B,OUTPUT);  // motor stuff

  //c1.begin(&input, &output, &setpoint, p, i ,d);

 


  
  // Attach interrupt for encoder
  attachInterrupt(digitalPinToInterrupt(ENCODER_A), encoderISR, RISING);

  lastTime = millis(); // Initialize time
}

void loop() {

  if(Serial.available() > 0)
  {
    v_target = Serial.read(); 
  }

  unsigned long currentTime = millis();
  
  // Calculate RPM 
  if (currentTime - lastTime >= 500) {
    float rpm = (pulseCount / (float)pulsesPerRevolution) * 60.0; 
   
   float Ang_velocity = rpm * M_PI / 60;
   float linear_velocity = Ang_velocity * 0.3; // LV = anugular velocity * radius (diameter is 6mm so radius is 0.3mm)
   v_current = round(linear_velocity * 10) / 10.0;


    //Serial.print("RPM: ");
    //Serial.println(rpm);

    //.print(" AV: ");
    //Serial.println(Ang_velocity);

   //Serial.print("LV: ");
    //Serial.print(v_current);

    //Serial.println(v_target);

    int pwm_target = ((v_target / v_max) * 255); // feed the PID a value between 0-255, it converts the target velocity from m/s to target pwn val.
    int pwm_current = ((v_current / v_max) * 255); // PWM 
    error = pwm_target - pwm_current;
    correction = (error * kp);
    pwm = pwm_current + correction;
    pwm = constrain(pwm, 0, 255);




   Serial.print(pwm_target);
   Serial.print(" ");


   Serial.print(pwm_current);
   Serial.print(" ");

   Serial.print(correction);
    Serial.print(" ");


    Serial.print(pwm);


    Serial.print(" ");

    Serial.print(hold);

    Serial.print(" ");

    Serial.print(v_target);
    Serial.print(" ");


    Serial.println(v_current);


    






    pulseCount = 0; // Reset pulse count
    lastTime = currentTime; // Update time
  }

  analogWrite(MOTOR_F, pwm);
  analogWrite(MOTOR_B, 0); // Drive forward.











}

// to do:
// write a PID 
// #1: current / max speed * 255


// serial write





// 0.8 m/s is the max speed @ 8V and 1.6 Amp
