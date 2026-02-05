
// Links that helped: https://arduino.github.io/arduino-cli/0.32/getting-started/
// https://forum.arduino.cc/t/two-ways-communication-between-python3-and-arduino/1219738
// https://forum.arduino.cc/t/serial-input-basics-updated/382007




void setup() {
  Serial.begin(9600);
  while (!Serial); // test test
  Serial.println("Bruz");
}

// loop is very close to how the encoder PID serial is setup 
// EDIT: not sure if this still applies post redoing serial
void loop() {
  if (Serial.available() > 0) { 
    printSerial();
  }
}

// this function mainly exists as proof of concept that we are no longer
// limited to single char messages to the arduino and to be easily transferable
String readSerialWithStartEndMarkers(){ 
  static boolean reading=false;
  static byte ndx=0;
  char startMarker='<';
  char endMarker='>';
  char readCharacter;

  const byte numChars=32;
  char receivedCharacters[numChars];
  boolean newData=false;


  while(Serial.available()>0&&!newData){
    readCharacter=Serial.read();

    if(reading){
      if(readCharacter!=endMarker){
        receivedCharacters[ndx++] = readCharacter;
        if(ndx>=numChars){
          ndx=numChars-1;
        }
      }
      else{
        receivedCharacters[ndx]='\0';
        reading=false;
        ndx=0;
        newData=true;
      }
    }

    else if(readCharacter==startMarker){
      reading=true;
    }
  }

  if(newData){
    return receivedCharacters;
  } else {
    return "";
  }
}

void printSerial(){
  String serialString = readSerialWithStartEndMarkers();
  if(serialString!=""){
    Serial.print("received: ");
    Serial.println(serialString);
  }
}