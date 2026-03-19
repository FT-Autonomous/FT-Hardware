/*
  This code is developed under the MYOSA (LearnTheEasyWay) initiative of MakeSense EduTech and Pegasus Automation.
  Code has been derived from internet sources and component datasheets.
  Existing readily-available libraries would have been used "AS IS" and modified for ease of learning purpose.
  
  Synopsis of Accelerometer and Gyroscope
  MYOSA Platform consists of an Accelerometer and Gyroscope Board. It is equiped with GY521/MPU6050 IC.
  MPU6050 provides a general X/Y/Z direction (3-axis) accelerometer and gyroscope.
  I2C Address of the board = 0x69.
  Detailed Information about Accelerometer And Gyroscope board Library and usage is provided in the link below.
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

#include "AccelAndGyro.h"

/**
 *
 */
AccelAndGyro::AccelAndGyro(uint8_t i2c_add)
{
	_i2cSlaveAddress	= i2c_add;
	_isConnected		= false;
	/* 1g = 9.80665 m/s^2 */
	/* Update the Accelerometer and Gyrometer scale factors */
	for(uint32_t fsr_sel=0u; fsr_sel < 4u; fsr_sel++)
	{
		_gyroScale[fsr_sel] = (250.f*(float)(1u << (fsr_sel+1u)))/32768.f;
		_accelScale[fsr_sel] = (2.f*9.80665f*(float)(1u << (fsr_sel+1u)))/32768.f;
	}
}

/**
 *
 */
bool AccelAndGyro::begin(bool calibrate)
{
	if(setClkSource(MPU6050_CLOCK_PLL_XGYRO) == false)
	{
		return false;
	}
	if(setFullScaleGyroRange(MPU_GYRO_CONFIG_FS_SEL_250) == false)
	{
		return false;
	}
	if(setFullScaleAccelRange(MPU_ACCEL_CONFIG_FS_SEL_2g) == false)
	{
		return false;
	}
	if(setSleep(false) == false)
	{
		return false;
	}
	if(setIntZeroMotionEnabled(false) == false)
	{
		return false;
	}
	if(setIntMotionEnabled(true) == false)
	{
		return false;
	}
	if(setMotionDetectionThreshold(2) == false)
	{
		return false;
	}
	if(setZeroMotionDetectionThreshold(2) == false)
	{
		return false;
	}
	if(setMotionDetectionDuration(40) == false)
	{
		return false;
	}
	if(setZeroMotionDetectionDuration(1) == false)
	{
		return false;
	}
	/* Calibrate the sensor if required */
	if(calibrate)
	{
		if(accelGyroCalibrate() == false)
		{
			return false;
		}
	}
	_isConnected = true;
	return true;
}

bool AccelAndGyro::accelGyroCalibrate(void)
{
	float sumAx=0.f,sumAy=0.f,sumAz=0.f;
	float sumGx=0.f,sumGy=0.f,sumGz=0.f;
	getAccelX(false);
	getAccelY(false);
	getAccelZ(false);
	getGyroX(false);
	getGyroY(false);
	getGyroZ(false);
	for(uint8_t nReading=0u; nReading < CALIBRATION_READINGS; nReading++)
	{
		sumAx += getAccelX(false);
		sumAy += getAccelY(false);
		sumAz += getAccelZ(false);
		sumGx += getGyroX(false);
		sumGy += getGyroY(false);
		sumGz += getGyroZ(false);
		delay_ms(20);
	}
	float meanAx, meanAy, meanAz;
	float meanGx, meanGy, meanGz;
	meanAx = sumAx/CALIBRATION_READINGS;
	meanAy = sumAy/CALIBRATION_READINGS;
	meanAz = sumAz/CALIBRATION_READINGS;
	meanGx = sumGx/CALIBRATION_READINGS;
	meanGy = sumGy/CALIBRATION_READINGS;
	meanGz = sumGz/CALIBRATION_READINGS;
	Serial.println("Calibrate values");
	Serial.print("meanAx:");
	Serial.println(meanAx,2);
	Serial.print("meanAy:");
	Serial.println(meanAy,2);
	Serial.print("meanAx:");
	Serial.println(meanAz,2);
	Serial.print("meanAz:");
	Serial.println(meanGx,2);
	Serial.print("meanGx:");
	Serial.println(meanGy,2);
	Serial.print("meanGz:");
	Serial.println(meanGz,2);
	return true;
}

/**
 *
 */
bool AccelAndGyro::ping(void)
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
uint8_t AccelAndGyro::getDeviceId(void)
{
	uint8_t deviceId;
	if(readByte(MPU6050_WHO_AM_I_REG, &deviceId) == false)
	{
		deviceId = 0U;
	}
	return (deviceId & MPU_WHO_AM_I_MSK);
}

