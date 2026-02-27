
void wifiSetup(){
  
  WiFi.begin(ssid, pass);
  IPAddress ip = WiFi.localIP();  //get the IP
  IPAddress vip = WiFi.localIP();  //get the IP
  Serial.println(ip);             //print IP so it can be easily added to the processing code


  while(ip == vip){
    WiFi.begin(ssid, pass);
    IPAddress vip = WiFi.localIP();  //get the IP
    Serial.println(vip);             //print IP so it can be easily added to the processing code
    if(vip != ip)
      break;
  }

  R4.begin();

}

void wifiLoop(){
  WiFiClient laptop = R4.available();

  while (laptop.available()) {  // if process
    char temp = laptop.read();
    command(temp, laptop);
  }
}

void command(char data, WiFiClient laptop){
  if( data == 'p')
    paused = !paused;
  else if(data == 's'){
    //prepare to receive 3 value updates: A B and t
    aVal = getInt(laptop);
    Serial.print("aVal set to: ");
    Serial.println(aVal);

    bVal = getInt(laptop);
    Serial.print("bVal set to: ");
    Serial.println(bVal);

    st = getInt(laptop);
    Serial.print("delay set to: ");
    st = st*1000;
    Serial.println(st);
    
  }
  else if (data == 'r'){
    count = 0;  //set counter to 0
    countR = 0;
  }
  else if ( data == 'l'){
    cycleTest = !cycleTest;
  }
  else if( data == 'a'){
    //set aVal to incoming integer
    aVal = getInt(laptop);
    Serial.print("aVal set to: ");
    Serial.println(aVal);
    cycleTest = false;
  }
  else if( data == 'b'){
    //set bVal to incoming integer
    bVal = getInt(laptop);
    Serial.print("bVal set to: ");
    Serial.println(bVal);
    cycleTest = false;
  }


}

int getInt(WiFiClient laptop){
  int temp = -1;
  while(temp == -1){
    temp = laptop.read();
  }
  return temp;
}

void sendUpdate(){
  WiFiClient laptop = R4.available();
  if(laptop.connected()){
    laptop.write('c');
    laptop.write(count);
  }
}