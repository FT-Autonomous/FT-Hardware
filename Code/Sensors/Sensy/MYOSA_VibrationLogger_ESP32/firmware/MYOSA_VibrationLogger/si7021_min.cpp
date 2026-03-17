#include "si7021_min.h"

static constexpr uint8_t CMD_RESET = 0xFE;
static constexpr uint8_t CMD_READ_USER_REG = 0xE7;
static constexpr uint8_t CMD_MEASURE_TEMP_NOHOLD = 0xF3;

SI7021Minimal::SI7021Minimal() : _wire(nullptr), _addr(0x40), _connected(false) {}

bool SI7021Minimal::begin(TwoWire &wire, uint8_t addr) {
  _wire = &wire;
  _addr = addr;
  _connected = false;

  // Soft reset
  if (!writeCmd(CMD_RESET)) {
    return false;
  }
  delay(20);

  // Try reading user register (simple presence check)
  if (!writeCmd(CMD_READ_USER_REG)) {
    return false;
  }
  uint8_t reg = 0;
  if (!readBytes(&reg, 1)) {
    return false;
  }

  _connected = true;
  return true;
}

bool SI7021Minimal::readTemperatureC(float &temp_c) {
  if (!_connected) {
    return false;
  }

  // Trigger measurement (no-hold)
  if (!writeCmd(CMD_MEASURE_TEMP_NOHOLD)) {
    return false;
  }

  // Typical max conversion time ~11ms (depends on resolution). Give it a little margin.
  delay(15);

  uint8_t buf[3] = {0};
  if (!readBytes(buf, sizeof(buf))) {
    return false;
  }

  const uint8_t crc = crc8(buf, 2);
  if (crc != buf[2]) {
    return false;
  }

  const uint16_t raw = (uint16_t)((buf[0] << 8) | buf[1]);

  // Datasheet: T = (175.72 * raw / 65536) - 46.85
  temp_c = (175.72f * (float)raw / 65536.0f) - 46.85f;
  return true;
}

bool SI7021Minimal::writeCmd(uint8_t cmd) {
  _wire->beginTransmission(_addr);
  _wire->write(cmd);
  const uint8_t err = _wire->endTransmission();
  return (err == 0);
}

bool SI7021Minimal::readBytes(uint8_t *buf, size_t len) {
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

uint8_t SI7021Minimal::crc8(const uint8_t *data, size_t len) {
  // Si70xx CRC8: polynomial 0x31 (x^8 + x^5 + x^4 + 1), init 0x00.
  uint8_t crc = 0x00;
  for (size_t i = 0; i < len; i++) {
    crc ^= data[i];
    for (uint8_t b = 0; b < 8; b++) {
      if (crc & 0x80) {
        crc = (uint8_t)((crc << 1) ^ 0x31);
      } else {
        crc <<= 1;
      }
    }
  }
  return crc;
}