/**
 *
 */
bool AccelAndGyro::reset(void)
{
	uint8_t pwrMgmt1Val;
	if(readByte(MPU6050_PWR_MGMT_1_REG,&pwrMgmt1Val) == false)
	{
		return false;
	}
	pwrMgmt1Val |= MPU_PWR_MGMT_1_DEVICE_RESET_MSK;
	return writeByte(MPU6050_PWR_MGMT_1_REG,pwrMgmt1Val);
}

/**
 *
 */
bool AccelAndGyro::resetGyroPath(void)
{
	uint8_t signalPath;
	if(readByte(MPU6050_SIGNAL_PATH_RESET_REG,&signalPath) == false)
	{
		return false;
	}
	signalPath |= MPU_SIGNAL_PATH_GYRO_RESET_MSK;
	return writeByte(MPU6050_PWR_MGMT_1_REG,signalPath);
}

/**
 *
 */
bool AccelAndGyro::resetAccelPath(void)
{
	uint8_t signalPath;
	if(readByte(MPU6050_SIGNAL_PATH_RESET_REG,&signalPath) == false)
	{
		return false;
	}
	signalPath |= MPU_SIGNAL_PATH_ACCEL_RESET_MSK;
	return writeByte(MPU6050_PWR_MGMT_1_REG,signalPath);
}

/**
 *
 */
bool AccelAndGyro::resetTempPath(void)
{
	uint8_t signalPath;
	if(readByte(MPU6050_SIGNAL_PATH_RESET_REG,&signalPath) == false)
	{
		return false;
	}
	signalPath |= MPU_SIGNAL_PATH_TEMP_RESET_MSK;
	return writeByte(MPU6050_PWR_MGMT_1_REG,signalPath);
}

/**
 *
 */
bool AccelAndGyro::setFullScaleGyroRange(uint8_t range)
{
	uint8_t gyroConfig;
	if(readByte(MPU6050_GYRO_CONFIG_REG,&gyroConfig) == false)
	{
		return false;
	}
	gyroConfig &= ~MPU_GYRO_CONFIG_FS_SEL_MASK;
	gyroConfig |= (range << MPU_GYRO_CONFIG_FS_SEL_POS);
	return writeByte(MPU6050_GYRO_CONFIG_REG,gyroConfig);
}

/**
 *
 */
uint8_t AccelAndGyro::getFullScaleGyroRange(void)
{
	uint8_t gyroConfig;
	uint8_t range;
	if(readByte(MPU6050_GYRO_CONFIG_REG,&gyroConfig) == false)
	{
		return 0x0Fu;
	}
	range = (gyroConfig & MPU_GYRO_CONFIG_FS_SEL_MASK)>>MPU_GYRO_CONFIG_FS_SEL_POS;
	return range&0x0Fu;
}

/**
 *
 */
bool AccelAndGyro::setFullScaleAccelRange(uint8_t range)
{
	uint8_t accelConfig;
	if(readByte(MPU6050_ACCEL_CONFIG_REG,&accelConfig) == false)
	{
		return false;
	}
	accelConfig &= ~MPU_ACCEL_CONFIG_FS_SEL_MASK;
	accelConfig |= (range << MPU_ACCEL_CONFIG_FS_SEL_POS);
	return writeByte(MPU6050_ACCEL_CONFIG_REG,accelConfig);
}

/**
 *
 */
uint8_t AccelAndGyro::getFullScaleAccelRange(void)
{
	uint8_t accelConfig;
	uint8_t range;
	if(readByte(MPU6050_ACCEL_CONFIG_REG,&accelConfig) == false)
	{
		return 0x0Fu;
	}
	range = (accelConfig & MPU_ACCEL_CONFIG_FS_SEL_MASK)>>MPU_ACCEL_CONFIG_FS_SEL_POS;
	return range&0x0Fu;
}

/**
 *
 */
bool AccelAndGyro::setSleep(bool enable)
{
	uint8_t pwrMgmt1Val;
	if(readByte(MPU6050_PWR_MGMT_1_REG,&pwrMgmt1Val) == false)
	{
		return false;
	}
	pwrMgmt1Val &= ~MPU_PWR_MGMT_1_SLEEP_MSK;
	if(enable)
	{
		pwrMgmt1Val |= MPU_PWR_MGMT_1_SLEEP_MSK;
	}
	return writeByte(MPU6050_PWR_MGMT_1_REG,pwrMgmt1Val);
}

/**
 *
 */
