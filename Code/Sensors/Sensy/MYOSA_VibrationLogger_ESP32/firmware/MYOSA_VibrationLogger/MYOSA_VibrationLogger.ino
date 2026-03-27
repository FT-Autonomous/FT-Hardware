/*
  MYOSA Vibration + Temperature Logger (ESP32)
  ------------------------------------------------
  - No external sensor libraries required
  - Uses I2C MPU6050 (Accel/Gyro board in MYOSA kit)
  - Optionally uses I2C Si7021 (Temp+Humidity board in MYOSA kit) for ambient
  temperature
  - Streams data every 0.5s over:
      * Serial (human-readable by default, CSV optional)
      * BLE notifications (binary packet for Web Bluetooth)

  Hardware:
    - ESP32-WROOM-32E
    - MPU6050 at I2C address 0x69 (MYOSA docs) or 0x68
    - Si7021 at I2C address 0x40 (optional)

  Notes:
    - "Vibration" here is computed as RMS and peak of *linear* acceleration
  magnitude over a 0.5 second window.
    - Linear acceleration is approximated by subtracting a low-pass gravity
  estimate (good for vibration/shaking; not a full IMU fusion).
*/

#include <Arduino.h>
#include <math.h>
#include <string>
#include <cstring>
#include <Wire.h>

#include "freertos/FreeRTOS.h"
#include "freertos/portmacro.h"

#include <BLEDevice.h>
#include <BLEServer.h>
#include <BLEUtils.h>
#include <BLE2902.h>

#include "mpu6050_min.h"
#include "si7021_min.h"

// ------------------------- User-tunable settings -------------------------

// Choose serial output format: 0 for CSV (spreadsheets), 1 for Human-readable
static constexpr uint8_t SERIAL_FORMAT = 1;

// Tracking output period (ms). This determines how often a complete "packet" of summarized vibration data is sent out.
static constexpr uint32_t WINDOW_MS = 500;

// Sensor sampling rate in Hertz (Hz). How many raw readings the ESP32 asks the motion sensor for every single second.
static constexpr uint16_t SAMPLE_HZ = 500;

// Accelerometer physical range. More headroom (higher Gs) means it can measure violent shaking without clipping.
static constexpr MPU6050Minimal::AccelRange ACCEL_RANGE = MPU6050Minimal::RANGE_4G;

// MPU6050 DLPF config: 1 = 184Hz filter.
static constexpr uint8_t MPU_DLPF_CFG = 1;

// I2C pins. -1 means use the default pins for the board (usually SDA=21, SCL=22).
static constexpr int8_t I2C_SDA_PIN = -1;
static constexpr int8_t I2C_SCL_PIN = -1;

// Speed of the I2C wires. 400,000 Hz (Fast Mode).
static constexpr uint32_t I2C_CLOCK_HZ = 400000;

// The text name that will show up when scanning for Bluetooth devices
static const char *BLE_DEVICE_NAME = "MYOSA-VibeLogger";

// -------------------------------------------------------------------------
// Shake score + severity thresholds
// -------------------------------------------------------------------------
// Below, we define the scores (0-1000) that push the reading into different severity buckets.
static constexpr uint16_t THRESH_CAUTION_SCORE = 25;
static constexpr uint16_t THRESH_PARTIAL_SCORE = 80;
static constexpr uint16_t THRESH_SEVERE_SCORE  = 150;

// Any score 0..5 is categorized as "Still" to prevent tiny natural background vibrations from triggering.
static constexpr uint16_t STILL_SCORE_MAX = 5;

static_assert(THRESH_CAUTION_SCORE > STILL_SCORE_MAX,
              "THRESH_CAUTION_SCORE must be > 5");
static_assert(THRESH_PARTIAL_SCORE > THRESH_CAUTION_SCORE,
              "THRESH_PARTIAL_SCORE must be > THRESH_CAUTION_SCORE");
static_assert(THRESH_SEVERE_SCORE > THRESH_PARTIAL_SCORE,
              "THRESH_SEVERE_SCORE must be > THRESH_PARTIAL_SCORE");
static_assert(THRESH_SEVERE_SCORE <= 1000,
              "THRESH_SEVERE_SCORE must be <= 1000");

