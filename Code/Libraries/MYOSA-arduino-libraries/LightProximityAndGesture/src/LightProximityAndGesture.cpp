/*
  This code is developed under the MYOSA (LearnTheEasyWay) initiative of MakeSense EduTech and Pegasus Automation.
  Code has been derived from internet sources and component datasheets.
  Existing readily-available libraries would have been used "AS IS" and modified for ease of learning purpose.

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

#include "LightProximityAndGesture.h"

/**
 *
 */
LightProximityAndGesture::LightProximityAndGesture()
{
  _i2cSlaveAddress = APDS9960_I2C_ADDRESS;
}

/**
 *
 */
bool LightProximityAndGesture::ping(void)
{
    bool getConnectSts = writeAddress();
    if(!_isConnected && getConnectSts)
    {
      begin();
      
      enableAmbientLightSensor(DISABLE);
      enableProximitySensor(DISABLE);
      setProximityGain(PGAIN_2X);
      // enableGestureSensor(DISABLE);
      delay(500u);
    

    }
    _isConnected = getConnectSts;
    return getConnectSts;
}

/**
 *
 */
bool LightProximityAndGesture::begin(void)
{
  uint8_t deviceId;
  /* Read ID register and check against known values for APDS9960 */
  if( !readByte(APDS9960_ID,&deviceId) )
  {
    return false;
  }
  if( (deviceId != APDS9960_ID_1) && (deviceId != APDS9960_ID_2) && (deviceId != APDS9960_ID_3) )
  {
    return false;
  }
  /* Set ENABLE register to 0 (disable all features) */
  if( !writeByte(APDS9960_ENABLE,0u) )
  {
    return false;
  }
  /* Set default values for ambient light and proximity registers */
  if( !writeByte(APDS9960_ATIME,DEFAULT_ATIME) )
  {
    return false;
  }
  if( !writeByte(APDS9960_WTIME,DEFAULT_WTIME) )
  {
    return false;
  }
  if( !writeByte(APDS9960_PPULSE,DEFAULT_PROX_PPULSE) )
  {
    return false;
  }
  if( !writeByte(APDS9960_POFFSET_UR,DEFAULT_POFFSET_UR) )
  {
    return false;
  }
  if( !writeByte(APDS9960_POFFSET_DL,DEFAULT_POFFSET_DL) )
  {
    return false;
  }
  if( !writeByte(APDS9960_CONFIG1,DEFAULT_CONFIG1) )
  {
    return false;
  }
  if( !setLedDrive(DEFAULT_LDRIVE) )
  {
    return false;
  }
  if( !setProximityGain(DEFAULT_PGAIN) )
  {
    return false;
  }
  if( !setAmbientLightGain(DEFAULT_AGAIN) )
  {
    return false;
  }
  if( !writeByte(APDS9960_PILT,DEFAULT_PILT))
  {
    return false;
  }
  if( !writeByte(APDS9960_PIHT,DEFAULT_PIHT))
  {
    return false;
  }
  if( !setLightIntLowThreshold(DEFAULT_AILT) )
  {
    return false;
  }
  if( !setLightIntHighThreshold(DEFAULT_AIHT) )
  {
    return false;
  }
  if( !writeByte(APDS9960_PERS, DEFAULT_PERS) )
  {
    return false;
  }
  if( !writeByte(APDS9960_CONFIG2, DEFAULT_CONFIG2) )
  {
    return false;
  }
  if( !writeByte(APDS9960_CONFIG3, DEFAULT_CONFIG3) )
  {
    return false;
  }
  /* Set default values for gesture sense registers */
  if( !writeByte(APDS9960_GPENTH,DEFAULT_GPENTH) )
  {
    return false;
  }
  if( !writeByte(APDS9960_GEXTH,DEFAULT_GEXTH) )
  {
    return false;
  }
  if( !writeByte(APDS9960_GCONF1, DEFAULT_GCONF1) )
  {
    return false;
  }
  if( !setGestureGain(DEFAULT_GGAIN) )
  {
    return false;
  }
  if( !setGestureLedDrive(DEFAULT_GLDRIVE) )
  {
    return false;
  }
  if( !setGestureWaitTime(DEFAULT_GWTIME) )
  {
    return false;
  }
  if( !writeByte(APDS9960_GOFFSET_U, DEFAULT_GOFFSET) )
  {
    return false;
  }
  if( !writeByte(APDS9960_GOFFSET_D, DEFAULT_GOFFSET) )
  {
    return false;
  }
  if( !writeByte(APDS9960_GOFFSET_L, DEFAULT_GOFFSET) )
  {
    return false;
  }
  if( !writeByte(APDS9960_GOFFSET_R, DEFAULT_GOFFSET) )
  {
    return false;
  }
  if( !writeByte(APDS9960_GPULSE, DEFAULT_GPULSE) )
  {
    return false;
  }
  if( !writeByte(APDS9960_GCONF3, DEFAULT_GCONF3) )
  {
    return false;
  }
  if( !setGestureInt(DISABLE) )
  {
    return false;
  }
  _isConnected = true;
  return true;
}

