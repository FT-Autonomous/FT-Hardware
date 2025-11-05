int pinA = 10;
int pinB = 11;

void setup() {
  // put your setup code here, to run once:
  Serial.begin(9600);
  Serial.println("----------------------");
  pinMode(pinA, OUTPUT);
  pinMode(pinB, OUTPUT);
}

int st = 2000; // delay
int aVal = 100;
int bVal = 100;

void loop() {
  // put your main code here, to run repeatedly:
  a(0);
  b(0);
  Serial.println("both low");

  delay(st);

  a(aVal);
  Serial.println("A high: Extend");

  delay(st);

  a(0);
  Serial.println("both low");

  delay(st);
  
  b(bVal);
  Serial.println("b high: Retract");

  delay(st);

  Serial.println("----------------------");
}

void a(int state){
  state = floor(255*state/10);
  analogWrite(pinA, state);
}//set A to state percent power

void b(int state){
  state = floor(255*state/10);
  analogWrite(pinB, state);
}