bool AccelAndGyro::getSleepSts(void)
{
	uint8_t pwrMgmt1Val;
	uint8_t sleepState;
	if(readByte(MPU6050_PWR_MGMT_1_REG,&pwrMgmt1Val) == false)
	{
		return false;
	}
	sleepState = (pwrMgmt1Val & MPU_PWR_MGMT_1_SLEEP_MSK)>>MPU_PWR_MGMT_1_SLEEP_POS;
	return (bool)sleepState;
}

/**
 *
 */
bool AccelAndGyro::setCycleMode(bool enable)
{
	uint8_t pwrMgmt1Val;
	if(readByte(MPU6050_PWR_MGMT_1_REG,&pwrMgmt1Val) == false)
	{
		return false;
	}
	pwrMgmt1Val &= ~MPU_PWR_MGMT_1_CYCLE_MSK;
	if(enable)
	{
		pwrMgmt1Val |= MPU_PWR_MGMT_1_CYCLE_MSK;
	}
	return writeByte(MPU6050_PWR_MGMT_1_REG,pwrMgmt1Val);
}

/**
 *
 */
bool AccelAndGyro::getCycleMode(void)
{
	uint8_t pwrMgmt1Val;
	uint8_t cycleMode;
	if(readByte(MPU6050_PWR_MGMT_1_REG,&pwrMgmt1Val) == false)
	{
		return false;
	}
	cycleMode = (pwrMgmt1Val & MPU_PWR_MGMT_1_CYCLE_MSK)>>MPU_PWR_MGMT_1_CYCLE_POS;
	return (bool)cycleMode;
}

/**
 *
 */
bool AccelAndGyro::setTempSensorDisable(bool enable)
{
	uint8_t pwrMgmt1Val;
	if(readByte(MPU6050_PWR_MGMT_1_REG,&pwrMgmt1Val) == false)
	{
		return false;
	}
	pwrMgmt1Val &= ~MPU_PWR_MGMT_1_TEMP_DIS_MSK;
	if(enable)
	{
		pwrMgmt1Val |= MPU_PWR_MGMT_1_TEMP_DIS_MSK;
	}
	return writeByte(MPU6050_PWR_MGMT_1_REG,pwrMgmt1Val);
}

/**
 *
 */
bool AccelAndGyro::getTempSensorDisableSts(void)
{
	uint8_t pwrMgmt1Val;
	uint8_t cycleMode;
	if(readByte(MPU6050_PWR_MGMT_1_REG,&pwrMgmt1Val) == false)
	{
		return false;
	}
	cycleMode = (pwrMgmt1Val & MPU_PWR_MGMT_1_TEMP_DIS_MSK)>>MPU_PWR_MGMT_1_TEMP_DIS_POS;
	return (bool)cycleMode;
}

/**
 *
 */
bool AccelAndGyro::setClkSource(uint8_t src)
{
	uint8_t pwrMgmt1Val;
	if(readByte(MPU6050_PWR_MGMT_1_REG,&pwrMgmt1Val) == false)
	{
		return false;
	}
	pwrMgmt1Val &= ~MPU_PWR_MGMT_1_CLKSEL_MSK;
	pwrMgmt1Val |= (src << MPU_PWR_MGMT_1_CLKSEL_POS);
	return writeByte(MPU6050_PWR_MGMT_1_REG,pwrMgmt1Val);
}

/**
 *
 */
uint8_t AccelAndGyro::getClkSource(void)
{
	uint8_t pwrMgmt1Val;
	uint8_t clkSrc;
	if(readByte(MPU6050_PWR_MGMT_1_REG,&pwrMgmt1Val) == false)
	{
		return 0u;
	}
	clkSrc = (pwrMgmt1Val & MPU_PWR_MGMT_1_CLKSEL_MSK)>>MPU_PWR_MGMT_1_CLKSEL_POS;
	return clkSrc;
}

/**
 *
 */
bool AccelAndGyro::setWakeFrequency(uint8_t frequency)
{
	uint8_t pwrMgmt2Val;
	if(readByte(MPU6050_PWR_MGMT_2_REG,&pwrMgmt2Val) == false)
	{
		return false;
	}
	pwrMgmt2Val &= ~MPU_PWR_MGMT_2_LP_WAKE_CTRL_MSK;
	pwrMgmt2Val |= (frequency << MPU_PWR_MGMT_2_LP_WAKE_CTRL_POS);
	return writeByte(MPU6050_PWR_MGMT_2_REG,pwrMgmt2Val);
}

/**
 *
 */
uint8_t AccelAndGyro::getWakeFrequency(void)
{
	uint8_t pwrMgmt2Val;
	uint8_t frequency;
	if(readByte(MPU6050_PWR_MGMT_2_REG,&pwrMgmt2Val) == false)
	{
		return 0u;
	}
	frequency = (pwrMgmt2Val & MPU_PWR_MGMT_2_LP_WAKE_CTRL_MSK)>>MPU_PWR_MGMT_2_LP_WAKE_CTRL_POS;
	return frequency;
}