/**
 *
 */
bool LightProximityAndGesture::enablePower(void)
{
  return setMode(POWER, ENABLE);
}

/**
 *
 */
bool LightProximityAndGesture::disablePower(void)
{
  return setMode(POWER, DISABLE);
}

/**
 *
 */
uint8_t LightProximityAndGesture::getMode(void)
{
  uint8_t enableVal;
  if(readByte(APDS9960_ENABLE,&enableVal) == false)
  {
    return ERROR;
  }
  return enableVal;
}

/**
 *
 */
bool LightProximityAndGesture::setMode(APDS9960_MODE_t mode, STATE_t state)
{
  uint8_t enableVal = getMode();
  if(enableVal == ERROR)
  {
    return false;
  }
  enableVal &= ~(1u << mode);
  enableVal |= (state << mode);
  if(writeByte(APDS9960_ENABLE,enableVal) == false)
  {
    return false;
  }
  return true;
}

/**
 *
 */
bool LightProximityAndGesture::enableAmbientLightSensor(STATE_t interrupt)
{
   /* Set default gain, interrupts, enable power, and enable sensor */
  if( !setAmbientLightGain(DEFAULT_AGAIN) )
  {
    return false;
  }
  if( !setAmbientLightInt(interrupt) )
  {
    return false;
  }
  if( !enablePower() )
  {
    return false;
  }
  if( !setMode(AMBIENT_LIGHT,ENABLE) )
  {
    return false;
  }
  return true;
}

/**
 *
 */
bool LightProximityAndGesture::disableAmbientLightSensor(void)
{
  if( !setAmbientLightInt(DISABLE) )
  {
    return false;
  }
  if( !setMode(AMBIENT_LIGHT,DISABLE) )
  {
    return false;
  }
  return true;
}

/**
 *
 */
bool LightProximityAndGesture::enableProximitySensor(STATE_t interrupt)
{
  /* Set default gain, LED, interrupts, enable power, and enable sensor */
  if( !setProximityGain(DEFAULT_PGAIN) )
  {
    return false;
  }
  if( !setLedDrive(DEFAULT_LDRIVE) )
  {
    return false;
  }
  if( !setProximityInt(interrupt) )
  {
    return false;
  }
  if( !enablePower() )
  {
    return false;
  }
  if( !setMode(PROXIMITY,ENABLE) )
  {
    return false;
  }
  return true;
}

/**
 *
 */
bool LightProximityAndGesture::disableProximitySensor(void)
{
  if( !setProximityInt(DISABLE) )
  {
    return false;
  }
  if( !setMode(PROXIMITY,DISABLE) )
  {
    return false;
  }
  return true;
}

/**
 *
 */
bool LightProximityAndGesture::enableGestureSensor(STATE_t interrupt)
{
  /*
    Enable gesture mode
    Set ENABLE to 0 (power off)
    Set WTIME to 0xFF
    Set AUX to LED_BOOST_300
    Enable PON, WEN, PEN, GEN in ENABLE
  */
  resetGestureParameters();
  if( !writeByte(APDS9960_WTIME,0xFFu) )
  {
    return false;
  }
  if( !writeByte(APDS9960_PPULSE,DEFAULT_GESTURE_PPULSE) )
  {
    return false;
  }
  if( !setLedBoost(LED_BOOST_300) )
  {
    return false;
  }
  if( !setGestureInt(interrupt) )
  {
    return false;
  }
  if( !setGestureMode(1u) )
  {
    return false;
  }
  if( !enablePower() )
  {
    return false;
  }
  if( !setMode(WAIT,ENABLE) )
  {
    return false;
  }
  if( !setMode(PROXIMITY,ENABLE) )
  {
    return false;
  }
  if( !setMode(GESTURE,ENABLE) )
  {
    return false;
  }
  return true;
}

/**
 *
 */
bool LightProximityAndGesture::disableGestureSensor(void)
{
  resetGestureParameters();
  if( !setGestureInt(DISABLE) )
  {
    return false;
  }
  if( !setGestureMode(0u) )
  {
    return false;
  }
  if( !setMode(GESTURE,DISABLE) )
  {
    return false;
  }
  return true;
}

/**
 *
 */
bool LightProximityAndGesture::getAmbientLightIntState(void)
{
  uint8_t enableVal;
  if(readByte(APDS9960_ENABLE,&enableVal) == false)
  {
    return false;
  }
  return ((enableVal & AIEN_EN_MSK) >> AIEN_EN_POS);
}

/**
 *
 */
bool LightProximityAndGesture::setAmbientLightInt(STATE_t intState)
{
  uint8_t enableVal;
  /* get the existing gain value from sensor */
  if(readByte(APDS9960_ENABLE,&enableVal) == false)
  {
    return false;
  }
  /* update the ALS sensor gain value */
  enableVal &= ~AIEN_EN_MSK;
  enableVal |= (((uint8_t)intState & ENABLE_MSK) << AIEN_EN_POS);
  /* write new gain value to register */
  if(writeByte(APDS9960_ENABLE,enableVal) == false)
  {
    return false;
  }
  return true;
}

