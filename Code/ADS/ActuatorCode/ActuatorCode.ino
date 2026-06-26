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
    // put your main code here, to run repeatedly:
    a(0);
    b(0);
    Serial.println("both low");

    delay(st);

    armEnable();
    
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
  else
    disarmAll();

  if(Serial.read() == 'c'){
    on = !on;
    Serial.print( "ON: " );
    Serial.println( on );
  }

}

void a(int state) {
  state = floor(255 * state / 10);
  analogWrite(pinA, state);
}  //set A to state percent power

void b(int state) {
  state = floor(255 * state / 10);
  analogWrite(pinB, state);
}

void disarmAll(){
  // the level shifters are on by default so we need to start with digital low
  digitalWrite(pinA; LOW);
  digitalWrite(pinB; LOW);

  disarmEnable();
  
}

void armEnable(){
  digitalWrite(L_En, HIGH);
  digitalWrite(R_En, HIGH);
}

void disarmEnable(){
  digitalWrite(L_En; LOW);
  digitalWrite(R_En; LOW);
}