/**
 *
 */
bool AccelAndGyro::setStandbyXAccel(bool enable)
{
	uint8_t pwrMgmt2Val;
	if(readByte(MPU6050_PWR_MGMT_2_REG,&pwrMgmt2Val) == false)
	{
		return false;
	}
	pwrMgmt2Val &= ~MPU_PWR_MGMT_2_LP_STBY_XA_MSK;
	if(enable)
	{
		pwrMgmt2Val |= MPU_PWR_MGMT_2_LP_STBY_XA_MSK;
	}
	return writeByte(MPU6050_PWR_MGMT_2_REG,pwrMgmt2Val);
}

/**
 *
 */
bool AccelAndGyro::getStandbyXAccelSts(void)
{
	uint8_t pwrMgmt2Val;
	uint8_t standySts;
	if(readByte(MPU6050_PWR_MGMT_2_REG,&pwrMgmt2Val) == false)
	{
		return false;
	}
	standySts = (pwrMgmt2Val & MPU_PWR_MGMT_2_LP_STBY_XA_MSK)>>MPU_PWR_MGMT_2_LP_STBY_XA_POS;
	return (bool)standySts;
}

/**
 *
 */
bool AccelAndGyro::setStandbyYAccel(bool enable)
{
	uint8_t pwrMgmt2Val;
	if(readByte(MPU6050_PWR_MGMT_2_REG,&pwrMgmt2Val) == false)
	{
		return false;
	}
	pwrMgmt2Val &= ~MPU_PWR_MGMT_2_LP_STBY_YA_MSK;
	if(enable)
	{
		pwrMgmt2Val |= MPU_PWR_MGMT_2_LP_STBY_YA_MSK;
	}
	return writeByte(MPU6050_PWR_MGMT_2_REG,pwrMgmt2Val);
}

/**
 *
 */
bool AccelAndGyro::getStandbyYAccelSts(void)
{
	uint8_t pwrMgmt2Val;
	uint8_t standySts;
	if(readByte(MPU6050_PWR_MGMT_2_REG,&pwrMgmt2Val) == false)
	{
		return false;
	}
	standySts = (pwrMgmt2Val & MPU_PWR_MGMT_2_LP_STBY_YA_MSK)>>MPU_PWR_MGMT_2_LP_STBY_YA_POS;
	return (bool)standySts;
}

/**
 *
 */
bool AccelAndGyro::setStandbyZAccel(bool enable)
{
	uint8_t pwrMgmt2Val;
	if(readByte(MPU6050_PWR_MGMT_2_REG,&pwrMgmt2Val) == false)
	{
		return false;
	}
	pwrMgmt2Val &= ~MPU_PWR_MGMT_2_LP_STBY_ZA_MSK;
	if(enable)
	{
		pwrMgmt2Val |= MPU_PWR_MGMT_2_LP_STBY_ZA_MSK;
	}
	return writeByte(MPU6050_PWR_MGMT_2_REG,pwrMgmt2Val);
}

/**
 *
 */
bool AccelAndGyro::getStandbyZAccelSts(void)
{
	uint8_t pwrMgmt2Val;
	uint8_t standySts;
	if(readByte(MPU6050_PWR_MGMT_2_REG,&pwrMgmt2Val) == false)
	{
		return false;
	}
	standySts = (pwrMgmt2Val & MPU_PWR_MGMT_2_LP_STBY_ZA_MSK)>>MPU_PWR_MGMT_2_LP_STBY_ZA_POS;
	return (bool)standySts;
}

/**
 *
 */
bool AccelAndGyro::setStandbyXGyro(bool enable)
{
	uint8_t pwrMgmt2Val;
	if(readByte(MPU6050_PWR_MGMT_2_REG,&pwrMgmt2Val) == false)
	{
		return false;
	}
	pwrMgmt2Val &= ~MPU_PWR_MGMT_2_LP_STBY_XG_MSK;
	if(enable)
	{
		pwrMgmt2Val |= MPU_PWR_MGMT_2_LP_STBY_XG_MSK;
	}
	return writeByte(MPU6050_PWR_MGMT_2_REG,pwrMgmt2Val);
}

/**
 *
 */
bool AccelAndGyro::getStandbyXGyroSts(void)
{
	uint8_t pwrMgmt2Val;
	uint8_t standySts;
	if(readByte(MPU6050_PWR_MGMT_2_REG,&pwrMgmt2Val) == false)
	{
		return false;
	}
	standySts = (pwrMgmt2Val & MPU_PWR_MGMT_2_LP_STBY_XG_MSK)>>MPU_PWR_MGMT_2_LP_STBY_XG_POS;
	return (bool)standySts;
}