/**
 *
 */
bool LightProximityAndGesture::getProximityIntState(void)
{
  uint8_t enableVal;
  if(readByte(APDS9960_ENABLE,&enableVal) == false)
  {
    return false;
  }
  return ((enableVal & PIEN_EN_MSK) >> PIEN_EN_POS);
}

/**
 *
 */
bool LightProximityAndGesture::setProximityInt(STATE_t intState)
{
  uint8_t enableVal;
  /* get the existing gain value from sensor */
  if(readByte(APDS9960_ENABLE,&enableVal) == false)
  {
    return false;
  }
  /* update the ALS sensor gain value */
  enableVal &= ~PIEN_EN_MSK;
  enableVal |= (((uint8_t)intState & ENABLE_MSK) << PIEN_EN_POS);
  /* write new gain value to register */
  if(writeByte(APDS9960_ENABLE,enableVal) == false)
  {
    return false;
  }
  return true;
}

/**
 *
 */
bool LightProximityAndGesture::getGestureIntState(void)
{
  uint8_t gconfig4;
  if(readByte(APDS9960_GCONF4,&gconfig4) == false)
  {
    return false;
  }
  return ((gconfig4 & GES_GIEN_MSK) >> GES_GIEN_POS);
}

/**
 *
 */
bool LightProximityAndGesture::setGestureInt(STATE_t intState)
{
  uint8_t gconfig4;
  /* get the existing gain value from sensor */
  if(readByte(APDS9960_GCONF4,&gconfig4) == false)
  {
    return false;
  }
  /* update the ALS sensor gain value */
  gconfig4 &= ~GES_GIEN_MSK;
  gconfig4 |= (((uint8_t)intState & GES_GCONFIG4_MSK) << GES_GIEN_POS);
  /* write new gain value to register */
  if(writeByte(APDS9960_GCONF4,gconfig4) == false)
  {
    return false;
  }
  return true;
}

/**
 *
 */
uint8_t LightProximityAndGesture::getAmbientLightGain(void)
{
  uint8_t gain;
  if(readByte(APDS9960_CONTROL,&gain) == false)
  {
    return ERROR;
  }
  return ((gain & ALS_GAIN_MSK) >> ALS_GAIN_POS);
}

/**
 *
 */
bool LightProximityAndGesture::setAmbientLightGain(uint8_t alsGain)
{
  uint8_t gain;
  /* get the existing gain value from sensor */
  if(readByte(APDS9960_CONTROL,&gain) == false)
  {
    return false;
  }
  /* update the ALS sensor gain value */
  gain &= ~ALS_GAIN_MSK;
  gain |= ((alsGain & APSD9960_GAIN_MSK) << ALS_GAIN_POS);
  /* write new gain value to register */
  if(writeByte(APDS9960_CONTROL,gain) == false)
  {
    return false;
  }
  return true;
}

/**
 *
 */
uint8_t LightProximityAndGesture::getProximityGain(void)
{
  uint8_t gain;
  if( !readByte(APDS9960_CONTROL,&gain) )
  {
    return ERROR;
  }
  return ((gain & PRX_GAIN_MSK) >> PRX_GAIN_POS);
}

/**
 *
 */
bool LightProximityAndGesture::setProximityGain(uint8_t prxGain)
{
  uint8_t gain;
  /* get the existing gain value from sensor */
  if( !readByte(APDS9960_CONTROL,&gain) )
  {
    return false;
  }
  /* update the ALS sensor gain value */
  gain &= ~PRX_GAIN_MSK;
  gain |= ((prxGain & APSD9960_GAIN_MSK) << PRX_GAIN_POS);
  /* write new gain value to register */
  if( !writeByte(APDS9960_CONTROL,gain) )
  {
    return false;
  }
  return true;
}

/**
 *
 */
uint8_t LightProximityAndGesture::getLedDrive(void)
{
  uint8_t gain;
  if(readByte(APDS9960_CONTROL,&gain) == false)
  {
    return ERROR;
  }
  return ((gain & LED_DRIVE_MSK) >> LED_DRIVE_POS);
}

/**
 *
 */
bool LightProximityAndGesture::setLedDrive(uint8_t drive)
{
  uint8_t gain;
  /* get the existing gain value from sensor */
  if(readByte(APDS9960_CONTROL,&gain) == false)
  {
    return false;
  }
  /* update the ALS sensor gain value */
  gain &= ~LED_DRIVE_MSK;
  gain |= ((drive & APSD9960_GAIN_MSK) << LED_DRIVE_POS);
  /* write new gain value to register */
  if(writeByte(APDS9960_CONTROL,gain) == false)
  {
    return false;
  }
  return true;
}

/**
 *
 */
uint8_t LightProximityAndGesture::getGestureGain(void)
{
  uint8_t gain;
  if(readByte(APDS9960_GCONF2,&gain) == false)
  {
    return ERROR;
  }
  return ((gain & GES_GAIN_MSK) >> GES_GAIN_POS);
}

