/*
  This code is developed under the MYOSA (LearnTheEasyWay) initiative of MakeSense EduTech and Pegasus Automation.
  Code has been derived from internet sources and component datasheets.
  Existing readily-available libraries would have been used "AS IS" and modified for ease of learning purpose.

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

#ifndef __PCA9536_H__
#define __PCA9536_H__

#include <stdint.h>
#include <Wire.h>

#define Actuator_I2C_ADDRESS     0x41u

/* Defining Relay and Buzzer IO */
#define AC_SWITCH_IO    IO0
#define BUZZER_IO   IO1

const uint8_t ALL_INPUT        = 0xFF;
const uint8_t ALL_OUTPUT       = 0x00;
const uint8_t ALL_LOW          = 0x00;
const uint8_t ALL_HIGH         = 0xFF;
const uint8_t ALL_NON_INVERTED = 0x00;
const uint8_t ALL_INVERTED     = 0xFF;

/*!
 * List of registers available in PCA9536 4bit I/O exanpder
 */
typedef enum
{
  INPUT_REG   = 0u,
  OUTPUT_REG  = 1u,
  POLARITY_REG= 2u,
  CONFIG_REG  = 3u
}PCA_REG_t;

/*!
 * list of I/O pins in PCA9536 4bit I/O exanpder
 */
typedef enum
{
  IO0 = 0u,
  IO1 = 1u,
  IO2 = 2u,
  IO3 = 3u
}PCA_PIN_t;

/*!
 * PCA9536 4bit I/O exanpder IO pin modes
 */
typedef enum
{
  IO_OUTPUT = 0u,
  IO_INPUT  = 1u
}PIN_MODE_t;

/*!
 * PCA9536 4bit I/O exanpder IO pin state control values
 */
typedef enum
{
  IO_LOW   = 0u,
  IO_HIGH  = 1u
}PIN_STATE_t;

/*!
 * PCA9536 4bit I/O exanpder IO pin polarity control values
 */
typedef enum
{
  IO_NON_INVERTED  = 0u,
  IO_INVERTED      = 1u
}PIN_POLARITY_t;

class Actuator
{
  public:
    Actuator();
    bool ping(void);
    PIN_MODE_t getMode(PCA_PIN_t pin);
    PIN_STATE_t getState(PCA_PIN_t pin);
    PIN_POLARITY_t getPolarity(PCA_PIN_t pin);
    void setMode(PCA_PIN_t pin, PIN_MODE_t newMode);
    void setMode(PIN_MODE_t newMode);
    void setState(PCA_PIN_t pin, PIN_STATE_t newState);
    void setState(PIN_STATE_t newState);
    void toggleState(PCA_PIN_t pin);
    void toggleState(void);
    void setPolarity(PCA_PIN_t pin, PIN_POLARITY_t newPolarity);
    void setPolarity(PIN_POLARITY_t newPolarity);
  private:
    uint8_t _i2cSlaveAddress;
    void i2c_init(void);
    bool readByte(uint8_t reg, uint8_t *in);
    bool writeAddress(void);
    bool writeByte(uint8_t reg, uint8_t out);
};

#endif
