/*
  This code is developed under the MYOSA (LearnTheEasyWay) initiative of MakeSense EduTech and Pegasus Automation.
  Code has been derived from internet sources and component datasheets.
  Existing readily-available libraries would have been used "AS IS" and modified for ease of learning purpose.

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

#include "TempAndHumidity.h"

/**
 *
 */
TempAndHumidity::TempAndHumidity()
{
  _i2cSlaveAddress = Si7021_I2C_ADDRESS;
  _isConnected = false;
}

/**
 *
 */
bool TempAndHumidity::begin(void)
{
  if(reset())
  {
    delay_ms(Si7021_SOFT_RESET_DELAY);
    _isConnected = true;
    return true;
  }
  else
  {
    return false;
  }
}

/**
 *
 */
bool TempAndHumidity::reset(void)
{
  return writeByte(Si7021_RESET);
}

/**
 *
 */
bool TempAndHumidity::ping(void)
{
  bool getConnectSts = writeAddress();
  if(!_isConnected && getConnectSts)
  {
    begin();
  }
  _isConnected = getConnectSts;
  return getConnectSts;
}

/**
 *
 */
float TempAndHumidity::getRelativeHumdity(bool print)
{
  uint8_t data[3u];
  uint16_t RH_Code;
  float RH;
  /*
  if(readMultiBytes(Si7021_MEAS_RH_HOLD_MODE,3u,data) == false)
  {
    return 0.f;
  }
  */
  do {
    if(writeByte(Si7021_MEAS_RH_NOHOLD_MODE) == false)
    {
      RH = 0.f;
      continue;
    }
    delay_ms(25);
    if(readMultiBytes(3u,data) == false)
    {
      RH = 0.f;
      continue;
    }
    RH_Code = ((uint16_t)data[0u] << 8u)|data[1u];
    RH      = (((125.f*(float)RH_Code)/65536.f)-6.f);
  }while(0);
  /* print the value on serial terminal if required */
  if(print)
  {
    Serial.print("Relative Humidity (%): ");
    Serial.print(RH,2);
    Serial.println("%");
  }
  return RH;
}

/**
 *
 */
float TempAndHumidity::getTempC(bool print)
{
  uint8_t data[3u];
  uint16_t Temp_Code;
  float temperature;
  /*
  if(readMultiBytes(Si7021_MEAS_TEMP_HOLD_MODE,3u,data) == false)
  {
    return 0.f;
  }
  */
  do {
    if(writeByte(Si7021_MEAS_TEMP_NOHOLD_MODE) == false)
    {
      temperature = 0.f;
      continue;
    }
    delay_ms(25);
    if(readMultiBytes(3u,data) == false)
    {
      temperature = 0.f;
      continue;
    }
    Temp_Code   = ((uint16_t)data[0u] << 8u)|data[1u];
    temperature = (((175.72f*(float)Temp_Code)/65536.f)-46.85f);
  } while(0);
  /* print the value on serial terminal if required */
  if(print)
  {
    Serial.print("Temperature (°C): ");
    Serial.print(temperature,2);
    Serial.println("°C");
  }
  return temperature;
}

/**
 *
 */
float TempAndHumidity::getTempF(bool print)
{
  float temperature = (getTempC(false) * (9.f / 5.f)) + 32.f;
  if(print)
  {
    Serial.print("Temperature (°F): ");
    Serial.print(temperature,2);
    Serial.println("°F");
  }
  return temperature;
}

/**
 *
 */
 float TempAndHumidity::getHeatIndexC(bool print)
 {
   float T = getTempC(false);
   float RH = getRelativeHumdity(false);
   float HI = 0.f;

    float c1 = -8.78469475556;
    float c2 = 1.61139411;
    float c3 = 2.33854883889;
    float c4 = -0.14611605;
    float c5 = -0.012308094;
    float c6 = -0.0164248277778;
    float c7 = 0.002211732;
    float c8 = 0.00072546;
    float c9 = -0.000003582;
   HI = c1 + (c2*T) + (c3*RH) + (c4*T*RH) + (c5*T*T) + (c6*RH*RH) + (c7*T*T*RH) + (c8*T*RH*RH) + (c9*T*T*RH*RH);
    if(print)
    {
      Serial.print("Heat Index (°C): ");
      Serial.print(HI,2);
      Serial.println("°C");
    }
   return HI;
 }


/**
 *
 */
