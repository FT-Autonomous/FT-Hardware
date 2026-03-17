/*
  This code is developed under the MYOSA (LearnTheEasyWay) initiative of MakeSense EduTech and Pegasus Automation.
  Code has been derived from internet sources and component datasheets.
  Existing readily-available libraries would have been used "AS IS" and modified for ease of learning purpose.

  Gesture Demo
  Connection: Connect the "Light Proximity and Gesture" board from the MYOSA kit with the "Controller" board and power them up.
  Working: Controller board prints (on Serial Monitor) the gesture detected continuously.
  
  Synopsis of Light Proximity and Gesture Board
  MYOSA Platform consists of an Light Proximity and Gesture Board. It is equiped with APDS9960 IC.
  It is a digital RGB, ambient light, proximity and gesture sensor device with I2C compatible interface.
  I2C Address of the board = 0x39.
  Detailed Information about Light Proximity and Gesture board Library and usage is provided in the link below.
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
#include <LightProximityAndGesture.h>

/* Creating Object of LightProximityAndGesture Class */
LightProximityAndGesture Lpg;

/* Setup Function */
void setup() {

  /* Setting up communication */
  Serial.begin(115200);
  Wire.begin();
  Wire.setClock(100000);
  
  /* Setting up the LightProximityAndGesture Board. */
  for(;;)
  {
    if(Lpg.begin())
    {
      Serial.println("Proximity, Ambient Light, RGB & Gesture sensor is connected...");
      break;
    }
    Serial.println("Proximity, Ambient Light, RGB & Gesture sensor is disconnected...");
    delay(500u);
  }
  Serial.println("APDS9960 initialization completed");

  /* Start running the gesture sensor engine */
  if( Lpg.enableGestureSensor(DISABLE) )
  {
    Serial.println("Gesture sensor is now running");
  }
  else
  {
    Serial.println("Something went wrong during gesture sensor init!");
  }

  /* Wait for initialization and calibration to finish */
  delay(500u);
}

/* Loop Function */
void loop() {

  /* Loop function continuously reads gesture data and print */
  if(Lpg.ping())
  {
    Lpg.getGesture();
  }
}