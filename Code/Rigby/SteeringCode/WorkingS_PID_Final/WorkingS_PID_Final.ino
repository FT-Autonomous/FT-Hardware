//Latest code as of 27/02/2026 (Hi Conor)
// The CODE WORKS despite the alligations that it does not, tested it again using a diffrent pot
// The issue? the wires to the pot are not secured, values get stuck to a fixed value (0 in this case)

const int pot = A3; // Analog read
const int MOTOR_B = 5; // DC Motor that contorls the steering module
const int MOTOR_F = 6;

// Resrouces: https://forum.arduino.cc/t/dc-motor-pid-control-with-arduino-motor-shield-and-encoder/217946/3 & https://forum.arduino.cc/t/how-do-i-control-the-position-of-my-dc-motor-by-using-a-pot/251307/20

// Ideally ==  start @ 0, feed angle through serial mon, use the pot to read the current angle and head towards the goal, stop once you reach the goal

// One thing left to do (27/02/2026): Check the pot, the readings are not present here, idk why or how but that should 
//To do: find a way to reduce the swing rate needed for the pot to read an angle
// the currecnt setup assumes that the pot movement is bigger than what acctually happens, so figure this out
// think 


// this code does NOT work as of 27/02/2026 -- tested on wokwi, target angle did not get properly set
// i am going to tidy it up and then start debugging it
// blocking code shall also be eliminated because yea


void setup() {

 Serial.begin(9600);
 Serial.setTimeout(1000); 
 pinMode(pot, INPUT);
 pinMode(MOTOR_F,OUTPUT);
 pinMode(MOTOR_B, OUTPUT);


 analogWrite(MOTOR_F, 0); // the DC motor pins start@ zero
 analogWrite(MOTOR_B, 0);


}

float angle_target; // Ideally start @Zero 
float angle_current = 0.0; // Float works for now, round off happens down in line 56
float angle_current_raw = 0.0;
float error = 0;

char* str_dir = "N";

float kp = 1.2;
float angle_max = 30; 

unsigned long lastTime = 0;
unsigned long sample = 10;   // smol boy when running (10–20 ms) and big when debug

void loop() {
  unsigned long currentTime = millis();

  if(Serial.available() > 0)
  {
    float angle_input = Serial.parseFloat(); // Read the current pot 
    angle_target = constrain(angle_input, -angle_max, angle_max); // HARD LIMIT to avoied accidents
  }

  // Sample every T seconds for any update --- why are we doing this instead of running continiously?
  if(currentTime - lastTime >= sample)
  {
    lastTime = currentTime; // update 
    int pot_ang = analogRead(pot); // Update, use int insted 
    angle_current = map(pot_ang, 0 , 1023, -angle_max, angle_max); // TL;DR match the limits to the pot

    //angle_current = round(angle_current_raw * 10) / 10;
    error = angle_target - angle_current; // the diffrence is how we compute the PID

    // Now we should scale the PWM in a range with the pot:
    int pwm = abs(error * kp * (255.0 / angle_max));
    pwm = constrain(pwm, 0, 255); // Possible issue with the limit of pwm, wraps value like a clock

    
    // Control the motor direction using simple if
    str_dir = motor_power(error, pwm)

   // Print and hope this thing works if not, i give up
    output(angle_target, angle_current, error, pwm, pot_ang, str_dir);
  }
}

char* motor_power(float error, int pwm){
  // Control the motor direction using simple if
    float range = 1;
    char* str_dir;
    if (error >= range) //if error is greater than 1
    {
      analogWrite(MOTOR_F, 0);    
      analogWrite(MOTOR_B, pwm);  // Go left (or right can't rememeber)
      str_dir = "L";
    }
   else if (error <= -range) //if error is greater than -1 but not greatrer than 1
    {
      analogWrite(MOTOR_F, pwm);    
      analogWrite(MOTOR_B, 0);  // Go left (or right can't rememeber)
      str_dir = "R";
    }
   else //if error is less than 1 and less than -1 
   {
     analogWrite(MOTOR_F, 0);    
     analogWrite(MOTOR_B, 0);  // STOOOOOOOP, if not vittu
     str_dir = "S";
   } 
   return str_dir;
}


void output(float angle_target, float angle_current, float error, int pwm, int pot_ang, char* str_dir){
  Serial.print("Target: "); Serial.print(angle_target);
  Serial.print(" | Current: "); Serial.print(angle_current);
  Serial.print(" | Err: "); Serial.print(error);
  Serial.print(" | PWM: "); Serial.print(pwm);
  Serial.print(" | ANG: "); Serial.println(pot_ang);
  Serial.print(" | DIR: "); Serial.println(str_dir);
}