/**
 *
 */
bool LightProximityAndGesture::setGestureGain(uint8_t gesGain)
{
  uint8_t gain;
  /* get the existing gain value from sensor */
  if(readByte(APDS9960_GCONF2,&gain) == false)
  {
    return false;
  }
  /* update the ALS sensor gain value */
  gain &= ~GES_GAIN_MSK;
  gain |= ((gesGain & APSD9960_GAIN_MSK) << GES_GAIN_POS);
  /* write new gain value to register */
  if(writeByte(APDS9960_GCONF2,gain) == false)
  {
    return false;
  }
  return true;
}

/**
 *
 */
uint8_t LightProximityAndGesture::getGestureLedDrive(void)
{
  uint8_t config;
  if(readByte(APDS9960_GCONF2,&config) == false)
  {
    return ERROR;
  }
  return ((config & GES_LDRIVE_MSK) >> GES_LDRIVE_POS);
}

/**
 *
 */
bool LightProximityAndGesture::setGestureLedDrive(uint8_t drive)
{
  uint8_t config;
  /* get the existing gain value from sensor */
  if(readByte(APDS9960_GCONF2,&config) == false)
  {
    return false;
  }
  /* update the ALS sensor gain value */
  config &= ~GES_LDRIVE_MSK;
  config |= ((drive & APSD9960_GAIN_MSK) << GES_LDRIVE_POS);
  /* write new gain value to register */
  if(writeByte(APDS9960_GCONF2,config) == false)
  {
    return false;
  }
  return true;
}

/**
 *
 */
uint8_t LightProximityAndGesture::getGestureWaitTime(void)
{
  uint8_t config;
  if(readByte(APDS9960_GCONF2,&config) == false)
  {
    return ERROR;
  }
  return ((config & GES_WTIME_MSK) >> GES_WTIME_POS);
}

/**
 *
 */
bool LightProximityAndGesture::setGestureWaitTime(uint8_t drive)
{
  uint8_t config;
  /* get the existing gain value from sensor */
  if(readByte(APDS9960_GCONF2,&config) == false)
  {
    return false;
  }
  /* update the ALS sensor gain value */
  config &= ~GES_WTIME_MSK;
  config |= ((drive & GES_WTIME_MSK) << GES_WTIME_POS);
  /* write new gain value to register */
  if(writeByte(APDS9960_GCONF2,config) == false)
  {
    return false;
  }
  return true;
}

/**
 *
 */
uint8_t LightProximityAndGesture::getGestureMode(void)
{
  uint8_t gconfig4;
  if(readByte(APDS9960_GCONF4,&gconfig4) == false)
  {
    return ERROR;
  }
  return ((gconfig4 & GES_GMODE_MSK) >> GES_GMODE_POS);
}

/**
 *
 */
bool LightProximityAndGesture::setGestureMode(uint8_t mode)
{
  uint8_t gconfig4;
  /* get the existing gain value from sensor */
  if(readByte(APDS9960_GCONF4,&gconfig4) == false)
  {
    return false;
  }
  /* update the ALS sensor gain value */
  gconfig4 &= ~GES_GMODE_MSK;
  gconfig4 |= (((uint8_t)mode & GES_GMODE_MSK) << GES_GMODE_POS);
  /* write new gain value to register */
  if(writeByte(APDS9960_GCONF4,gconfig4) == false)
  {
    return false;
  }
  return true;
}

/**
 *
 */
bool LightProximityAndGesture::getLightIntLowThreshold(uint16_t *threshold)
{
  uint8_t data[2];
  if(readByte(APDS9960_AILTL,&data[0u]) == false)
  {
    return false;
  }
  if(readByte(APDS9960_AILTH,&data[1u]) == false)
  {
    return false;
  }
  /* update the threshold */
  *threshold = (((uint16_t)data[1u] << 8u) | data[0u]);
  return true;
}

/**
 *
 */
bool LightProximityAndGesture::setLightIntLowThreshold(uint16_t threshold)
{
  uint8_t data = (uint8_t)(threshold & 0xFFu);
  if(writeByte(APDS9960_AILTL,data) == false)
  {
    return false;
  }
  data = (uint8_t)((threshold >> 8u) & 0xFFu);
  if(writeByte(APDS9960_AILTH,data) == false)
  {
    return false;
  }
  return true;
}

/**
 *
 */
bool LightProximityAndGesture::getLightIntHighThreshold(uint16_t *threshold)
{
  uint8_t data[2];
  if(readByte(APDS9960_AIHTL,&data[0u]) == false)
  {
    return false;
  }
  if(readByte(APDS9960_AIHTH,&data[1u]) == false)
  {
    return false;
  }
  /* update the threshold */
  *threshold = (((uint16_t)data[1u] << 8u) | data[0u]);
  return true;
}

/**
 *
 */
