int pot = A3; // Analog read
int MOTOR_F = 5;
int MOTOR_B = 6;

void setup() {
  Serial.begin(9600);
  pinMode(MOTOR_F, OUTPUT);
  pinMode(MOTOR_B, OUTPUT);
}

void loop() {
  int value = analogRead(pot); // Read raw value 
  Serial.print(pot); // to see if there uis a change in vaue
  int angle = map(value, 0, 1023, 0, 100); // Scale to 0-100 range [Angle stuff using map function to print out values]
  
  Serial.print("Potentiometer angle: ");
  Serial.println(angle);
  
  //delay(200); // Smoool delay
  analogWrite(MOTOR_F, 150); // Turn right for 2s
  analogWrite(MOTOR_B, 0);
  delay(2000);

  analogWrite(MOTOR_F, 0); // Turn Left for 2s
  analogWrite(MOTOR_B, 0);
  delay(2000);

  analogWrite(MOTOR_F, 255); // hold for 2s
  analogWrite(MOTOR_F, 255);
  delay(2000);




}


