// Reads a PWM value (0-255) from the Serial Monitor
// and outputs it on pin 3

const int pwmPin = 3;
const int motorOutput = 2;

int pwmValue = 0;
int inputVal = 0;
int prevVal = inputVal;

int tolerance = 5;

// ===== RPM VARIABLES =====

const int polePairs = 2;
const int MAXRPM = 3500;
// Driver outputs 3 * polePairs pulses per revolution
const int pulsesPerRev = 3 * polePairs;  // = 6 pulses/rev

volatile unsigned long pulseCount = 0;

float currentRPM = 0;

unsigned long prevRPMTime = 0;
const int rpmUpdateTime = 100;  // ms


int targetRPM = 0;

void setup() {
  pinMode(pwmPin, OUTPUT);
  pinMode(motorOutput, INPUT_PULLUP);

  // Interrupt for PG signal
  attachInterrupt(digitalPinToInterrupt(motorOutput), countPulse, RISING);

  Serial.begin(9600);
  Serial.println("Enter targetRPM value (0-3500):");

  prevRPMTime = millis();
}

void loop() {
  if (Serial.read() == 'c') {

    // Read integer from Serial Monitor
    targetRPM = Serial.parseInt();
    targetRPM = constrain(targetRPM, 0, MAXRPM);

    // Print confirmation
    Serial.print("RPM target set to: ");
    Serial.println(targetRPM);
  }

  int dumb = 1;
  if (millis() > 2000*dumb) {
    PID();
    dumb++;
  }


  // Output PWM signal
  analogWrite(pwmPin, pwmValue);
}


void updateRPM() {
  if (millis() - prevRPMTime >= rpmUpdateTime) {

    noInterrupts();
    unsigned long pulses = pulseCount;
    pulseCount = 0;
    interrupts();

    float deltaTime = (millis() - prevRPMTime) / 1000.0;

    currentRPM = (pulses * 60.0) / (pulsesPerRev * deltaTime);

    prevRPMTime = millis();

    if (abs(currentRPM - prevVal) > tolerance) {

      Serial.print("---------------RPM = ");
      Serial.println(currentRPM);

      prevVal = currentRPM;
    }
  }
}

// ===== INTERRUPT =====

void countPulse() {
  pulseCount++;
}