// ------------------------- BLE UUIDs (ID numbers used by Bluetooth) -------------------------
static const char *SERVICE_UUID   = "7b4e3a0d-6f1b-4d1a-9c62-4c6b8a2e7a10";
static const char *DATA_CHAR_UUID = "7b4e3a0d-6f1b-4d1a-9c62-4c6b8a2e7a11";
static const char *TIME_CHAR_UUID = "7b4e3a0d-6f1b-4d1a-9c62-4c6b8a2e7a12";

// ------------------------- Globals -------------------------
MPU6050Minimal mpu;
SI7021Minimal si7021;
bool hasMpu = false;
bool hasSi7021 = false;

BLEServer *g_server = nullptr;
BLECharacteristic *g_dataChar = nullptr;
BLECharacteristic *g_timeChar = nullptr;

// 'volatile' is used here as these values are modified in background callbacks
volatile bool g_deviceConnected = false;
volatile bool g_shouldAdvertise = false;

// ------------------------- Time sync mechanics -------------------------
static portMUX_TYPE g_timeMux = portMUX_INITIALIZER_UNLOCKED;

// Tracking what the real world time (Epoch) was at a specific uptime (Millis)
static uint64_t g_epochBaseMs = 0;   // The real world time received from browser (in ms since 1970)
static uint32_t g_millisBase = 0;    // The internal ESP32 uptime when we received the real-world time
static bool g_timeSynced = false;

// Calculates current real world time using the baseline sync and current uptime
static uint64_t getEpochMsOrZero() {
  if (!g_timeSynced) {
    return 0;
  }
  uint64_t baseMs;
  uint32_t baseMillis;

  // Protect the read operation since time may update from another core
  portENTER_CRITICAL(&g_timeMux);
  baseMs = g_epochBaseMs;
  baseMillis = g_millisBase;
  portEXIT_CRITICAL(&g_timeMux);

  const uint32_t nowMillis = millis();
  const uint32_t delta = (uint32_t)(nowMillis - baseMillis);
  return baseMs + (uint64_t)delta;
}

// Saves the real world time when we receive it over BLE
static void setEpochMs(uint64_t epochMs) {
  portENTER_CRITICAL(&g_timeMux);
  g_epochBaseMs = epochMs;
  g_millisBase = millis();
  g_timeSynced = true;
  portEXIT_CRITICAL(&g_timeMux);
}

// ------------------------- Bluetooth Packet Structure -------------------------
// #pragma pack(push, 1) instructs the compiler to pack these variables without empty padding bytes
#pragma pack(push, 1) 
struct VibePacket {
  uint32_t t_s;         // Real time in seconds
  uint16_t t_ms;        // Milliseconds fraction of a second (0-999)
  uint16_t rms_mg;      // RMS average shaking energy in milli-G forces
  uint16_t peak_mg;     // Single highest spike shake force in this 0.5s window
  uint16_t shake_score; // 0..1000 score
  int16_t temp_c_x100;  // Temp in celsius multiplied by 100 (e.g., 23.50C sent as 2350)
  uint8_t level;        // 0 to 4 meaning Still, Light, Caution, Partial, Severe
};
#pragma pack(pop)

// ------------------------- Data Math Utilities -------------------------
// Convert the hardware setting (like RANGE_4G) to actual math decimal (4.0)
static constexpr float accelRangeFullScaleG(MPU6050Minimal::AccelRange r) {
  return (r == MPU6050Minimal::RANGE_2G)  ? 2.0f :
         (r == MPU6050Minimal::RANGE_4G)  ? 4.0f :
         (r == MPU6050Minimal::RANGE_8G)  ? 8.0f :
         (r == MPU6050Minimal::RANGE_16G) ? 16.0f :
                                            4.0f;
}

static constexpr float SCORE_FULL_SCALE_G = accelRangeFullScaleG(ACCEL_RANGE);

// Formula to translate raw physics math into a 0-1000 score
static uint16_t shakeScoreFromRmsG(float rms_g) {
  if (!isfinite(rms_g) || rms_g <= 0.0f) {
    return 0;
  }
  // Convert based on percentage of max force.
  const float s = (rms_g / SCORE_FULL_SCALE_G) * 1000.0f;
  const int score = (int)lroundf(s);
  return (uint16_t)constrain(score, 0, 1000);
}

// Convert a numerical score into a 0 to 4 intensity category
static uint8_t levelFromScore(uint16_t score) {
  if (score <= STILL_SCORE_MAX) {
    return 0; // Still
  }
  if (score < THRESH_CAUTION_SCORE) {
    return 1; // Light
  }
  if (score < THRESH_PARTIAL_SCORE) {
    return 2; // Caution
  }
  if (score < THRESH_SEVERE_SCORE) {
    return 3; // Partial
  }
  return 4;   // Severe
}

