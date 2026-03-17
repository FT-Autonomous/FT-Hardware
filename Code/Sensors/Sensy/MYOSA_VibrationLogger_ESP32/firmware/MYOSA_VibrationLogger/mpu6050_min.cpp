#include "mpu6050_min.h"

// MPU6050 registers
static constexpr uint8_t REG_WHO_AM_I     = 0x75;
static constexpr uint8_t REG_PWR_MGMT_1   = 0x6B;
static constexpr uint8_t REG_SMPLRT_DIV   = 0x19;
static constexpr uint8_t REG_CONFIG       = 0x1A;
static constexpr uint8_t REG_GYRO_CONFIG  = 0x1B;
static constexpr uint8_t REG_ACCEL_CONFIG = 0x1C;
static constexpr uint8_t REG_ACCEL_XOUT_H = 0x3B;
static constexpr uint8_t REG_TEMP_OUT_H   = 0x41;

MPU6050Minimal::MPU6050Minimal()
    : _wire(nullptr), _addr(0), _connected(false), _accelLSBPerG(16384.0f) {}

bool MPU6050Minimal::begin(TwoWire &wire,
                           int8_t forcedAddress,
                           uint16_t sampleRateHz,
                           AccelRange accelRange,
                           uint8_t dlpfCfg) {
  _wire = &wire;
  _connected = false;

  if (forcedAddress >= 0) {
    if (!detect(static_cast<uint8_t>(forcedAddress))) {
      return false;
    }
  } else {
    // Try common addresses: 0x69 (MYOSA docs) then 0x68.
    if (!detect(0x69) && !detect(0x68)) {
      return false;
    }
  }

  if (!configure(sampleRateHz, accelRange, dlpfCfg)) {
    _connected = false;
    return false;
  }

  _connected = true;
  return true;
}

bool MPU6050Minimal::detect(uint8_t addr) {
  _addr = addr;
  uint8_t who = 0;
  if (!readRegs(REG_WHO_AM_I, &who, 1)) {
    return false;
  }
  // WHO_AM_I is typically 0x68.
  if (who != 0x68) {
    return false;
  }
  return true;
}

bool MPU6050Minimal::configure(uint16_t sampleRateHz, AccelRange accelRange, uint8_t dlpfCfg) {
  // Wake up + select PLL with X gyro as clock source (more stable than internal oscillator)
  // CLKSEL=1, SLEEP=0
  if (!writeReg(REG_PWR_MGMT_1, 0x01)) {
    return false;
  }
  delay(10);

  // DLPF config (1..6 recommended). 1=184Hz, 2=94Hz, 3=44Hz...
  dlpfCfg &= 0x07;
  if (dlpfCfg == 0 || dlpfCfg == 7) {
    // Avoid 0/7 here because that switches gyro output rate to 8kHz,
    // which complicates sample-rate math. Force to 1.
    dlpfCfg = 1;
  }
  if (!writeReg(REG_CONFIG, dlpfCfg)) {
    return false;
  }

  // Gyro full scale: ±250 deg/s (0)
  if (!writeReg(REG_GYRO_CONFIG, 0x00)) {
    return false;
  }

  // Accel full scale range
  const uint8_t accelCfg = (uint8_t)((accelRange & 0x03) << 3);
  if (!writeReg(REG_ACCEL_CONFIG, accelCfg)) {
    return false;
  }

  // Accel sensitivity (LSB per g)
  switch (accelRange) {
    case RANGE_2G:
      _accelLSBPerG = 16384.0f;
      break;
    case RANGE_4G:
      _accelLSBPerG = 8192.0f;
      break;
    case RANGE_8G:
      _accelLSBPerG = 4096.0f;
      break;
    case RANGE_16G:
      _accelLSBPerG = 2048.0f;
      break;
  }

  // Sample-rate divider math (with DLPF enabled, internal rate is 1kHz)
  if (sampleRateHz < 10) sampleRateHz = 10;
  if (sampleRateHz > 1000) sampleRateHz = 1000;

  // div = (1000 / sampleRateHz) - 1
  uint16_t div = 0;
  if (sampleRateHz >= 1000) {
    div = 0;
  } else {
    div = (uint16_t)(1000 / sampleRateHz);
    if (div == 0) div = 1;
    div = (uint16_t)(div - 1);
  }
  if (div > 255) div = 255;

  if (!writeReg(REG_SMPLRT_DIV, (uint8_t)div)) {
    return false;
  }

  return true;
}

bool MPU6050Minimal::readAccelG(float &ax_g, float &ay_g, float &az_g) {
  uint8_t buf[6] = {0};
  if (!readRegs(REG_ACCEL_XOUT_H, buf, sizeof(buf))) {
    return false;
  }

  const int16_t ax_raw = (int16_t)((buf[0] << 8) | buf[1]);
  const int16_t ay_raw = (int16_t)((buf[2] << 8) | buf[3]);
  const int16_t az_raw = (int16_t)((buf[4] << 8) | buf[5]);

  const float k = 1.0f / _accelLSBPerG;
  ax_g = ax_raw * k;
  ay_g = ay_raw * k;
  az_g = az_raw * k;
  return true;
}

bool MPU6050Minimal::readTemperatureC(float &temp_c) {
  uint8_t buf[2] = {0};
  if (!readRegs(REG_TEMP_OUT_H, buf, sizeof(buf))) {
    return false;
  }
  const int16_t raw = (int16_t)((buf[0] << 8) | buf[1]);

  // Datasheet: Temp in °C = (raw / 340) + 36.53
  temp_c = (raw / 340.0f) + 36.53f;
  return true;
}

bool MPU6050Minimal::writeReg(uint8_t reg, uint8_t val) {
  _wire->beginTransmission(_addr);
  _wire->write(reg);
  _wire->write(val);
  const uint8_t err = _wire->endTransmission();
  return (err == 0);
}

bool MPU6050Minimal::readRegs(uint8_t reg, uint8_t *buf, size_t len) {
  _wire->beginTransmission(_addr);
  _wire->write(reg);
  const uint8_t err = _wire->endTransmission(false);  // repeated start
  if (err != 0) {
    return false;
  }
  const size_t got = _wire->requestFrom((int)_addr, (int)len);
  if (got != len) {
    while (_wire->available()) {
      (void)_wire->read();
    }
    return false;
  }
  for (size_t i = 0; i < len; i++) {
    buf[i] = _wire->read();
  }
  return true;
}
