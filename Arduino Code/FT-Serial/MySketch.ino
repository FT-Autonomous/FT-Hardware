
// Links that helped: https://arduino.github.io/arduino-cli/0.32/getting-started/
// https://forum.arduino.cc/t/two-ways-communication-between-python3-and-arduino/1219738


void setup() {
  Serial.begin(9600);
  while (!Serial); // test test
  Serial.println("Bruz");
}

// loop is very close to how the encoder PID serial is setup
void loop() {
  if (Serial.available() > 0) { 
    String data = Serial.readStringUntil('\n'); 
    Serial.print("received: ");
    Serial.println(data);
  }
}