static const char *levelLabel(uint8_t level) {
  switch (level) {
    case 0:
      return "Still";
    case 1:
      return "Light";
    case 2:
      return "Caution";
    case 3:
      return "Partial";
    default:
      return "Severe";
  }
}

// ------------------------- Bluetooth Handlers -------------------------

class MyServerCallbacks : public BLEServerCallbacks {
  void onConnect(BLEServer *pServer) override {
    g_deviceConnected = true;
  }

  void onDisconnect(BLEServer *pServer) override {
    g_deviceConnected = false;
    // Tell the main loop to resume advertising so devices can reconnect
    g_shouldAdvertise = true;
  }
};

// Handlers for receiving the time payload from the client
class TimeCharCallbacks : public BLECharacteristicCallbacks {
  void onWrite(BLECharacteristic *pCharacteristic) override {
    const size_t len = pCharacteristic->getLength();
    const uint8_t *data = pCharacteristic->getData();

    if (!data) return;

    if (len == 8) {
      uint64_t epochMs = 0;
      memcpy(&epochMs, data, 8);
      setEpochMs(epochMs);
      Serial.println("[Time] Synced via 8-byte epoch ms");
    } else if (len == 4) {
      uint32_t epochS = 0;
      memcpy(&epochS, data, 4);
      setEpochMs((uint64_t)epochS * 1000ULL);
      Serial.println("[Time] Synced via 4-byte epoch seconds");
    }
  }
};

static void setupBle() {
  BLEDevice::init(BLE_DEVICE_NAME);

  g_server = BLEDevice::createServer();
  g_server->setCallbacks(new MyServerCallbacks());

  BLEService *service = g_server->createService(SERVICE_UUID);

  g_dataChar = service->createCharacteristic(
      DATA_CHAR_UUID,
      BLECharacteristic::PROPERTY_NOTIFY | BLECharacteristic::PROPERTY_READ);
  g_dataChar->addDescriptor(new BLE2902());

  g_timeChar = service->createCharacteristic(
      TIME_CHAR_UUID,
      BLECharacteristic::PROPERTY_WRITE);
  g_timeChar->setCallbacks(new TimeCharCallbacks());

  service->start();

  BLEAdvertising *advertising = BLEDevice::getAdvertising();
  advertising->addServiceUUID(SERVICE_UUID);
  advertising->setScanResponse(true);
  advertising->setMinPreferred(0x06);
  advertising->setMinPreferred(0x12);

  BLEDevice::startAdvertising();
}

// ------------------------- Sensor Math Engine -------------------------
// Custom structure to hold all math being compiled over our 0.5s window
struct VibeAccum {
  float sumSq = 0.0f; // Running tally of shake power for RMS
  float peak = 0.0f;  // Highest spike felt in the current window
  uint32_t n = 0;     // Number of readings taken (usually ~250 per 0.5s)

  // Running low-pass average of G-force to estimate gravity direction
  float gx = 0.0f;
  float gy = 0.0f;
  float gz = 0.0f;

  bool gravityInit = false;
};

static VibeAccum g_acc;

static void resetWindow() {
  g_acc.sumSq = 0.0f;
  g_acc.peak = 0.0f;
  g_acc.n = 0;
}

// Feeds one single reading from the sensor into our math engine
static void addSample(float ax_g, float ay_g, float az_g) {
  // Use a low-pass filter to separate Gravity from linear Shaking.
  constexpr float tau = 0.5f;                       // seconds
  constexpr float dt = 1.0f / (float)SAMPLE_HZ;     // time between readings
  constexpr float alpha = dt / (tau + dt);          // low-pass multiplier

  if (!g_acc.gravityInit) {
    g_acc.gx = ax_g;
    g_acc.gy = ay_g;
    g_acc.gz = az_g;
    g_acc.gravityInit = true;
  } else {
    // Slowly pull the gravity variable toward the current reading
    g_acc.gx += alpha * (ax_g - g_acc.gx); 
    g_acc.gy += alpha * (ay_g - g_acc.gy);
    g_acc.gz += alpha * (az_g - g_acc.gz);
  }

  // Find pure shaking by subtracting the gravity offset
  const float lx = ax_g - g_acc.gx;
  const float ly = ay_g - g_acc.gy; 
  const float lz = az_g - g_acc.gz;

  // Total combined power of shaking in any direction
  const float linMag = sqrtf(lx * lx + ly * ly + lz * lz);

  g_acc.sumSq += linMag * linMag;
  
  if (linMag > g_acc.peak) {
    g_acc.peak = linMag;
  }
  g_acc.n++;
}

