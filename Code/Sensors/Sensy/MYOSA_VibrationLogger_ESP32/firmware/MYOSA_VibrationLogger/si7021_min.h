#pragma once

#include <Arduino.h>
#include <Wire.h>

// Minimal Si7021 temperature driver (no external dependencies)
// (MYOSA Temp & Humidity board uses Si7021 at I2C 0x40)

class SI7021Minimal {
 public:
  SI7021Minimal();

  bool begin(TwoWire &wire = Wire, uint8_t addr = 0x40);
  bool isConnected() const { return _connected; }

  // Reads ambient temperature in °C.
  bool readTemperatureC(float &temp_c);

 private:
  TwoWire *_wire;
  uint8_t _addr;
  bool _connected;

  bool writeCmd(uint8_t cmd);
  bool readBytes(uint8_t *buf, size_t len);

  static uint8_t crc8(const uint8_t *data, size_t len);
};
