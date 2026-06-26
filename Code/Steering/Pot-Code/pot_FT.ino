int pot = A3; // Analog read

void setup() {
  Serial.begin(9600);
}

void loop() {
  int value = analogRead(pot); // Read raw value 
  Serial.print(pot); // to see if there uis a change in vaue
  int angle = map(value, 0, 1023, 0, 100); // Scale to 0-100 range [Angle stuff using map function to print out values]
  
  Serial.print("Potentiometer angle: ");
  Serial.println(angle);
  
  delay(200); // Smoool delay
}