/**
 *
 */
bool AccelAndGyro::setStandbyYGyro(bool enable)
{
	uint8_t pwrMgmt2Val;
	if(readByte(MPU6050_PWR_MGMT_2_REG,&pwrMgmt2Val) == false)
	{
		return false;
	}
	pwrMgmt2Val &= ~MPU_PWR_MGMT_2_LP_STBY_YG_MSK;
	if(enable)
	{
		pwrMgmt2Val |= MPU_PWR_MGMT_2_LP_STBY_YG_MSK;
	}
	return writeByte(MPU6050_PWR_MGMT_2_REG,pwrMgmt2Val);
}

/**
 *
 */
bool AccelAndGyro::getStandbyYGyroSts(void)
{
	uint8_t pwrMgmt2Val;
	uint8_t standySts;
	if(readByte(MPU6050_PWR_MGMT_2_REG,&pwrMgmt2Val) == false)
	{
		return false;
	}
	standySts = (pwrMgmt2Val & MPU_PWR_MGMT_2_LP_STBY_YG_MSK)>>MPU_PWR_MGMT_2_LP_STBY_YG_POS;
	return (bool)standySts;
}

/**
 *
 */
bool AccelAndGyro::setStandbyZGyro(bool enable)
{
	uint8_t pwrMgmt2Val;
	if(readByte(MPU6050_PWR_MGMT_2_REG,&pwrMgmt2Val) == false)
	{
		return false;
	}
	pwrMgmt2Val &= ~MPU_PWR_MGMT_2_LP_STBY_ZG_MSK;
	if(enable)
	{
		pwrMgmt2Val |= MPU_PWR_MGMT_2_LP_STBY_ZG_MSK;
	}
	return writeByte(MPU6050_PWR_MGMT_2_REG,pwrMgmt2Val);
}

/**
 *
 */
bool AccelAndGyro::getStandbyZGyroSts(void)
{
	uint8_t pwrMgmt2Val;
	uint8_t standySts;
	if(readByte(MPU6050_PWR_MGMT_2_REG,&pwrMgmt2Val) == false)
	{
		return false;
	}
	standySts = (pwrMgmt2Val & MPU_PWR_MGMT_2_LP_STBY_ZG_MSK)>>MPU_PWR_MGMT_2_LP_STBY_ZG_POS;
	return (bool)standySts;
}

/**
 *
 */
uint8_t AccelAndGyro::getMotionDetectionThreshold(void)
{
	uint8_t threshold;
	if(readByte(MPU6050_MOTION_THR,&threshold))
	{
		return threshold;
	}
	return 0u;
}

/**
 *
 */
bool AccelAndGyro::setMotionDetectionThreshold(uint8_t threshold)
{
	return writeByte(MPU6050_MOTION_THR,threshold);
}

/**
 *
 */
uint8_t AccelAndGyro::getMotionDetectionDuration(void)
{
	uint8_t duration;
	if(readByte(MPU6050_MOTION_DUR,&duration))
	{
		return duration;
	}
	return 0u;
}

/**
 *
 */
bool AccelAndGyro::setMotionDetectionDuration(uint8_t threshold)
{
	return writeByte(MPU6050_MOTION_DUR,threshold);
}

/**
 *
 */
uint8_t AccelAndGyro::getZeroMotionDetectionThreshold(void)
{
	uint8_t threshold;
	if(readByte(MPU6050_ZERO_MOTION_THR,&threshold))
	{
		return threshold;
	}
	return 0u;
}

/**
 *
 */
bool AccelAndGyro::setZeroMotionDetectionThreshold(uint8_t threshold)
{
	return writeByte(MPU6050_ZERO_MOTION_THR,threshold);
}

/**
 *
 */
uint8_t AccelAndGyro::getZeroMotionDetectionDuration(void)
{
	uint8_t duration;
	if(readByte(MPU6050_ZERO_MOTION_DUR,&duration))
	{
		return duration;
	}
	return 0u;
}

/**
 *
 */
bool  AccelAndGyro::setZeroMotionDetectionDuration(uint8_t threshold)
{
	return writeByte(MPU6050_ZERO_MOTION_DUR,threshold);
}

/**
 *
 */
bool AccelAndGyro::getIntMotionEnabled(void)
{
	uint8_t intEnable;
	uint8_t motionEn;
	if(readByte(MPU6050_INT_ENABLE,&intEnable) == false)
	{
		return false;
	}
	motionEn = (intEnable & MPU_INT_MOTION_DETECT_MSK)>>MPU_INT_MOTION_DETECT_POS;
	return (bool)motionEn;
}

