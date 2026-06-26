// This is a simple test code for the steering functionality (check your connections)


int pin = A3;
int MOTOR_A = 5;
int MOTOR_B = 6;


void setup() {
  // put your setup code here, to run once:
  Serial.begin(9600);
  pinMode(pin, INPUT);
  pinMode(MOTOR_A, OUTPUT);
  pinMode(MOTOR_B, OUTPUT);

}

void loop() {
  // put your main code here, to run repeatedly:
  Serial.println("one way");
  analogWrite(MOTOR_A, 125);
  analogWrite(MOTOR_B, 0);
  delay(2000);

  Serial.println("other way");
  analogWrite(MOTOR_A, 0);
  analogWrite(MOTOR_B, 125);
  delay(2000);

  Serial.println("stop");
  analogWrite(MOTOR_A, 0);
  analogWrite(MOTOR_B, 0);
  delay(2000);

}
