int pot = A3; // Analog read

void setup() {
  // put your setup code here, to run once:
  Serial.begin(9600);

}

float ang_target = 0.0;
float ang_current = 0.0;
float error = 0;  //error is dynamic
float kp = 1;

int pwm = 0;
int correction = 0;
float v_max = 0.8; // the max speed m/s
float v_input = 0.0;

void loop() {
  // put your main code here, to run repeatedly:
   if(Serial.available() > 0)
  {
    v_input = Serial.parseFloat();
    if(v_input != 0.0)
   {  
     v_target = v_input; // this allows to update the input using the serial mon.
   }
  }

   unsigned long currentTime = millis();
  
  // Calculate RPM 
  if (currentTime - lastTime >= 500) {
    float rpm = (pulseCount / (float)pulsesPerRevolution) * 60.0; 
   
   float Ang_velocity = rpm * M_PI / 60;
   float linear_velocity = Ang_velocity * 0.3; // LV = anugular velocity * radius (diameter is 6mm so radius is 0.3mm)
   v_current = round(linear_velocity * 10) / 10.0;

    

}

int pwm_target = ((v_target / v_max) * 255); // feed the PID a value between 0-255, it converts the target velocity from m/s to target pwn val.
int pwm_current = ((v_current / v_max) * 255); // PWM 
error = pwm_target - pwm_current;
correction = (error * kp);
pwm = pwm_current + correction;
pwm = constrain(pwm, 0, 255);
