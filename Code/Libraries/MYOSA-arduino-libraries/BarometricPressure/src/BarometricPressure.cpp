/*
  This code is developed under the MYOSA (LearnTheEasyWay) initiative of MakeSense EduTech and Pegasus Automation.
  Code has been derived from internet sources and component datasheets.
  Existing readily-available libraries would have been used "AS IS" and modified for ease of learning purpose.
 
  Synopsis of Barometric Pressure Board
  MYOSA Platform consists of a Barometric Pressure Board. It is equiped with BMP180 IC which has a pressure sensing range
  of 300-1100 hPa (9000m to -500m above sea level), with a precision up to 0.03hPa/0.25m resolution.
  It also have temperature sensing element with -40 to +85°C operational range, ±2°C temperature accuracy.
  I2C Address of the board = 0x77u.
  Detailed Information about Barometric Pressure board Library and usage is provided in the link below.
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

#include "BarometricPressure.h"

/**
 *   @brief constructor to initialise the BMP180 sampling accuracy mode
 */
BarometricPressure::BarometricPressure(bmp180AccuracyMode_t accr){
  _accuracy = accr;
  _i2cSlaveAddress = BMP180_I2C_ADDRESS;
  _isConnected = false;
}

/**
 *
 */
void BarometricPressure::setAccuracyMode(bmp180AccuracyMode_t mode){
  _accuracy = mode;
}

/**
 *
 */
bool BarometricPressure::begin(void){
  /* Check device ID to verify the communication establishment */
  if(getDeviceId() != BMP180_CHIP_ID)
  {
    return false;
  }
  _isConnected = true;
  /* Get the sensor coefficeints */
  return readCalibrationCoefficients();
}

/**
 *
 */
bool BarometricPressure::ping(void)
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
float BarometricPressure::getTemperature(void){
  /* get the raw temperature */
  int16_t UT = readRawTemperature();
  /* Compute the temperature */
  if (UT == BMP180_ERROR)
  {
    return BMP180_ERROR;
  }
  float temperature = ((computeB5(UT) + 8) >> 4)/10.f;
  return temperature;
}

/**
 *
 */
