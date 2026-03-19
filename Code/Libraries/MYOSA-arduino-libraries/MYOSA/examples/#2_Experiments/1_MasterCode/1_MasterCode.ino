/*
  This code is developed under the MYOSA (LearnTheEasyWay) initiative of MakeSense EduTech and Pegasus Automation.
  Code has been derived from internet sources and component datasheets.
  Existing readily-available libraries would have been used "AS IS" and modified for ease of learning purpose.
  
  Master Code
  Connection: Connect all the boards from the MYOSA kit with the "Controller" board and power them up.
  Working: Controller board will display data (on the OLED board) from all the modules in cyclic fashion demonstrating complete RAW capabilities of the kit.

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

/* Library Inclusion */
#include <myosa.h>

/* Create Object of MYOSA class */
MYOSA myosa;

/* Set the timer to zero */
unsigned long previousMillis = 0;        // will store last time screen was updated

/* Global Constants */
const long perModuleInterval = 1500;           // interval at which screen will update next data (milliseconds)
uint8_t nScreen = 0u;

/* Setup Function */
void setup() {

  /* Setting up communication */
  Serial.begin(115200);
  Wire.begin();
  Wire.setClock(100000);

  /* This function initializes all the modules attached. */
  Serial.println(myosa.begin());
}

/* Loop Function */
void loop() {

	/* Loop Function make use of all the connected modules in cyclic form and prints the data/action in OLED display. */
	unsigned long currentMillis = millis();
	if (currentMillis - previousMillis >= perModuleInterval) {
		 previousMillis = currentMillis;
		 switch (nScreen) {
		 	case 0u:
				myosa.printAceelAndGyro();
				nScreen = 1u;
				break;
			case 1u:
				myosa.printAceelAndGyro();
				nScreen = 2u;
				break;
			case 2u:
				myosa.printAceelAndGyro();
				nScreen = 3u;
				break;
			case 3u:
				myosa.printAceelAndGyro();
				nScreen = 4u;
				break;
			case 4u:
				myosa.printAirQuality();
				nScreen = 5u;
				break;
			case 5u:
				myosa.printBarometricPressure();
				nScreen = 6u;
				break;
			case 6u:
				myosa.printBarometricPressure();
				nScreen = 7u;
				break;
			case 7u:
				myosa.printLightProximityAndGesture();
				nScreen = 8u;
				break;
			case 8u:
				myosa.printLightProximityAndGesture();
				nScreen = 9u;
				break;
			case 9u:
				myosa.printTempAndHumidity();
				nScreen = 10u;
				break;
			case 10u:
				myosa.printTempAndHumidity();
				nScreen = 0u;
				break;
			default:
				nScreen = 0u;
				break;
	 }
   myosa.sendBleData();
	}
}
