/*
  This code is developed under the MYOSA (LearnTheEasyWay) initiative of MakeSense EduTech and Pegasus Automation.
  Code has been derived from internet sources and component datasheets.
  Existing readily-available libraries would have been used "AS IS" and modified for ease of learning purpose.
  
  Get Date and Time from Internet
  Connection: Connect the "Controller" board and power it up.
  Working: This example is intended to further demonstrate the capabilities of WiFi (Controller) board. Here the controller board connects to a existing WiFi network, uses the internet to get the current date and time information from the internet.

  Synopsis of MYOSA platform
  MYOSA Platform consists of a centralized motherboard a.k.a Controller board, 5 different sensor modules, an OLED display and an actuator board in the kit.
  Controller board is designed on ESP32 module. It is a low-power system on a chip microcontrollers with integrated Wi-Fi and Bluetooth.
  5 Sensors are as below,
  1 --> Accelerometer and Gyroscope (6-axis motion sensor)
  2 --> Temperature and Humidity Sensor
  3 --> Barometric Pressure Sensor
  4 --> Light, Proximity and Gesture Sensor
  5 --> Air Quality Sensor
  Actuator board contains a Buzzer and an AC switching circuit to turn on/off an electrical appliance.
  There is also an OLED display in the MYOSA kit.

  You can design N number of such utility examples as a part of your learning from this kit.
  
  Detailed Information about MYOSA platform and usage is provided in the link below.
  Detailed Guide: https://drive.google.com/file/d/1On6kzIq3ejcu9aMGr2ZB690NnFrXG2yO/view

  NOTE
  All information, including URL references, is subject to change without prior notice.
  Please always use the latest versions of software-release for best performance.
  Unless required by applicable law or agreed to in writing, this software is distributed on an 
  "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied

  Modifications
  1 December, 2021 by Pegasus Automation
  (as a part of MYOSA Initiative)
  
  Contact Team MakeSense EduTech for any kind of feedback/issues pertaining to performance or any update request.
  Email: dev.myosa@gmail.com
*/

/* Library Inclusion - WiFi.h is generic ESP32 library available */
#include <WiFi.h>

/* Enter your WiFi credentials here so that MYOSA Controller Board can connect to Internet. */
const char* ssid       = "MYOSAbyMakeSense";
const char* password   = "LearnTheEasyWay";

/* NTP Server Host Name Used - pool.ntp.org. It is available worldwide. */
const char* ntpServer = "pool.ntp.org";

/* Adjust UTC Offset for local timezone. For India, it is UTC +5:30 hrs (=19,800 seconds). */
const long  UTCOffset_sec = 19800;

/* If your country uses daylight savings, update that information accordingly below. India don't use it. Hence 0 (ZERO) is set. */
const int   daylightOffset_sec = 0;

void printLocalTime()
{
  struct tm timeinfo;
  if(!getLocalTime(&timeinfo)){
    Serial.println("Failed to obtain time from the internet");
    return;
  }
  Serial.println(&timeinfo, "%A, %B %d %Y %H:%M:%S");
}

/* Setup Function */
void setup()
{

  /* Settinmg up the communication */
  Serial.begin(115200);
  
  /* Connecting  to WiFi */
  Serial.printf("Connecting to %s ", ssid);
  WiFi.begin(ssid, password);
  while (WiFi.status() != WL_CONNECTED) {
      delay(500);
      Serial.print(".");
  }
  Serial.println(" CONNECTED");
  
  /* init internal timer and get the time from the site */
  configTime(UTCOffset_sec, daylightOffset_sec, ntpServer);
  printLocalTime();

  /* disconnect WiFi as it's no longer needed */
  WiFi.disconnect(true);
  WiFi.mode(WIFI_OFF);
}

/* Loop Function */
void loop()
{
  
  /* Loop function continously prints time from the internal timer every 2 seconds */
  delay(2000);
  printLocalTime();
}