float BarometricPressure::getTempC(bool print)
{
  float temperature = getTemperature();
  if(temperature == BMP180_ERROR)
  {
    temperature = 0.f;
  }
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
float BarometricPressure::getTempF(bool print)
{
  float temperature = (getTemperature() * (9.f/5.f)) + 32.f;
  if(temperature == BMP180_ERROR)
  {
    temperature = 0.f;
  }
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
 int32_t BarometricPressure::getPressure(void)
 {
   int32_t  UT       = 0;
   int32_t  UP       = 0;
   int32_t  B3       = 0;
   int32_t  B5       = 0;
   int32_t  B6       = 0;
   int32_t  X1       = 0;
   int32_t  X2       = 0;
   int32_t  X3       = 0;
   int32_t  pressure = 0;
   uint32_t B4       = 0;
   uint32_t B7       = 0;

   UT = readRawTemperature();                           //read uncompensated temperature, 16-bit
   if (UT == BMP180_ERROR) return BMP180_ERROR;         //error handler, collision on i2c bus

   UP = readRawPressure();                              //read uncompensated pressure, 19-bit
   if (UP == BMP180_ERROR) return BMP180_ERROR;         //error handler, collision on i2c bus

   B5 = computeB5(UT);

   /* pressure calculation */
   B6 = B5 - 4000;
   X1 = ((int32_t)_calibCoeff._B2 * ((B6 * B6) >> 12)) >> 11;
   X2 = ((int32_t)_calibCoeff._AC2 * B6) >> 11;
   X3 = X1 + X2;
   B3 = ((((int32_t)_calibCoeff._AC1 * 4 + X3) << _accuracy) + 2) / 4;

   X1 = ((int32_t)_calibCoeff._AC3 * B6) >> 13;
   X2 = ((int32_t)_calibCoeff._B1 * ((B6 * B6) >> 12)) >> 16;
   X3 = ((X1 + X2) + 2) >> 2;
   B4 = ((uint32_t)_calibCoeff._AC4 * (X3 + 32768L)) >> 15;
   B7 = (UP - B3) * (50000UL >> _accuracy);

   if (B4 == 0) return BMP180_ERROR;                                     //safety check, avoiding division by zero

   if   (B7 < 0x80000000) pressure = (B7 * 2) / B4;
   else                   pressure = (B7 / B4) * 2;

   X1 = pow((pressure >> 8), 2);
   X1 = (X1 * 3038L) >> 16;
   X2 = (-7357L * pressure) >> 16;

   return pressure = pressure + ((X1 + X2 + 3791L) >> 4);
 }

/**
 *
 */
float BarometricPressure::getPressurePascal(bool print)
{
  float pressurePascal = getPressure();
  if(pressurePascal == BMP180_ERROR)
  {
    pressurePascal = 0.f;
  }
  if(print)
  {
    Serial.print("Pressure (kilo-pascal): ");
    Serial.print((pressurePascal/1000.f),2);
    Serial.println("kilo-pascal");
  }
  return pressurePascal/1000.f;
}

/**
 *
 */
float BarometricPressure::getPressureHg(bool print)
{
  float pressurePascal = getPressure();
  if(pressurePascal == BMP180_ERROR)
  {
    pressurePascal = 0.f;
  }
  if(print)
  {
    Serial.print("Pressure (mmHg): ");
    Serial.print((pressurePascal/133.f),2);
    Serial.println("mmHg");
  }
  return pressurePascal/133.f;
}

/**
 *
 */
float BarometricPressure::getPressureBar(bool print)
{
  float pressurePascal = getPressure();
  if(pressurePascal == BMP180_ERROR)
  {
    pressurePascal = 0.f;
  }
  if(print)
  {
    Serial.print("Pressure (mbar): ");
    Serial.print((pressurePascal/100.f),2);
    Serial.println("mbar");
  }
  return pressurePascal/100.f;
}

/**
 *
 */
float BarometricPressure::getSeaLevelPressure(float altitude, bool print)
{
    float slp;
    float pressure = getPressureBar(false);
    slp = pressure / pow(1.0 - altitude/44330.0 , 5.255);
    if(print)
    {
        Serial.print("Sea Level Pressure: ");
        Serial.print(slp,2);
        Serial.println("mbar");
    }
    return slp;
}

/**
 *
 */
float BarometricPressure::getAltitude(float p0, bool print)
{
    float altitude;
    float pressure = getPressureBar(false);
    altitude = 44330.0 * (1.0 - pow((pressure/p0),(1.0/5.255)));
    if(print)
    {
        Serial.print("Altitude: ");
        Serial.print(altitude,2);
        Serial.println("meters");
    }
    return altitude;
}

/**
 *
 */
int32_t BarometricPressure::computeB5(int32_t UT)
{
  int32_t X1 = ((UT - (int32_t)_calibCoeff._AC6) * (int32_t)_calibCoeff._AC5) >> 15;
  int32_t X2 = ((int32_t)_calibCoeff._MC << 11) / (X1 + (int32_t)_calibCoeff._MD);
  return X1 + X2;
}

/**
 *
 */
void BarometricPressure::reset(void){
  write8bit(SOFT_RESET_REG, BMP180_SOFT_REST_VALUE);
}

/**
 *
 */
 uint8_t BarometricPressure::getDeviceId(void){
   if (read8bit(CHIP_ID_REG) == BMP180_CHIP_ID)
   {
      return BMP180_CHIP_ID;
   }
   else
   {
     return BMP180_ERROR;
   }
 }

/**
 *
 */
bool BarometricPressure::readCalibrationCoefficients(void)
{
  /* get the sensor calibration coefficients */
  _calibCoeff._AC1 = read16bit(AC1_REG);
  _calibCoeff._AC3 = read16bit(AC3_REG);
  _calibCoeff._AC2 = read16bit(AC2_REG);
  _calibCoeff._AC5 = read16bit(AC5_REG);
  _calibCoeff._AC4 = read16bit(AC4_REG);
  _calibCoeff._AC6 = read16bit(AC6_REG);
  _calibCoeff._B1 = read16bit(B1_REG);
  _calibCoeff._B2 = read16bit(B2_REG);
  _calibCoeff._MB = read16bit(MB_REG);
  _calibCoeff._MC = read16bit(MC_REG);
  _calibCoeff._MD = read16bit(MD_REG);
  return true;
}

/**
 *
 */
uint16_t BarometricPressure::readRawTemperature(void)
{
  /* Send the temperature measure command */
  if(write8bit(CONTROL_REG,BMP180_GET_TEMPERATURE) == false)
  {
    return BMP180_ERROR;
  }
  /* wait until measurement completion */
  delay_ms(5u);
  /* read the raw temperature value */
  return read16bit(ADC_OUT_MSB_REG);
}

/**
 *
 */
uint32_t BarometricPressure::readRawPressure(void)
{
  uint8_t regVal=0u;
  uint32_t rawPressure=0u;
  uint8_t delayMs=0u;
  switch(_accuracy)
  {
    case ULTRA_LOW_POWER:
      regVal = BMP180_GET_PRESSURE_OSS0;
      delayMs = 5u;
      break;
    case STANDARD:
      regVal = BMP180_GET_PRESSURE_OSS1;
      delayMs = 8u;
      break;
    case HIGH_RESOLUTION:
      regVal = BMP180_GET_PRESSURE_OSS2;
      delayMs = 14u;
      break;
    case ULTRA_HIGH_RESOLUTION:
      regVal = BMP180_GET_PRESSURE_OSS3;
      delayMs = 26u;
      break;
    default:
      break;
  }
  /* Send the pressure measure command */
  if(write8bit(CONTROL_REG,regVal) == false)
  {
    return BMP180_ERROR;
  }
  /* wait until measurement completion */
  delay_ms(delayMs);
  /* read pressure msb + lsb */
  if((rawPressure = read16bit(ADC_OUT_MSB_REG)) == BMP180_ERROR)
  {
    return BMP180_ERROR;
  }
  /* shift out left 8 times to store the xlsb*/
  rawPressure <<= 8u;
  /* read pressure xlsb */
  rawPressure |= read8bit(ADC_OUT_XLSB_REG);
  rawPressure >>= (8u - _accuracy);
  /* return the raw pressure value */
  return rawPressure;
}

/***********************************************************************************************
 * Platform dependent routines. Change these functions implementation based on microcontroller *
 ***********************************************************************************************/
/**
 *
 */
void BarometricPressure::i2c_init(void)
{
  Wire.begin();
  Wire.setClock(100000);
}

/**
 *
 */
uint8_t BarometricPressure::read8bit(bmp180Reg_t reg)
{
  Wire.beginTransmission(_i2cSlaveAddress);
  Wire.write(reg);
  if(Wire.endTransmission(true) != 0)
  {
    return BMP180_ERROR;
  }
  Wire.requestFrom(_i2cSlaveAddress, (uint8_t)1u, (uint8_t)1u);
  if(Wire.available() != 1u)
  {
    return BMP180_ERROR;
  }
  return Wire.read();
}

/**
 *
 */
uint16_t BarometricPressure::read16bit(bmp180Reg_t reg)
{
  uint16_t value=0u;
  Wire.beginTransmission(_i2cSlaveAddress);
  Wire.write(reg);
  if(Wire.endTransmission(true) != 0)
  {
    return BMP180_ERROR;
  }
  Wire.requestFrom(_i2cSlaveAddress, (uint8_t)2u, (uint8_t)1u);
  if(Wire.available() != 2u)
  {
    return BMP180_ERROR;
  }
  value  = Wire.read() << 8;
  value |= Wire.read();
  return value;
}

/**
 *
 */
bool BarometricPressure::write8bit(bmp180Reg_t reg, uint8_t val)
{
  Wire.beginTransmission(_i2cSlaveAddress);
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
bool BarometricPressure::writeAddress(void)
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
void BarometricPressure::delay_ms(uint16_t ms)
{
  delay(ms);
}