static bool readTemperatureC(float &tempC) {
  if (hasSi7021) {
    if (si7021.readTemperatureC(tempC)) {
      return true;
    }
  }
  if (hasMpu) {
    return mpu.readTemperatureC(tempC);
  }
  return false;
}

static void publishWindow() {
  bool validVibe = (hasMpu && g_acc.n > 0); 

  float rms_g = NAN;
  float peak_g = NAN;
  uint16_t shake_score = 0; 
  uint8_t level = 0;

  if (validVibe) {
    rms_g = sqrtf(g_acc.sumSq / (float)g_acc.n);
    peak_g = g_acc.peak;
    shake_score = shakeScoreFromRmsG(rms_g);
    level = levelFromScore(shake_score);
  }

  float tempC = NAN;
  bool validTemp = readTemperatureC(tempC);

  // Handle Timestamp
  const uint64_t epochMs = getEpochMsOrZero();
  uint32_t t_s = 0;
  uint16_t t_ms = 0;
  
  if (epochMs != 0) {
    t_s = (uint32_t)(epochMs / 1000ULL);
    t_ms = (uint16_t)(epochMs % 1000ULL);
  }

  // Handle serial printing out the physical USB connection
  const uint64_t serialTimeMs = (epochMs != 0) ? epochMs : (uint64_t)millis();

  if (validVibe) {
    if (SERIAL_FORMAT == 0) {
      Serial.printf("%llu,%.4f,%.4f,%.2f,%u,%u,%s\n", 
                    (unsigned long long)serialTimeMs,
                    (double)rms_g,
                    (double)peak_g,
                    (double)(validTemp ? tempC : NAN),
                    (unsigned)shake_score,
                    (unsigned)level,
                    levelLabel(level));
    } else {
      const char *timeKey = (epochMs != 0) ? "epoch_ms" : "uptime_ms";
      Serial.printf("%s=%llu | rms_g=%.4f | peak_g=%.4f | temp_c=%.2f | shake_score=%u/1000 | level=%u (%s)\n",
                    timeKey,
                    (unsigned long long)serialTimeMs,
                    (double)rms_g,
                    (double)peak_g, 
                    (double)(validTemp ? tempC : NAN),
                    (unsigned)shake_score,
                    (unsigned)level,
                    levelLabel(level));
    }
  } else {
    if (SERIAL_FORMAT == 0) {
      Serial.printf("%llu,nan,nan,%.2f,0,0,NoSensor\n",
                    (unsigned long long)serialTimeMs,
                    (double)(validTemp ? tempC : NAN));
    } else {
      const char *timeKey = (epochMs != 0) ? "epoch_ms" : "uptime_ms";
      Serial.printf("%s=%llu | sensor=missing (MPU6050 not detected) | temp_c=%.2f\n",
                    timeKey,
                    (unsigned long long)serialTimeMs,
                    (double)(validTemp ? tempC : NAN));
    }
  }

  // Handle Bluetooth Transmission
  if (g_deviceConnected && g_dataChar) {
    VibePacket pkt;
    pkt.t_s = t_s;
    pkt.t_ms = t_ms;

    if (validVibe) {
      const int rms_mg = (int)lroundf(rms_g * 1000.0f);
      const int peak_mg = (int)lroundf(peak_g * 1000.0f);
      pkt.rms_mg = (uint16_t)constrain(rms_mg, 0, 65534); 
      pkt.peak_mg = (uint16_t)constrain(peak_mg, 0, 65534);
      pkt.shake_score = shake_score;
      pkt.level = level;
    } else {
      // Send max invalid sentinel values to indicate sensor disconnect
      pkt.rms_mg = 65535;
      pkt.peak_mg = 65535;
      pkt.shake_score = 65535;
      pkt.level = 0;
    }

    if (validTemp) {
      const int temp_x100 = (int)lroundf(tempC * 100.0f);
      pkt.temp_c_x100 = (int16_t)constrain(temp_x100, -32767, 32767);
    } else {
      pkt.temp_c_x100 = (int16_t)-32768; // Error code baseline
    }

    g_dataChar->setValue((uint8_t *)&pkt, sizeof(pkt));
    g_dataChar->notify();
  }

  resetWindow();
}

