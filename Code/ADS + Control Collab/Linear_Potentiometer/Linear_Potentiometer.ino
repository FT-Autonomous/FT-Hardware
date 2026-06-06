const int potentiometerPin = A3;

const float sliderTravelMM = 100.0;

// Global current position estimate in millimeters
float currentSliderMM = 0.0;

void setup() {
  Serial.begin(9600);
}

void loop() {
  int rawValue = analogRead(potentiometerPin);

  float percentage = rawValue / 1023.0;
  currentSliderMM = percentage * sliderTravelMM;

  Serial.print("Raw: ");
  Serial.print(rawValue);

  Serial.print("  Percent: ");
  Serial.print(percentage * 100.0, 1);
  Serial.print("%");

  Serial.print("  Approx Position: ");
  Serial.print(currentSliderMM, 2);
  Serial.println(" mm");

  delay(100);
}