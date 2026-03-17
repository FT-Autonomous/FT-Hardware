/*
  This code is developed under the MYOSA (LearnTheEasyWay) initiative of MakeSense EduTech and Pegasus Automation.
  Code has been derived from internet sources and component datasheets.
  Existing readily-available libraries would have been used "AS IS" and modified for ease of learning purpose.

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

#ifndef __MYOSA_H__
#define __MYOSA_H__

#include <AccelAndGyro.h>
#include <AirQuality.h>
#include <BarometricPressure.h>
#include <LightProximityAndGesture.h>
#include <oled.h>
#include <Actuator.h>
#include <TempAndHumidity.h>
#include "VeraMono7pt7b.h"
#include "VeraMonoBold7pt7b.h"
#include "VeraMonoItalic7pt7b.h"

#define RELAY_IO    	IO0
#define BUZZER_IO   	IO1
/* define OLED display instance */
#define SCREEN_WIDTH 	128 // OLED display width, in pixels
#define SCREEN_HEIGHT 	64 // OLED display height, in pixels
// Declaration for an SSD1306 display connected to I2C (SDA, SCL pins)
#define OLED_RESET 		4 // Reset pin # (or -1 if sharing Arduino reset pin)

class MYOSA
{
	public:
		/* Create sensor objects */
		AccelAndGyro Ag;
		AirQuality Aq;
		BarometricPressure Pr;
		LightProximityAndGesture Lpg;
		Actuator gpioExpander;
		TempAndHumidity Th;
		oLed display;

		/* Variables */
		float altitude;
		uint16_t *rgbProportion;

		/* functions */
		MYOSA();
		bool begin(void);
		void printAceelAndGyro(void);
		void printAirQuality(void);
		void printBarometricPressure(void);
		void printLightProximityAndGesture(void);
		void printTempAndHumidity(void);
		void turnOnRelay();
		void turnOffRelay();
		void turnOnBuzzer(int print);
		void turnOffBuzzer(int print);
		void drawCentreString(const String &buf);
		void drawDegreeSymbol(void);
		void drawSuperscriptSymbol(const String &buf);
		void drawSubscriptSymbol(const String &buf);
		void sendBleData(void);
	private:
		const float refResitance = 10000.f;
};

#endif
