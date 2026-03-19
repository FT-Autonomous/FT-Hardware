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

#include "Actuator.h"

/*
 *
 */
Actuator::Actuator()
{
  _i2cSlaveAddress = Actuator_I2C_ADDRESS;
}

/*
 *
 */
bool Actuator::ping(void)
{
  return writeAddress();
}

/*
 *
 */
PIN_MODE_t Actuator::getMode(PCA_PIN_t pin)
{
  uint8_t mode;
  readByte(CONFIG_REG,&mode);
  return (PIN_MODE_t)((mode >> pin) & 0x01u);
}

/*
 *
 */
PIN_STATE_t Actuator::getState(PCA_PIN_t pin)
{
  uint8_t state;
  PCA_REG_t reg = getMode(pin) ? INPUT_REG : OUTPUT_REG ;
  readByte(reg,&state);
  return (PIN_STATE_t)((state >> pin) & 0x01u);
}

/*
 *
 */
PIN_POLARITY_t Actuator::getPolarity(PCA_PIN_t pin)
{
  uint8_t polarity;
  readByte(POLARITY_REG,&polarity);
  return (PIN_POLARITY_t)((polarity >> pin) & 0x01u);
}

/*
 *
 */
void Actuator::setMode(PCA_PIN_t pin, PIN_MODE_t newMode)
{
  uint8_t mode_all;
  readByte(CONFIG_REG,&mode_all);
  mode_all &= ~(1u << pin);
  mode_all |= (newMode << pin);
  writeByte(CONFIG_REG,mode_all);
}

/*
 *
 */
void Actuator::setMode(PIN_MODE_t newMode)
{
  uint8_t mode_all = newMode ? ALL_INPUT : ALL_OUTPUT;
  writeByte(CONFIG_REG,mode_all);
}

/*
 *
 */
void Actuator::setState(PCA_PIN_t pin, PIN_STATE_t newState)
{
  uint8_t state_all;
  readByte(OUTPUT_REG,&state_all);
  state_all &= ~(1u << pin);
  state_all |= (newState << pin);
  writeByte(OUTPUT_REG,state_all);
}

/*
 *
 */
void Actuator::setState(PIN_STATE_t newState)
{
  uint8_t state_all = newState ? ALL_HIGH : ALL_LOW;
  writeByte(OUTPUT_REG,state_all);
}

/*
 *
 */
void Actuator::toggleState(PCA_PIN_t pin)
{
  uint8_t state_all;
  readByte(OUTPUT_REG,&state_all);
  state_all ^= (1u << pin);
  writeByte(OUTPUT_REG,state_all);
}

/*
 *
 */
void Actuator::toggleState(void)
{
  uint8_t state_all;
  readByte(OUTPUT_REG,&state_all);
  state_all ^= 0xFFu;
  writeByte(OUTPUT_REG,state_all);
}

/*
 *
 */
void Actuator::setPolarity(PCA_PIN_t pin, PIN_POLARITY_t newPolarity)
{
  uint8_t polarity_all;
  if(getMode(pin) == IO_INPUT)
  {
    readByte(POLARITY_REG,&polarity_all);
    polarity_all &= ~(1u << pin);
    polarity_all |= (newPolarity << pin);
    writeByte(POLARITY_REG,polarity_all);
  }
}

/*
 *
 */
void Actuator::setPolarity(PIN_POLARITY_t newPolarity)
{
  uint8_t polarity_all;
  uint8_t polarity_msk;
  uint8_t polarity_new;
  readByte(POLARITY_REG,&polarity_all);
  readByte(CONFIG_REG,&polarity_msk);
  polarity_new = newPolarity ? ALL_INVERTED : ALL_NON_INVERTED;
  writeByte(POLARITY_REG,(polarity_all & ~polarity_msk) | (polarity_new & polarity_msk));
}

/***********************************************************************************************
 * Platform dependent routines. Change these functions implementation based on microcontroller *
 ***********************************************************************************************/
/**
 *
 */
void Actuator::i2c_init(void)
{
  Wire.begin();
  Wire.setClock(100000);
}

/**
 *
*/
bool Actuator::readByte(uint8_t reg, uint8_t *in)
{
  Wire.beginTransmission((uint8_t)_i2cSlaveAddress);
  Wire.write(reg);
  if(Wire.endTransmission(true) != 0)
  {
    return false;
  }
  Wire.requestFrom((uint8_t)_i2cSlaveAddress, (uint8_t)1u, (uint8_t)1u);
  if(Wire.available() != 1u)
  {
    return false;
  }
  *in = Wire.read();
  return true;
}

/**
 *
 */
bool Actuator::writeAddress(void)
{
  Wire.beginTransmission((uint8_t)_i2cSlaveAddress);
  if (Wire.endTransmission(true) == 0)
  {
    return true;
  }
  return false;
}

/**
 *
 */
bool Actuator::writeByte(uint8_t reg, uint8_t val)
{
  Wire.beginTransmission((uint8_t)_i2cSlaveAddress);
  Wire.write(reg);
  Wire.write(val);
  if (Wire.endTransmission(true) == 0)
  {
    return true;
  }
  return false;
}