/**
 *
 */
bool AccelAndGyro::setIntMotionEnabled(bool enable)
{
	uint8_t intEnable;
	if(readByte(MPU6050_INT_ENABLE,&intEnable) == false)
	{
		return false;
	}
	intEnable &= ~MPU_INT_MOTION_DETECT_MSK;
	if(enable)
	{
		intEnable |= MPU_INT_MOTION_DETECT_MSK;
	}
	return writeByte(MPU6050_INT_ENABLE,intEnable);
}


/**
 *
 */
bool AccelAndGyro::getIntMotionStatus(void)
{
	uint8_t intSts;
	uint8_t motionSts;
	if(readByte(MPU6050_INT_STATUS,&intSts) == false)
	{
		return false;
	}
	motionSts = (intSts & MPU_INT_MOTION_DETECT_MSK)>>MPU_INT_MOTION_DETECT_POS;
	return (bool)motionSts;
}

/**
 *
 */
bool AccelAndGyro::getIntZeroMotionEnabled(void)
{
	uint8_t intEnable;
	uint8_t zeroMotionEn;
	if(readByte(MPU6050_INT_ENABLE,&intEnable) == false)
	{
		return false;
	}
	zeroMotionEn = (intEnable & MPU_INT_ZMOTION_DETECT_MSK)>>MPU_INT_ZMOTION_DETECT_POS;
	return (bool)zeroMotionEn;
}

/**
 *
 */
bool AccelAndGyro::setIntZeroMotionEnabled(bool enable)
{
	uint8_t intEnable;
	if(readByte(MPU6050_INT_ENABLE,&intEnable) == false)
	{
		return false;
	}
	intEnable &= ~MPU_INT_ZMOTION_DETECT_MSK;
	if(enable)
	{
		intEnable |= MPU_INT_ZMOTION_DETECT_MSK;
	}
	return writeByte(MPU6050_INT_ENABLE,intEnable);
}

/**
 *
 */
bool AccelAndGyro::getIntZeroMotionStatus(void)
{
	uint8_t intSts;
	uint8_t zeroMotionSts;
	if(readByte(MPU6050_INT_STATUS,&intSts) == false)
	{
		return false;
	}
	zeroMotionSts = (intSts & MPU_INT_ZMOTION_DETECT_MSK)>>MPU_INT_ZMOTION_DETECT_POS;
	return (bool)zeroMotionSts;
}

/**
 *
 */
float AccelAndGyro::getAccelX(bool print)
{
	int8_t data[2u];
	uint8_t fsrSel;
	int16_t raw=0;
	float aX = 0.0f;
	if(readMultiBytes(MPU6050_ACCEL_XOUT_H_REG,2u,(uint8_t *)data))
	{
		raw = (data[0u] << 8)| data[1u];
	}
	fsrSel =  getFullScaleAccelRange();
	if(fsrSel == 0x0Fu)
	{
		return 0.0f;
	}
	aX = (float)raw * _accelScale[fsrSel] * 100.f;
	if(print)
	{
		Serial.print("Acceleration(X): ");
		Serial.print(aX,2);
		Serial.println("cm/s^2");
	}
	return aX;
}

/**
 *
 */
float AccelAndGyro::getAccelY(bool print)
{
	int8_t data[2u];
	uint8_t fsrSel;
	int16_t raw=0;
	float aY = 0.0f;
	if(readMultiBytes(MPU6050_ACCEL_YOUT_H_REG,2u,(uint8_t *)data))
	{
		raw = (data[0u] << 8)| data[1u];
	}
	fsrSel =  getFullScaleAccelRange();
	if(fsrSel == 0x0Fu)
	{
		return 0.0f;
	}
	aY = (float)raw * _accelScale[fsrSel] * 100.f;
	if(print)
	{
		Serial.print("Acceleration(Y): ");
		Serial.print(aY,2);
		Serial.println("cm/s^2");
	}
	return aY;
}

/**
 *
 */
float AccelAndGyro::getAccelZ(bool print)
{
	int8_t data[2u];
	uint8_t fsrSel;
	int16_t raw=0;
	float aZ = 0.0f;
	if(readMultiBytes(MPU6050_ACCEL_ZOUT_H_REG,2u,(uint8_t *)data))
	{
		raw = (data[0u] << 8)| data[1u];
	}
	fsrSel =  getFullScaleAccelRange();
	if(fsrSel == 0x0Fu)
	{
		return 0.0f;
	}
	aZ = (float)raw * _accelScale[fsrSel] * 100.f;
	if(print)
	{
		Serial.print("Acceleration(Z): ");
		Serial.print(aZ,2);
		Serial.println("cm/s^2");
	}
	return aZ;
}

