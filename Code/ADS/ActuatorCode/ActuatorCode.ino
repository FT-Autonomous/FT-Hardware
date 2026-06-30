int pinA = 27;  // D4 = RPWM
int pinB = 26;  // D3 = LPWM

int L_En = 25;  //D2
int R_En = 13;  //D7
// D2 Len D7 Ren

bool on;
bool go;

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

  on = false;
  go = true;
  Serial.print("ON: ");
  Serial.println(on);
}

int st = 1000;  // delay
//int aVal = 100;int bVal = 100;

void loop() {
  if (on) {
    digitalWrite(pinA, LOW);
    digitalWrite(pinB, LOW);
    Serial.println("both low");

    delay(st);

    //armEnable();

    digitalWrite(pinA, HIGH);
    Serial.print("A high: Extend");

    delay(st);
    if (!go) {
      Serial.print(" (enter 'r' to Retract)");
      while (Serial.read() != 'r'){}
      //Wait for request for retraction
    }
    Serial.println();

    digitalWrite(pinA, LOW);
    Serial.println("both low");

    delay(st);

    digitalWrite(pinB, HIGH);
    Serial.print("B high: Retract");

    delay(st);
    if (!go) {
      Serial.print(" (enter 'e' to extend)");
      while (Serial.read() != 'e'){}
      //Wait for request for extension
    }
    Serial.println();

    digitalWrite(pinB, LOW);
    Serial.println("----------------------");
  }
  
  char curr = Serial.read();
  if (curr == 'p') {
    //receiving the char p over serial pauses and unpauses the loop (this is functional)
    on = !on;
    Serial.print("ON: ");
    Serial.println(on);
  }

  else if (curr == 'g')
    go = !go;
}  //end loop

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
