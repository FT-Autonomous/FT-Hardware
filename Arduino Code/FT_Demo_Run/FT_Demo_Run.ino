const int(dmotorf) = 10;
const int(dmotorb) = 11;  

void setup() {
  // put your setup code here, to run once:
  pinMode(dmotorf, OUTPUT); 
  pinMode(dmotorb, OUTPUT); 

  delay(5000);

  analogWrite(dmotorf, 120); // speed - no more than 120
  analogWrite(dmotorb, 0);

  delay(5000); // this is how long to run for 1000 = 1 sec

  analogWrite(dmotorf, 0);
  analogWrite(dmotorb, 0);
}

void loop() {
  // put your main code here, to run repeatedly:

}