bool LightProximityAndGesture::setLightIntHighThreshold(uint16_t threshold)
{
  uint8_t data = (uint8_t)(threshold & 0xFFu);
  if(writeByte(APDS9960_AIHTL,data) == false)
  {
    return false;
  }
  data = (uint8_t)((threshold >> 8u) & 0xFFu);
  if(writeByte(APDS9960_AIHTH,data) == false)
  {
    return false;
  }
  return true;
}

/**
 *
 */
bool LightProximityAndGesture::getProximityIntLowThreshold(uint8_t *threshold)
{
  *threshold = 0u;
  return readByte(APDS9960_PILT,threshold);
}

/**
 *
 */
bool LightProximityAndGesture::setProximityIntLowThreshold(uint8_t threshold)
{
  return writeByte(APDS9960_PILT,threshold);
}

/**
 *
 */
bool LightProximityAndGesture::getProximityIntHighThreshold(uint8_t *threshold)
{
  *threshold = 0u;
  return readByte(APDS9960_PIHT,threshold);
}

/**
 *
 */
bool LightProximityAndGesture::setProximityIntHighThreshold(uint8_t threshold)
{
  return writeByte(APDS9960_PIHT,threshold);
}

/**
 *
 */
void LightProximityAndGesture::resetGestureParameters(void)
{
  _gesture_data.index = 0;
  _gesture_data.total_gestures = 0;

  _gesture_ud_delta = 0;
  _gesture_lr_delta = 0;

  _gesture_ud_count = 0;
  _gesture_lr_count = 0;

  _gesture_near_count = 0;
  _gesture_far_count = 0;

  _gesture_state = 0;
  _gesture_motion = DIR_NONE;
}

/**
 *
 */
bool LightProximityAndGesture::processGestureData(void)
{
  uint8_t u_first = 0;
  uint8_t d_first = 0;
  uint8_t l_first = 0;
  uint8_t r_first = 0;
  uint8_t u_last = 0;
  uint8_t d_last = 0;
  uint8_t l_last = 0;
  uint8_t r_last = 0;
  int ud_ratio_first;
  int lr_ratio_first;
  int ud_ratio_last;
  int lr_ratio_last;
  int ud_delta;
  int lr_delta;
  int i;

  /* If we have less than 4 total gestures, that's not enough */
  if( _gesture_data.total_gestures <= 4 )
  {
    return false;
  }

  /* Check to make sure our data isn't out of bounds */
  if( (_gesture_data.total_gestures <= 32) && \
      (_gesture_data.total_gestures > 0) )
  {
    /* Find the first value in U/D/L/R above the threshold */
    for( i = 0; i < _gesture_data.total_gestures; i++ )
    {
      if( (_gesture_data.u_data[i] > GESTURE_THRESHOLD_OUT) &&
          (_gesture_data.d_data[i] > GESTURE_THRESHOLD_OUT) &&
          (_gesture_data.l_data[i] > GESTURE_THRESHOLD_OUT) &&
          (_gesture_data.r_data[i] > GESTURE_THRESHOLD_OUT) )
      {
          u_first = _gesture_data.u_data[i];
          d_first = _gesture_data.d_data[i];
          l_first = _gesture_data.l_data[i];
          r_first = _gesture_data.r_data[i];
          break;
      }
    }
    /* If one of the _first values is 0, then there is no good data */
    if( (u_first == 0) || (d_first == 0) || \
        (l_first == 0) || (r_first == 0) ) {
              return false;
    }
    /* Find the last value in U/D/L/R above the threshold */
    for( i = _gesture_data.total_gestures - 1; i >= 0; i-- )
    {
      if( (_gesture_data.u_data[i] > GESTURE_THRESHOLD_OUT) &&
          (_gesture_data.d_data[i] > GESTURE_THRESHOLD_OUT) &&
          (_gesture_data.l_data[i] > GESTURE_THRESHOLD_OUT) &&
          (_gesture_data.r_data[i] > GESTURE_THRESHOLD_OUT) )
      {
          u_last = _gesture_data.u_data[i];
          d_last = _gesture_data.d_data[i];
          l_last = _gesture_data.l_data[i];
          r_last = _gesture_data.r_data[i];
          break;
      }
    }
  }
  /* Calculate the first vs. last ratio of up/down and left/right */
  ud_ratio_first = ((u_first - d_first) * 100) / (u_first + d_first);
  lr_ratio_first = ((l_first - r_first) * 100) / (l_first + r_first);
  ud_ratio_last = ((u_last - d_last) * 100) / (u_last + d_last);
  lr_ratio_last = ((l_last - r_last) * 100) / (l_last + r_last);

  /* Determine the difference between the first and last ratios */
  ud_delta = ud_ratio_last - ud_ratio_first;
  lr_delta = lr_ratio_last - lr_ratio_first;

  /* Accumulate the UD and LR delta values */
  _gesture_ud_delta += ud_delta;
  _gesture_lr_delta += lr_delta;

  /* Determine U/D gesture */
  if( _gesture_ud_delta >= GESTURE_SENSITIVITY_1 ) {
      _gesture_ud_count = 1;
  } else if( _gesture_ud_delta <= -GESTURE_SENSITIVITY_1 ) {
      _gesture_ud_count = -1;
  } else {
      _gesture_ud_count = 0;
  }

  /* Determine L/R gesture */
  if( _gesture_lr_delta >= GESTURE_SENSITIVITY_1 ) {
      _gesture_lr_count = 1;
  } else if( _gesture_lr_delta <= -GESTURE_SENSITIVITY_1 ) {
      _gesture_lr_count = -1;
  } else {
      _gesture_lr_count = 0;
  }

  /* Determine Near/Far gesture */
  if( (_gesture_ud_count == 0) && (_gesture_lr_count == 0) ) {
    if( (abs(ud_delta) < GESTURE_SENSITIVITY_2) && \
        (abs(lr_delta) < GESTURE_SENSITIVITY_2) )
    {
      if( (ud_delta == 0) && (lr_delta == 0) ) {
          _gesture_near_count++;
      } else if( (ud_delta != 0) || (lr_delta != 0) ) {
          _gesture_far_count++;
      }

      if( (_gesture_near_count >= 10) && (_gesture_far_count >= 2) )
      {
        if( (ud_delta == 0) && (lr_delta == 0) ) {
            _gesture_state = NEAR_STATE;
        } else if( (ud_delta != 0) && (lr_delta != 0) ) {
            _gesture_state = FAR_STATE;
        }
        return true;
      }
    }
  } else {
    if( (abs(ud_delta) < GESTURE_SENSITIVITY_2) && \
        (abs(lr_delta) < GESTURE_SENSITIVITY_2) ) {
        if( (ud_delta == 0) && (lr_delta == 0) ) {
            _gesture_near_count++;
        }

        if( _gesture_near_count >= 10 ) {
            _gesture_ud_count = 0;
            _gesture_lr_count = 0;
            _gesture_ud_delta = 0;
            _gesture_lr_delta = 0;
        }
    }
  }
  return false;
}

