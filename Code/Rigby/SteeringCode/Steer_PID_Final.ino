int pot = A3; // Analog read
int MOTOR_B = 5; // DC Motor that contorls the steering module
int MOTOR_F = 6;

// Resrouces: https://forum.arduino.cc/t/dc-motor-pid-control-with-arduino-motor-shield-and-encoder/217946/3 & https://forum.arduino.cc/t/how-do-i-control-the-position-of-my-dc-motor-by-using-a-pot/251307/20
// Main issue(s): so when you match the current and present angle Motor stops thats good, but it seems like there is an issue with the pot readings going haywire and 
// Ok to test there seems to be an issue with the timing variable, the == 0 caused a perma reset so it never updated
// Ideally ==  start @ 0, feed angle through serial mon, use the pot to read the current angle and head towards the goal, stop once you reach the goal
// Pot + Motor controller + DC motor + Voltage generator (12V) + Arduino == Should work 
// Youtube vids and arduino forms are worth looking at, They have hidden gems that you can adapt to work like below:


void setup() {

 Serial.begin(9600);
 Serial.setTimeout(1000); 
 pinMode(pot, INPUT);
 pinMode(MOTOR_F,OUTPUT);
 pinMode(MOTOR_B, OUTPUT);


 analogWrite(MOTOR_F, 0); // the DC motor pins start@ zero
 analogWrite(MOTOR_B, 0);


}

float angle_target = 0.0; // Ideally start @Zero
float angle_current = 0.0; // Float works for now, round off happens down in line 56
float angle_current_raw = 0.0;
float error = 0;

float kp = 1.2;
float angle_max = 30;

unsigned long lastTime = 0;
unsigned long sample = 20;   // smol boy when running (10–20 ms) and big when debug

void loop()
{
  unsigned long currentTime = millis();

  if(Serial.available() > 0)
  {
    float angle_input = Serial.parseFloat(); // Read the current pot 
   angle_target = constrain(angle_input, -angle_max, angle_max); // HARD LIMIT to avoied accidents
  }

  // Sample every T seconds for any update
  if(currentTime - lastTime >= sample)
  {
    lastTime = currentTime; // update 
    float pot_ang = analogRead(pot); // I hate this (hear me out float == decimal == round it off == better? idk its convoluted but it works)
    angle_current_raw = map(pot_ang, 0 , 1023, -angle_max, angle_max); // TL;DR match the limits to the pot

    angle_current = round(angle_current_raw * 10) / 10;
    error = angle_target - angle_current; // the diffrence is how we compute the PID
    // Now we should scale the PWM in a range with the pot:
    int pwm = abs(error * kp * (255.0 / angle_max));
   // pwm = constrain(pwm, 0, 255); // Possible issue 

   // if(pwm > 255) 
   //   pwm = 255;
   // else if(pwm < 0)
   //   pwm = 0;


    // An idea: (if this is does not work im *redacted* myself)
    // Control the motor direction using simple if
    float range = 1;
    if (error > range)
    {
      analogWrite(MOTOR_F, 0);    
      analogWrite(MOTOR_B, 255);  // Go left (or right can't rememeber)
    }
   else if (error > -range)
    {
      analogWrite(MOTOR_F, 255);    
      analogWrite(MOTOR_B, 0);  // Go left (or right can't rememeber)
    }
   else
   {
     analogWrite(MOTOR_F, 0);    
     analogWrite(MOTOR_B, 0);  // STOOOOOOOP, if not vittu
   } 

   // Print and hope this thing works if not, i give up
    Serial.print("Target: "); Serial.print(angle_target);
    Serial.print(" | Current: "); Serial.print(angle_current_raw);
    Serial.print(" | Err: "); Serial.print(error);
    Serial.print(" | PWM: "); Serial.println(pwm);

  }
}





// This below code was supposed to be used to check the sanity of the steering, go left, go right and then stop

/*
void loop() {
  if(currentTime - lastTime >= sample){
    delay(2000);

    analogWrite(MOTOR_F, 0);    
    analogWrite(MOTOR_B, 0); // We are gonna start @zero to test response
    delay(2000);

    analogWrite(MOTOR_F, 0);    
    analogWrite(MOTOR_B, 150);  // Go left (or right can't rememeber)
    delay(2000);

    analogWrite(MOTOR_F, 150);    
    analogWrite(MOTOR_B, 0);  // Go left (or right can't rememeber)
    delay(2000);

    analogWrite(MOTOR_F, 255);  // For fun what if we go all oiut in both directions
    analogWrite(MOTOR_B, 255);
    delay(2000);
  }
}
*/