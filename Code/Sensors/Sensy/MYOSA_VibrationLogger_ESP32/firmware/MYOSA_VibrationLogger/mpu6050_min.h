#pragma once

#include <Arduino.h>
#include <Wire.h>

// Minimal MPU6050 (GY-521) driver (no external dependencies)
// - Reads accelerometer (g)
// - Reads internal temperature (°C) (chip temperature, not ambient)
//
// Address can be 0x68 or 0x69 depending on AD0 pin.

class MPU6050Minimal {
 public:
  enum AccelRange {
    RANGE_2G  = 0,  // ±2g
    RANGE_4G  = 1,  // ±4g
    RANGE_8G  = 2,  // ±8g
    RANGE_16G = 3   // ±16g
  };

  MPU6050Minimal();

  // Auto-detect 0x68/0x69 unless an address is provided.
  // sampleRateHz: requested output rate (typical 100..500). Internally uses 1kHz base.
  // dlpfCfg: 1..6 for typical use (1=184Hz, 2=94Hz, 3=44Hz, ...). Use 1 for vibration.
  bool begin(TwoWire &wire = Wire,
             int8_t forcedAddress = -1,
             uint16_t sampleRateHz = 500,
             AccelRange accelRange = RANGE_4G,
             uint8_t dlpfCfg = 1);

  bool isConnected() const { return _connected; }
  uint8_t address() const { return _addr; }

  // Read accelerometer in g (gravity units). Returns false on I2C failure.
  bool readAccelG(float &ax_g, float &ay_g, float &az_g);

  // Read internal temperature in Celsius. Returns false on I2C failure.
  bool readTemperatureC(float &temp_c);

 private:
  TwoWire *_wire;
  uint8_t _addr;
  bool _connected;
  float _accelLSBPerG;

  bool writeReg(uint8_t reg, uint8_t val);
  bool readRegs(uint8_t reg, uint8_t *buf, size_t len);

  bool detect(uint8_t addr);
  bool configure(uint16_t sampleRateHz, AccelRange accelRange, uint8_t dlpfCfg);
};