/**
 *
 */
bool LightProximityAndGesture::decodeGesture(void)
{
  /* Return if near or far event is detected */
   if( _gesture_state == NEAR_STATE ) {
       _gesture_motion = DIR_NEAR;
       return true;
   } else if ( _gesture_state == FAR_STATE ) {
       _gesture_motion = DIR_FAR;
       return true;
   }

   /* Determine swipe direction */
   if( (_gesture_ud_count == -1) && (_gesture_lr_count == 0) ) {
       _gesture_motion = DIR_UP;
   } else if( (_gesture_ud_count == 1) && (_gesture_lr_count == 0) ) {
       _gesture_motion = DIR_DOWN;
   } else if( (_gesture_ud_count == 0) && (_gesture_lr_count == 1) ) {
       _gesture_motion = DIR_RIGHT;
   } else if( (_gesture_ud_count == 0) && (_gesture_lr_count == -1) ) {
       _gesture_motion = DIR_LEFT;
   } else if( (_gesture_ud_count == -1) && (_gesture_lr_count == 1) ) {
       if( abs(_gesture_ud_delta) > abs(_gesture_lr_delta) ) {
           _gesture_motion = DIR_UP;
       } else {
           _gesture_motion = DIR_RIGHT;
       }
   } else if( (_gesture_ud_count == 1) && (_gesture_lr_count == -1) ) {
       if( abs(_gesture_ud_delta) > abs(_gesture_lr_delta) ) {
           _gesture_motion = DIR_DOWN;
       } else {
           _gesture_motion = DIR_LEFT;
       }
   } else if( (_gesture_ud_count == -1) && (_gesture_lr_count == -1) ) {
       if( abs(_gesture_ud_delta) > abs(_gesture_lr_delta) ) {
           _gesture_motion = DIR_UP;
       } else {
           _gesture_motion = DIR_LEFT;
       }
   } else if( (_gesture_ud_count == 1) && (_gesture_lr_count == 1) ) {
       if( abs(_gesture_ud_delta) > abs(_gesture_lr_delta) ) {
           _gesture_motion = DIR_DOWN;
       } else {
           _gesture_motion = DIR_RIGHT;
       }
   } else {
       return false;
   }
   return true;
}

/**
 *
 */
uint8_t LightProximityAndGesture::getLedBoost(void)
{
  uint8_t config2;
  if( !readByte(APDS9960_CONFIG2, &config2) )
  {
    return ERROR;
  }
  return ((config2 & CFG2_LED_BOOST_MSK) >> CFG2_LED_BOOST_POS);
}

/**
 *
 */
bool LightProximityAndGesture::setLedBoost(uint8_t boost)
{
  uint8_t config2;
  if( !readByte(APDS9960_CONFIG2, &config2) )
  {
    return false;
  }
  config2 &= ~CFG2_LED_BOOST_MSK;
  config2 |= ((boost & 0x03u) << CFG2_LED_BOOST_POS);
  if( !writeByte(APDS9960_CONFIG2, config2) )
  {
    return false;
  }
  return true;
}

/**
 *
 */
