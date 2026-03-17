/*
  This code is developed under the MYOSA (LearnTheEasyWay) initiative of MakeSense EduTech and Pegasus Automation.
  Code has been derived from internet sources and component datasheets.
  Existing readily-available libraries would have been used "AS IS" and modified for ease of learning purpose.
  
  Air Quality Demo
  Connection: Connect the "Air Quality" board from the MYOSA kit with the "Controller" board and power them up.
  Working: Controller board prints (on Serial Monitor) the data of Total Volatile Organic Compounds (TVOCs) and equivalent carbon dioxide (eCO2) every second.

  Synopsis of Air Quality
  MYOSA Platform consists of an environmental Air Quality Board. It is equiped with CCS811 IC.
  It is a digital gas sesnor that senses wide range of TVOCs and eCO2. It is is intended for indoor air quality monitoring purposes.
  I2C Address of the board = 0x5B.
  Detailed Information about Air Quality board Library and usage is provided in the link below.
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
#include <AirQuality.h>

/* Creating Object of Air Quality Class */
AirQuality Aq(CCS811_I2C_ADDRESS1,refResitance);

/* Setup Function */
void setup() {
	
  /* Setting up communication */
  Serial.begin(115200);
  Wire.begin();
  Wire.setClock(100000);
  
  /* Setting up the Air Quality Board. */
  for(;;)
  {
    if(Aq.begin() == SENSOR_SUCCESS)
    {
      Serial.println("Air Quality sensor CCS811 is connected...");
      break;
    }
    Serial.println("Air Quality sensor ccs811 is disconnected...");
    delay(500u);
  }
  
  Serial.println("\nDevice Specifications");
  Serial.print("DEVICE ID       : 0x");
  Serial.println(Aq.getHwId(),HEX);
  Serial.print("HW VERSION      : ");
  Serial.println(Aq.getHwVersion());
  Serial.print("FW BOOT VERSION : ");
  Serial.println(Aq.getFwBootVersion());
  Serial.print("FW APP VERSION  : ");
  Serial.println(Aq.getFwAppVersion());
  Serial.println();
}

/* Loop function */
void loop() {
	
  /* Loop function continously prints data from the sensor every 1 second */
  if(Aq.ping())
  {
    /* Check if data is ready or not */
    if(Aq.isDataAvailable())
    {
      if(Aq.readAlgorithmResults() == SENSOR_SUCCESS)
      {
        Aq.getCO2();
        Aq.getTVOC();
        Serial.println();
      }
    }
  }
  delay(1010);
}