// FireBeetle ESP32 Serial Test
// Board: DFRobot FireBeetle ESP32 (DFR0478) — select "FireBeetle-ESP32" or "ESP32 Dev Module" in Arduino IDE
// Install: Add https://raw.githubusercontent.com/DFRobot/BoardManagerForDFRobot/master/package_DFRobot_index.json
//          to Arduino IDE > Preferences > Additional Board Manager URLs
//
// Refs:
//   https://www.dfrobot.com/product-1590.html
//   https://arduino.github.io/arduino-cli/0.32/getting-started/
//   https://forum.arduino.cc/t/serial-input-basics-updated/382007

#define BAUD_RATE 115200
#define MAX_MSG_LEN 64  // ESP32 has plenty of RAM — doubled from original 32

void setup() {
  Serial.begin(BAUD_RATE);
  delay(1000); // let ESP32 boot messages flush before we print
  Serial.println("FireBeetle serial test active");
}

void loop() {
  if (Serial.available() > 0) {
    printSerial();
  }
  yield(); // feed the ESP32 watchdog
}

// Reads messages wrapped in < > markers, e.g. <hello world>
String readSerialWithStartEndMarkers() {
  static boolean reading = false;
  static byte ndx = 0;
  char startMarker = '<';
  char endMarker = '>';
  char readCharacter;

  static char receivedCharacters[MAX_MSG_LEN];
  boolean newData = false;

  while (Serial.available() > 0 && !newData) {
    readCharacter = Serial.read();

    if (reading) {
      if (readCharacter != endMarker) {
        receivedCharacters[ndx++] = readCharacter;
        if (ndx >= MAX_MSG_LEN) {
          ndx = MAX_MSG_LEN - 1;
        }
      } else {
        receivedCharacters[ndx] = '\0';
        reading = false;
        ndx = 0;
        newData = true;
      }
    } else if (readCharacter == startMarker) {
      reading = true;
    }
  }

  if (newData) {
    return String(receivedCharacters);
  }
  return "";
}

// Reads until newline — type your message and press enter
String readSerialUntilNewline() {
  static byte ndx = 0;
  char readCharacter;
  static char receivedCharacters[MAX_MSG_LEN];

  while (Serial.available() > 0) {
    readCharacter = Serial.read();

    if (readCharacter == '\n') {
      receivedCharacters[ndx] = '\0';
      ndx = 0;
      return String(receivedCharacters);
    } else if (readCharacter != '\r') {
      receivedCharacters[ndx++] = readCharacter;
      if (ndx >= MAX_MSG_LEN) {
        ndx = MAX_MSG_LEN - 1;
      }
    }
  }

  return "";
}

void printSerial() {
  String serialString = readSerialUntilNewline();
  if (serialString != "") {
    Serial.print("received: ");
    Serial.println(serialString);
  }
}