float TempAndHumidity::getHeatIndexF(bool print)
{
  float T = getTempF(false);
  float RH = getRelativeHumdity(false);
  float HI = 0.f;
  HI = -42.379f + (2.04901523f*T) + (10.14333127f*RH) - (0.22475541f*T*RH) -
       (0.00683783f*T*T) - (0.05481717f*RH*RH) + (0.00122874f*T*T*RH) +
       (0.00085282f*T*RH*RH) - (0.00000199f*T*T*RH*RH);
   if(print)
   {
     Serial.print("Heat Index (°F): ");
     Serial.print(HI,2);
     Serial.println("°F");
   }
  return HI;
}

/**
 *
 */
uint64_t TempAndHumidity::getSerialNumber(void)
{
  uint8_t data[8u];
  uint64_t serialNumber = 0u;
  if(writeByte(Si7021_ID1_CMD0,Si7021_ID1_CMD1))
  {
    if(readMultiBytes(sizeof(data),data))
    {
      serialNumber  = (((uint64_t)data[0u] << 56u)|
                      ((uint64_t)data[2u] << 48u)|
                      ((uint64_t)data[4u] << 40u)|
                      ((uint64_t)data[6u] << 32u));
    }
  }

  if(writeByte(Si7021_ID2_CMD0,Si7021_ID2_CMD1))
  {
    if(readMultiBytes(sizeof(data),data))
    {
      serialNumber |= (((uint64_t)data[0u] << 24u)|
                      ((uint64_t)data[2u] << 16u)|
                      ((uint64_t)data[4u] << 8u)|
                      ((uint64_t)data[6u]));
    }
  }
  Serial.print("Temperature and Humidity Sensor Serial Number: 0x");
  Serial.print((uint32_t)(serialNumber>>32),HEX);
  Serial.println((uint32_t)serialNumber,HEX);
  return serialNumber;
}

/**
 *
 */
char *TempAndHumidity::getFirmwareVersion(void)
{
  uint8_t data;
  char *version;
  version = (char *)"Unknown";
  if(writeByte(Si7021_FIMWARE_REV_CMD0,Si7021_FIMWARE_REV_CMD1))
  {
    if(readMultiBytes(sizeof(data),&data))
    {
      if(data == 0xFFu)
      {
        version = (char *)"1.0";
      }
      if(data == 0x20u)
      {
        version = (char *)"2.0";
      }
    }
  }
  Serial.print("Si7021 Chip Firmware Revision: ");
  Serial.println(version);
  return version;
}

/***********************************************************************************************
 * Platform dependent routines. Change these functions implementation based on microcontroller *
 ***********************************************************************************************/
/**
 *
 */
void TempAndHumidity::i2c_init(void)
{
  Wire.begin();
  Wire.setClock(100000);
}

/**
 *
 */
bool TempAndHumidity::readByte(uint8_t reg, uint8_t *in)
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
bool TempAndHumidity::readMultiBytes(uint8_t reg, uint8_t length, uint8_t *in)
{
  Wire.beginTransmission((uint8_t)_i2cSlaveAddress);
  Wire.write(reg);
  if(Wire.endTransmission(true) != 0)
  {
    return false;
  }
  Wire.requestFrom((uint8_t)_i2cSlaveAddress, (uint8_t)length, (uint8_t)1u);
  if(Wire.available() != length)
  {
    return false;
  }
  uint8_t nData;
  for(nData = 0u; nData < length; nData++)
  {
    in[nData] = Wire.read();
  }
  return true;
}

/**
 *
 */
bool TempAndHumidity::readMultiBytes(uint8_t length, uint8_t *in)
{
  Wire.requestFrom((uint8_t)_i2cSlaveAddress, (uint8_t)length, (uint8_t)1u);
  if(Wire.available() != length)
  {
    return false;
  }
  uint8_t nData;
  for(nData = 0u; nData < length; nData++)
  {
    in[nData] = Wire.read();
  }
  return true;
}

/**
 *
 */
bool TempAndHumidity::writeByte(uint8_t reg)
{
  Wire.beginTransmission((uint8_t)_i2cSlaveAddress);
  Wire.write(reg);
  if (Wire.endTransmission(true) == 0)
  {
    return true;
  }
  return false;
}

/**
 *
 */
bool TempAndHumidity::writeByte(uint8_t reg, uint8_t val)
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

/**
 *
 */
bool TempAndHumidity::writeAddress(void)
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
void TempAndHumidity::delay_ms(uint16_t ms)
{
  delay(ms);
}