void setup() {
  Serial.begin(115200);
  delay(200);

  if (I2C_SDA_PIN >= 0 && I2C_SCL_PIN >= 0) {
    Wire.begin(I2C_SDA_PIN, I2C_SCL_PIN);
  } else {
    Wire.begin();
  }
  Wire.setClock(I2C_CLOCK_HZ);

  Serial.println("[Init] Searching for MPU6050...");
  hasMpu = mpu.begin(Wire, -1, SAMPLE_HZ, ACCEL_RANGE, MPU_DLPF_CFG);
  if (hasMpu) {
    Serial.printf("[Init] MPU6050 found at 0x%02X\n", mpu.address()); 
  } else {
    Serial.println("[Init] MPU6050 not found. BLE will still start so you can connect.");
    Serial.println("       Fix wiring (SDA=21, SCL=22, 3.3V, GND) and reset to retry.");
  }

  Serial.println("[Init] Searching for Si7021 (optional)...");
  if (si7021.begin(Wire, 0x40)) {
    hasSi7021 = true;
    Serial.println("[Init] Si7021 found (using ambient temperature)");
  } else {
    hasSi7021 = false;
    Serial.println("[Init] Si7021 not found (using MPU6050 internal temperature)");
  }

  Serial.println("[Init] Starting BLE...");
  setupBle();
  Serial.println("[Init] BLE advertising started");

  Serial.println("# Output every 0.5s");
  Serial.println("#  epoch_ms / uptime_ms : epoch milliseconds if time-synced, otherwise uptime milliseconds");
  Serial.println("#  rms_g                : RMS linear acceleration magnitude over last 0.5s (g)");
  Serial.println("#  peak_g               : Peak linear acceleration magnitude over last 0.5s (g)");
  Serial.println("#  temp_c               : Temperature in Celsius");
  Serial.println("#  shake_score          : 0..1000 (scaled from rms_g relative to accel range)");
  Serial.println("#  level/label          : 0..4 => Still, Light, Caution, Partial, Severe");

  if (SERIAL_FORMAT == 0) {
    Serial.println("time_ms,rms_g,peak_g,temp_c,shake_score,level,label");
  } else {
    Serial.println("# Format: key=value | key=value | ...");
  }

  resetWindow();
}

void loop() {
  static uint32_t lastSampleUs = micros();
  static uint32_t lastWindowMs = millis();
  static uint32_t lastProbeMs = 0;

  const uint32_t samplePeriodUs = (uint32_t)(1000000UL / SAMPLE_HZ); 

  if (g_shouldAdvertise) {
    g_shouldAdvertise = false;
    BLEDevice::startAdvertising();
  }

  uint32_t nowUs = micros();

  // Pacemaker to enforce accurate sample taking intervals
  if ((int32_t)(nowUs - lastSampleUs) >= (int32_t)samplePeriodUs) {
    
    // Manage timing lag to prevent processing stale data
    if ((nowUs - lastSampleUs) > (samplePeriodUs * 2)) {
      lastSampleUs = nowUs;
    } else {
      lastSampleUs += samplePeriodUs;
    }

    if (hasMpu) {
      float ax, ay, az;
      if (mpu.readAccelG(ax, ay, az)) {
        addSample(ax, ay, az);
      }
    }
  }

  const uint32_t nowMs = millis();

  // Occasional retry checks to handle hot-plugging missing sensors
  if ((uint32_t)(nowMs - lastProbeMs) >= 2000U) {
    if (!hasMpu || !hasSi7021) {
      lastProbeMs = nowMs;
      
      if (!hasMpu) {
        Serial.println("[Init] Re-trying MPU6050...");
        hasMpu = mpu.begin(Wire, -1, SAMPLE_HZ, ACCEL_RANGE, MPU_DLPF_CFG);
        if (hasMpu) {
          Serial.printf("[Init] MPU6050 found at 0x%02X\n", mpu.address());
        }
      }

      if (!hasSi7021) {
        if (si7021.begin(Wire, 0x40)) {
          hasSi7021 = true;
          Serial.println("[Init] Si7021 found (using ambient temperature)");
        }
      }
    }
  }

  // Pacemaker for the regular 0.5s broadcast window
  if ((int32_t)(nowMs - lastWindowMs) >= (int32_t)WINDOW_MS) {
    lastWindowMs += WINDOW_MS;
    publishWindow();
  }
}