bool LightProximityAndGesture::clearAmbientLightInt(void)
{
  if( !writeByte(APDS9960_AICLEAR) )
  {
    return false;
  }
  return true;
}

/**
 *
 */
bool LightProximityAndGesture::clearProximityInt(void)
{
  if( !writeByte(APDS9960_PICLEAR))
  {
    return false;
  }
  return true;
}

/**
 *
 */
bool LightProximityAndGesture::readAmbientLight(uint16_t *val)
{
  uint8_t data[2u];
  if (!readByte(APDS9960_CDATAL,&data[0u]))
  {
    return false;
  }
  if (!readByte(APDS9960_CDATAH,&data[1u]))
  {
    return false;
  }
  *val = (((uint16_t)data[1u] << 8u)|data[0]);
  return true;
}

/**
 *
 */
bool LightProximityAndGesture::readRedLight(uint16_t *val)
{
  uint8_t data[2u];
  if (!readByte(APDS9960_RDATAL,&data[0u]))
  {
    return false;
  }
  if (!readByte(APDS9960_RDATAH,&data[1u]))
  {
    return false;
  }
  *val = (((uint16_t)data[1u] << 8u)|data[0]);
  return true;
}

/**
 *
 */
bool LightProximityAndGesture::readGreenLight(uint16_t *val)
{
  uint8_t data[2u];
  if (!readByte(APDS9960_GDATAL,&data[0u]))
  {
    return false;
  }
  if (!readByte(APDS9960_GDATAH,&data[1u]))
  {
    return false;
  }
  *val = (((uint16_t)data[1u] << 8u)|data[0]);
  return true;
}

/**
 *
 */
bool LightProximityAndGesture::readBlueLight(uint16_t *val)
{
  uint8_t data[2u];
  if (!readByte(APDS9960_BDATAL,&data[0u]))
  {
    return false;
  }
  if (!readByte(APDS9960_BDATAH,&data[1u]))
  {
    return false;
  }
  *val = (((uint16_t)data[1u] << 8u)|data[0]);
  return true;
}

/**
 *
 */
bool LightProximityAndGesture::readProximity(uint8_t *val)
{
  return readByte(APDS9960_PDATA,val);
}

/**
 *
 */
bool LightProximityAndGesture::isGestureAvailable(void)
{
  uint8_t gsts;
  if (!readByte(APDS9960_GSTATUS,&gsts))
  {
    return false;
  }
  return (bool)((gsts&GSTS_GVALID_MSK) >> GSTS_GVALID_POS);
}

/**
 *
 */
int LightProximityAndGesture::readGesture(void)
{
  uint8_t fifo_level = 0;
  uint8_t fifo_data[128];
  uint8_t gstatus;
  int bytes_read = 0;
  int motion;
  int i;
  int timeCnt = 0;

  /* Make sure that power and gesture is on and data is valid */
  if( !isGestureAvailable() || !(getMode() & (GEN_EN_MSK|PON_EN_MSK)) ) {
      return DIR_NONE;
  }

  /* Keep looping as long as gesture data is valid */
  for(;;)
  {
    /* Wait some time to collect next batch of FIFO data */
    delay_ms(FIFO_PAUSE_TIME);

    /* increment timeout count */
    timeCnt++;

    /* Get the contents of the STATUS register. Is data still valid? */
    if( !readByte(APDS9960_GSTATUS, &gstatus) ) {
        return ERROR;
    }

    /* If we have valid data, read in FIFO */
    if( ((gstatus & GSTS_GVALID_MSK) == GSTS_GVALID_MSK ) && (timeCnt <= 10u)) {

        /* Read the current FIFO level */
        if( !readByte(APDS9960_GFLVL, &fifo_level) ) {
            return ERROR;
        }

        /* If there's stuff in the FIFO, read it into our data block */
        if( fifo_level > 0) {
            bytes_read = readMultiBytes(APDS9960_GFIFO_U,
                                        (fifo_level * 4),
                                        (uint8_t*)fifo_data);
            if( bytes_read == -1 ) {
                return ERROR;
            }

            /* If at least 1 set of data, sort the data into U/D/L/R */
            if( bytes_read >= 4 ) {
                for( i = 0; i < bytes_read; i += 4 ) {
                    _gesture_data.u_data[_gesture_data.index] = \
                                                        fifo_data[i + 0];
                    _gesture_data.d_data[_gesture_data.index] = \
                                                        fifo_data[i + 1];
                    _gesture_data.l_data[_gesture_data.index] = \
                                                        fifo_data[i + 2];
                    _gesture_data.r_data[_gesture_data.index] = \
                                                        fifo_data[i + 3];
                    _gesture_data.index++;
                    _gesture_data.total_gestures++;
                }

                /* Filter and process gesture data. Decode near/far state */
                if( processGestureData() ) {
                    if( decodeGesture() ) {
                        //***TODO: U-Turn Gestures
                    }
                }

                /* Reset data */
                _gesture_data.index = 0;
                _gesture_data.total_gestures = 0;
            }
        }
    } else {
        if(timeCnt >= 10u)
        {
            motion = TIMEOUT;
        }
        else
        {
            /* Determine best guessed gesture and clean up */
            delay_ms(FIFO_PAUSE_TIME);
            decodeGesture();
            motion = _gesture_motion;
            resetGestureParameters();
        }
        return motion;
    }
  }
}

