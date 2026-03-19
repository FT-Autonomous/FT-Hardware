/*
  This code is developed under the MYOSA (LearnTheEasyWay) initiative of MakeSense EduTech and Pegasus Automation.
  Code has been derived from internet sources and component datasheets.
  Existing readily-available libraries would have been used "AS IS" and modified for ease of learning purpose.

  TempAndHumidity Demo
  Connection: Connect the "Temperature And Humidity" board from the MYOSA kit with the "Controller" board and power them up.
  Working: Controller board prints (on Serial Monitor) the data of Temperature, Relative Humidity and Heat Index every second.  
  
  Synopsis of Temperature And Humidity Board
  MYOSA Platform consists of an Temperature And Humidity Board. It is equiped with Si7021 IC.
  It has ± 3% relative humidity measurements with a range of 0–80% RH, and ±0.4 °C temperature accuracy at a range of -10 to +85 °C.
  I2C Address of the board = 0x40.
  Detailed Information about Temperature and Humidity board Library and usage is provided in the link below.
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
#include <TempAndHumidity.h>

/* Creating Object of TempAndHumidity Class */
TempAndHumidity Th;

/* Setup Function */
void setup(void){
  
  /* Setting up communication */
  Serial.begin(115200);
  Wire.begin();
  Wire.setClock(100000);
  
  /* Setting up the TempAndHumidity Board. */
  for(;;)
  {
    if(Th.begin())
    {
      Serial.println("Temperature and Humidity Sensor is Connected");
      break;
    }
    Serial.println("Temperature and Humidity Sensor is Disconnected");
    delay(500u);
  }
  Th.getFirmwareVersion();
  Th.getSerialNumber();

}

/* Loop Function */
void loop(void){
  
  /* Loop function continuously gets data and print at every second */
  if(Th.ping())
  {
    Th.getRelativeHumdity();
    Th.getTempC();
    Th.getTempF();
    Th.getHeatIndexC();
    Th.getHeatIndexF();
    Serial.println();
  }
  delay(1000u);
}