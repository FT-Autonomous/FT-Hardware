/*
  This code is developed under the MYOSA (LearnTheEasyWay) initiative of MakeSense EduTech and Pegasus Automation.
  Code has been derived from internet sources and component datasheets.
  Existing readily-available libraries would have been used "AS IS" and modified for ease of learning purpose.
  
  Actuator Demo
  Connection: Connect the "Actuator" board from the MYOSA kit with the "Controller" board and power them up.
  Working: Turns Buzzer and AC switching ckt ON for 1 second and then turns it OFF.

  Synopsis of Actuator Board
  MYOSA Platform consists of an Actuator board. It is equiped with PCA9536 IC, a 4-bit I/O Expander with I2C operation.
  Hence, there are 4 Configurable I/O Ports available in the Actuator Board. We have utilized the ports as described below.
  1. ---> 5V Buzzer
  2. ---> AC switching Triac Circuit
  3. ---> Available for user configuration (Output Only)
  4. ---> Available for user configuration (Output Only)
  I2C Address of the board = 0x41.
  Detailed Information about Actuator board Library and usage is provided in the link below.
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
#include <Actuator.h>

/* Creating Object of Actuator Class */
Actuator gpioExpander;

/* Setup Function */
void setup() {

  /* Setting up communication */
  Serial.begin(115200);
  Wire.begin();
  Wire.setClock(100000);
  
  /* Setting up the Actuator Board. */
  for(;;)
  {
    if(gpioExpander.ping())
    {
      Serial.println("4bit IO Expander Actautor (PCA9536) is connected");
      break;
    }
    Serial.println("4bit IO Expander Actuator (PCA9536) is disconnected");
    delay(500u);
  }

  /* Set AC SWITCH IO as output */
  gpioExpander.setMode(AC_SWITCH_IO, IO_OUTPUT);
  gpioExpander.setState(AC_SWITCH_IO, IO_LOW);

  /* Set BUZZER IO as output */
  gpioExpander.setMode(BUZZER_IO, IO_OUTPUT);
  gpioExpander.setState(BUZZER_IO, IO_LOW);
  delay(2000);
  
  /* Turn-on AC SWITCH for one second */
  gpioExpander.setState(AC_SWITCH_IO, IO_HIGH);
  delay(1000);
  gpioExpander.setState(AC_SWITCH_IO, IO_LOW);
  delay(1000);
  
  /* Turn-on BUZZER for one second */
  gpioExpander.setState(BUZZER_IO, IO_HIGH);
  delay(1000);
  gpioExpander.setState(BUZZER_IO, IO_LOW);
  delay(1000);
}

/* Loop Function */
void loop() {

  /* Loop function does nothing */
  delay(1000u);
}