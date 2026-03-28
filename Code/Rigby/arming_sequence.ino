const int LPWM = 9; 
const int RPWM = 10;
const int L_EN = 25;
const int R_EN = 26;
const int LED = 27;

bool isArmed = false;

void setup() {
  Serial.begin(115200);

  // 1. Immediately force all pins LOW
  pinMode(LPWM, OUTPUT);
  pinMode(RPWM, OUTPUT);
  pinMode(L_EN, OUTPUT);
  pinMode(R_EN, OUTPUT);
  pinMode(LED, OUTPUT);
  
  digitalWrite(LPWM, LOW);
  digitalWrite(RPWM, LOW);
  digitalWrite(L_EN, LOW); // Keep driver disabled initially
  digitalWrite(R_EN, LOW);
  digitalWrite(LED, LOW); // LED starts OFF

  Serial.println("SYSTEM INACTIVE. Type 'ARM' to begin...");

  // 2. Wait for Serial Command
  while (!isArmed) {
    if (Serial.available() > 0) {
      String input = Serial.readStringUntil('\n');
      input.trim();
      if (input == "ARM") {
        isArmed = true;
        Serial.println("!!! SYSTEM ARMED - MOTORS LIVE !!!");
      }
    }
    delay(100); 
  }

  // 3. Enable the driver only after arming
  digitalWrite(L_EN, HIGH);
  digitalWrite(R_EN, HIGH);
  digitalWrite(LED, HIGH); // LED turns on when system armed
}

void loop() {
  analogWrite(LPWM, 255);
  analogWrite(RPWM, 0);
  delay(1000);
  analogWrite(LPWM, 0);
  analogWrite(RPWM, 255);
  delay(1000);
}