/**
 *
 */
float AccelAndGyro::getGyroX(bool print)
{
	int8_t data[2u];
	uint8_t fsrSel;
	int16_t raw=0;
	float gX = 0.0f;
	if(readMultiBytes(MPU6050_GYRO_XOUT_H_REG,2u,(uint8_t *)data))
	{
		raw = (data[0u] << 8)| data[1u];
	}
	fsrSel =  getFullScaleGyroRange();
	if(fsrSel == 0x0Fu)
	{
		return 0.0f;
	}
	gX = (float)raw * _gyroScale[fsrSel];
	if(print)
	{
		Serial.print("Angular Velocity(X): ");
		Serial.print(gX,2);
		Serial.println("°/s");
	}
	return gX;
}

/**
 *
 */
float AccelAndGyro::getGyroY(bool print)
{
	int8_t data[2u];
	uint8_t fsrSel;
	int16_t raw=0;
	float gY = 0.0f;
	if(readMultiBytes(MPU6050_GYRO_YOUT_H_REG,2u,(uint8_t *)data))
	{
		raw = (data[0u] << 8)| data[1u];
	}
	fsrSel =  getFullScaleGyroRange();
	if(fsrSel == 0x0Fu)
	{
		return 0.0f;
	}
	gY = (float)raw * _accelScale[fsrSel] * 100.f;
	if(print)
	{
		Serial.print("Angular Velocity(Y): ");
		Serial.print(gY,2);
		Serial.println("°/s");
	}
	return gY;
}

/**
 *
 */
float AccelAndGyro::getGyroZ(bool print)
{
	int8_t data[2u];
	uint8_t fsrSel;
	int16_t raw=0;
	float gZ = 0.0f;
	if(readMultiBytes(MPU6050_GYRO_ZOUT_H_REG,2u,(uint8_t *)data))
	{
		raw = (data[0u] << 8)| data[1u];
	}
	fsrSel =  getFullScaleGyroRange();
	if(fsrSel == 0x0Fu)
	{
		return 0.0f;
	}
	gZ = (float)raw * _gyroScale[fsrSel];
	if(print)
	{
		Serial.print("Angular Velocity(Z): ");
		Serial.print(gZ,2);
		Serial.println("°/s");
	}
	return gZ;
}

/**
 *
 */
bool AccelAndGyro::getAccel(int16_t *aX, int16_t *aY, int16_t *aZ)
{
	int8_t accel[6u];
	if(readMultiBytes(MPU6050_ACCEL_XOUT_H_REG,6u,(uint8_t *)accel))
	{
		*aX = (accel[0u] << 8)| accel[1u];
		*aY = (accel[2u] << 8)| accel[3u];
		*aZ = (accel[4u] << 8)| accel[5u];
		return true;
	}
	return false;
}

/**
 *
 */
bool AccelAndGyro::getGyro(int16_t *gX, int16_t *gY, int16_t *gZ)
{
	int8_t gyro[6u];
	if(readMultiBytes(MPU6050_GYRO_XOUT_H_REG,6u,(uint8_t *)gyro))
	{
		*gX = (gyro[0u] << 8)| gyro[1u];
		*gY = (gyro[2u] << 8)| gyro[3u];
		*gZ = (gyro[4u] << 8)| gyro[5u];
		return true;
	}
	return false;
}


/**
 *
 */
bool AccelAndGyro::getAccelOffset(int16_t *aX, int16_t *aY, int16_t *aZ)
{
	int8_t data[6u];
	if(readMultiBytes(MPU6050_XA_OFFS_USRH_REG,6u,(uint8_t *)data))
	{
		*aX = (data[0u] << 8)| data[1u];
		*aY = (data[2u] << 8)| data[3u];
		*aZ = (data[4u] << 8)| data[5u];
		return true;
	}
	return false;
}

/**
 *
 */
bool AccelAndGyro::setAccelOffset(int16_t *aX, int16_t *aY, int16_t *aZ)
{
	uint8_t data[6u];
	data[0u] = (*aX >> 8);
	data[1u] = (*aX & 0xFF);
	data[2u] = (*aY >> 8);
	data[3u] = (*aY & 0xFF);
	data[4u] = (*aZ >> 8);
	data[5u] = (*aZ & 0xFF);

	if(writeMultiBytes(MPU6050_XA_OFFS_USRH_REG,6u,data))
	{
		return true;
	}
	return false;
}

/**
 *
 */
