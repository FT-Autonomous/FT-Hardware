// FireBeetle ESP32 Serial Test
// Board: DFRobot FireBeetle ESP32 (DFR0478) — select "FireBeetle-ESP32" or "ESP32 Dev Module" in Arduino IDE
// Install: Add https://raw.githubusercontent.com/DFRobot/BoardManagerForDFRobot/master/package_DFRobot_index.json
//          to Arduino IDE > Preferences > Additional Board Manager URLs
//
// Refs:
//   https://www.dfrobot.com/product-1590.html
//   https://arduino.github.io/arduino-cli/0.32/getting-started/
//   https://forum.arduino.cc/t/serial-input-basics-updated/382007

#include "FTSerial.h"

#define BAUD_RATE 115200
#define MAX_MSG_LEN 64  // ESP32 has plenty of RAM — doubled from original 32

FTSerial ftSerial(Serial, MAX_MSG_LEN);

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

void printSerial() {
  String serialString = ftSerial.readUntilNewline();
  if (serialString != "") {
    Serial.print("received: ");
    Serial.println(serialString);
  }
}
