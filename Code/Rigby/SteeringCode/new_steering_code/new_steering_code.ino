int pot = A3; // Analog read

void setup() {
  // put your setup code here, to run once:
  Serial.begin(9600);

}

float angle_target = 0.0;
float angle_current = 0.0;
float error = 0;  //error is dynamic
float kp = 1; 

int pwm = 0;
int correction = 0;
float angle_max = 39; // Maxium angle (you can't go any further)
float angle_input = 0.0;

void loop() {
  // put your main code here, to run repeatedly:
   if(Serial.available() > 0)
  {
    angle_input = Serial.parseFloat();
    if(angle_input != 0.0)
   {  
     angle_target = constrain(angle_input, -angle_max, angle_max); // this allows to update the input using the serial mon.
   }
  }

   unsigned long currentTime = millis();
   unsigned long lastTime = 0;
  
  // Calculate RPM 
  if (currentTime - lastTime >= 500) {
   int value = analogRead(pot);
   angle_current = map(value, 0, 1023, -angle_max, angle_max); // Map out the angle to (-39 and 39 degrees the max the pot will go)
   angle_current = round(angle_current * 10) / 10.0;
}

int pwm_target = ((angle_target / angle_max) * 255); // feed the PID a value between 0-255, it converts the target velocity from m/s to target pwn val.
int pwm_current = ((angle_current / angle_max) * 255); // PWM 
error = pwm_target - pwm_current;
correction = (error * kp);
pwm = pwm_current + correction;
pwm = constrain(pwm, 0, 255);

 Serial.print(angle_target);
   Serial.print("Goal: ");


   Serial.print(angle_current);
   Serial.print("Current: ");

   Serial.print(correction);
    Serial.print("err: ");


    Serial.print(pwm);


   //  Serial.print(" ");

   // Serial.print(hold);

   // Serial.print(" ");

    Serial.print(angle_target);
    Serial.print("New_t ");


    Serial.println(angle_current);
    Serial.print("curr_t ");




    //pulseCount = 0; // Reset pulse count
    lastTime = currentTime; // Update time
  }