bool AccelAndGyro::getGyroOffset(int16_t *gX, int16_t *gY, int16_t *gZ)
{
	int8_t data[6u];
	if(readMultiBytes(MPU6050_XG_OFFS_USRH_REG,6u,(uint8_t *)data))
	{
		*gX = (data[0u] << 8)| data[1u];
		*gY = (data[2u] << 8)| data[3u];
		*gZ = (data[4u] << 8)| data[5u];
		return true;
	}
	return false;
}

/**
 *
 */
bool AccelAndGyro::setGyroOffset(int16_t *gX, int16_t *gY, int16_t *gZ)
{
	uint8_t data[6u];
	data[0u] = (*gX >> 8);
	data[1u] = (*gX & 0xFF);
	data[2u] = (*gY >> 8);
	data[3u] = (*gY & 0xFF);
	data[4u] = (*gZ >> 8);
	data[5u] = (*gZ & 0xFF);

	if(writeMultiBytes(MPU6050_XG_OFFS_USRH_REG,6u,data))
	{
		return true;
	}
	return false;
}

/**
 *
 */
float AccelAndGyro::getTempC(bool print)
{
	int8_t data[2u];
	int16_t temp;
	float tempC;
	if(readMultiBytes(MPU6050_TEMP_OUT_H_REG,6u,(uint8_t *)data))
	{
		temp 	= (data[0u] << 8u) | data[1u] ;
		tempC 	= ((float)temp/340.f)+36.53f;
		if(print)
		{
			Serial.print("Temperature (°C): ");
		    Serial.print(tempC,2);
		    Serial.println("°C");
		}
		return tempC;
	}
	return 0.f;
}

/**
 *
 */
float AccelAndGyro::getTempF(bool print)
{
	float tempF = (getTempC(false) * (9.f / 5.f)) + 32.f;
	if(print)
    {
      Serial.print("Temperature (°F): ");
      Serial.print(tempF,2);
      Serial.println("°F");
    }
	return tempF;
}

/**
 *
 */
float AccelAndGyro::getTiltX(bool print)
{
	float tiltX;
	int16_t aX, aY, aZ;

	getAccel(&aX,&aY,&aZ);
	tiltX = (180.0/M_PI)*atan(aX/(sqrt(pow(aY,2)+pow(aZ,2))));

	if(print)
    {
      Serial.print("Tilt Angle(X): ");
      Serial.print(tiltX,2);
      Serial.println("°");
    }
	return tiltX;
}

/**
 *
 */
float AccelAndGyro::getTiltY(bool print)
{
	float tiltY;
	int16_t aX, aY, aZ;

	getAccel(&aX,&aY,&aZ);
	tiltY = (180.0/M_PI)*atan(aY/(sqrt(pow(aX,2)+pow(aZ,2))));

	if(print)
    {
      Serial.print("Tilt Angle(Y): ");
      Serial.print(tiltY,2);
      Serial.println("°");
    }
	return tiltY;
}

/**
 *
 */
float AccelAndGyro::getTiltZ(bool print)
{
	float tiltZ;
	int16_t aX, aY, aZ;

	getAccel(&aX,&aY,&aZ);
	tiltZ = (180.0/M_PI)*atan((sqrt(pow(aX,2)+pow(aY,2))/aZ));

	if(print)
    {
      Serial.print("Tilt Angle(Z): ");
      Serial.print(tiltZ,2);
      Serial.println("°");
    }
	return tiltZ;
}

/**
 *
 */
bool AccelAndGyro::getMotionStatus(bool print)
{
	bool motionSts = getIntMotionStatus();
	Serial.print("Motion Detection Status: ");
	if(motionSts)
	{
		 Serial.println("True");
	}
	else
	{
		Serial.println("False");
	}
	return motionSts;
}

/***********************************************************************************************
 * Platform dependent routines. Change these functions implementation based on microcontroller *
 ***********************************************************************************************/
/**
 *
 */
void AccelAndGyro::i2c_init(void)
{
	Wire.begin();
	Wire.setClock(100000);
}

/**
 *
 */
bool AccelAndGyro::readByte(uint8_t reg, uint8_t *in)
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
bool AccelAndGyro::readMultiBytes(uint8_t reg, uint8_t length, uint8_t *in)
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
bool AccelAndGyro::writeByte(uint8_t reg)
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
bool AccelAndGyro::writeByte(uint8_t reg, uint8_t val)
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
bool AccelAndGyro::writeAddress(void)
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
bool AccelAndGyro::writeMultiBytes(uint8_t reg, uint8_t length, const uint8_t *in)
{
  Wire.beginTransmission((uint8_t)_i2cSlaveAddress);
  Wire.write(reg);
  Wire.write(in,length);
  if (Wire.endTransmission(true) == 0)
  {
    return true;
  }
  return false;
}

/**
 *
 */
void AccelAndGyro::delay_ms(uint16_t ms)
{
  delay(ms);
}