/**
 *
 */
uint16_t *LightProximityAndGesture::getRGBProportion(bool print)
{
  static uint16_t color[3u];
  color[0u] = color[1u] = color[2u] = 0u;
  if(readRedLight(&color[0u]))
  {
      if(readGreenLight(&color[1u]))
      {
          if(readBlueLight(&color[2u]))
          {
              if(print)
              {
                  Serial.print("Red:");
                  Serial.print(color[0u]);
                  Serial.println("%");
                  Serial.print("Green:");
                  Serial.print(color[1u]);
                  Serial.println("%");
                  Serial.print("Blue:");
                  Serial.print(color[2u]);
                  Serial.println("%");
              }
          }
      }
  }
  return color;
}

/**
 *
 */
uint16_t LightProximityAndGesture::getAmbientLight(bool print)
{
    uint16_t ambient_light;
    if(readAmbientLight(&ambient_light))
    {
        if(print)
        {
            Serial.print("Ambient Light: ");
            Serial.print(ambient_light);
            Serial.println("Lux");
        }
        return ambient_light;
    }
    return 0u;
}

/**
 *
 */
uint16_t LightProximityAndGesture::getRedProportion(void)
{
    uint16_t red_light = 0;
    if(readRedLight(&red_light) == false)
    {
        return 0u;
    }
    return red_light;
}

/**
 *
 */
uint16_t LightProximityAndGesture::getGreenProportion(void)
{
    uint16_t green_light = 0;
    if(readGreenLight(&green_light) == false)
    {
        return 0u;
    }
    return green_light;
}

/**
 *
 */
uint16_t LightProximityAndGesture::getBlueProportion(void)
{
    uint16_t blue_light = 0;
    if(readBlueLight(&blue_light) == false)
    {
        return 0u;
    }
    return blue_light;
}

/**
 *
 */
float LightProximityAndGesture::getProximity(bool print)
{
    uint8_t proximity_data = 0;
    if(readProximity(&proximity_data))
    {
        if(print)
        {
            Serial.print("Proximity: ");
            Serial.print((float)proximity_data,2);
            Serial.println();
        }
        return (float)proximity_data;
    }
    return 0.0f;
}

/**
 *
 */
char *LightProximityAndGesture::getGesture(bool print)
{
    char *gesture = (char *)"NONE";
    if(isGestureAvailable())
    {
      Serial.print("Gesture: ");
      switch(readGesture())
      {
        case DIR_UP:
          gesture = (char *)"UP";
          Serial.println("UP");
          break;
        case DIR_DOWN:
          gesture = (char *)"DOWN";
          Serial.println("DOWN");
          break;
        case DIR_LEFT:
          gesture = (char *)"LEFT";
          Serial.println("LEFT");
          break;
        case DIR_RIGHT:
          gesture = (char *)"RIGHT";
          Serial.println("RIGHT");
          break;
        case DIR_NEAR:
          gesture = (char *)"NEAR";
          Serial.println("NEAR");
          break;
        case DIR_FAR:
          gesture = (char *)"FAR";
          Serial.println("FAR");
          break;
        case TIMEOUT:
          gesture = (char *)"TIMEOUT";
          Serial.println("TIMEOUT");
          break;
        default:
          gesture = (char *)"NONE";
          Serial.println("NONE");
          break;
      }
    }
    return gesture;
}

/***********************************************************************************************
* Platform dependent routines. Change these functions implementation based on microcontroller *
***********************************************************************************************/
/**
 *
 */
void LightProximityAndGesture::i2c_init(void)
{
  Wire.begin();
  Wire.setClock(100000);
}

/**
 *
 */
bool LightProximityAndGesture::readByte(uint8_t reg, uint8_t *in)
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
int8_t LightProximityAndGesture::readMultiBytes(uint8_t reg, uint8_t length, uint8_t *in)
{
  int8_t nData = 0u;
  Wire.beginTransmission((uint8_t)_i2cSlaveAddress);
  Wire.write(reg);
  if(Wire.endTransmission(true) != 0)
  {
    return -1;
  }
  Wire.requestFrom((uint8_t)_i2cSlaveAddress, length, 1u);
  while(Wire.available())
  {
    if(nData >= length)
    {
      return -1;
    }
    in[nData] = Wire.read();
    nData++;
  }
  return nData;
}

/**
 *
 */
bool LightProximityAndGesture::readMultiBytes(uint8_t length, uint8_t *in)
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
bool LightProximityAndGesture::writeByte(uint8_t reg)
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
bool LightProximityAndGesture::writeByte(uint8_t reg, uint8_t val)
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
bool LightProximityAndGesture::writeAddress(void)
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
void LightProximityAndGesture::delay_ms(uint16_t ms)
{
  delay(ms);
}
