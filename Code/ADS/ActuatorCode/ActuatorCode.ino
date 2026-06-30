int pinA = 27;  // D4 = RPWM
int pinB = 26;  // D3 = LPWM

int L_En = 25;  //D2
int R_En = 13;  //D7
// D2 Len D7 Ren

void setup() {
  // put your setup code here, to run once:
  Serial.begin(9600);
  Serial.println("----------------------");
  pinMode(pinA, OUTPUT);
  pinMode(pinB, OUTPUT);

  pinMode(L_En, OUTPUT);
  pinMode(R_En, OUTPUT);

  disarmAll();
  armEnable();  // toggle Enables pins High
}

int st = 1000;  // delay
int aVal = 100;
int bVal = 100;

bool on = false;
void loop() {
  if (on) {
    digitalWrite(pinA, LOW);
    digitalWrite(pinB, LOW);
    Serial.println("both low");

    delay(st);

    //armEnable();

    digitalWrite(pinA, HIGH);
    Serial.println("A high: Extend");

    delay(st);

    digitalWrite(pinA, LOW);
    Serial.println("both low");

    delay(st);

    digitalWrite(pinB, HIGH);
    Serial.println("b high: Retract");

    delay(st);
    digitalWrite(pinB, LOW);
    Serial.println("----------------------");
  }

  if (Serial.read() == 'p') {
    //receiving the char p over serial pauses and unpauses the loop (this is functional)
    on = !on;
    Serial.print("ON: ");
    Serial.println(on);
  }
}  //end loop

void a(int state) {
  state = floor(255 * state / 10);
  analogWrite(pinA, state);
}  //set A to state percent power

void b(int state) {
  state = floor(255 * state / 10);
  analogWrite(pinB, state);
}

void disarmAll() {
  // the level shifters are on by default so we need to start with digital low
  digitalWrite(pinA, LOW);
  digitalWrite(pinB, LOW);

  disarmEnable();
}

void armEnable() {
  digitalWrite(L_En, HIGH);
  digitalWrite(R_En, HIGH);
}

void disarmEnable() {
  digitalWrite(L_En, LOW);
  digitalWrite(R_En, LOW);
}
