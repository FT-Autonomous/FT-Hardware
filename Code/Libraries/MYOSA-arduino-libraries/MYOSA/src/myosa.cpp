/*
  This code is developed under the MYOSA (LearnTheEasyWay) initiative of MakeSense EduTech and Pegasus Automation.
  Code has been derived from internet sources and component datasheets.
  Existing readily-available libraries would have been used "AS IS" and modified for ease of learning purpose.
  
  Synopsis of MYOSA platform
  MYOSA Platform consists of a centralized motherboard a.k.a Controller board, 5 different sensor modules, an OLED display and an actuator board in the kit.
  Controller board is designed on ESP32 module. It is a low-power system on a chip microcontrollers with integrated Wi-Fi and Bluetooth.
  5 Sensors are as below,
  1 --> Accelerometer and Gyroscope (6-axis motion sensor)
  2 --> Temperature and Humidity Sensor
  3 --> Barometric Pressure Sensor
  4 --> Light, Proximity and Gesture Sensor
  5 --> Air Quality Sensor
  Actuator board contains a Buzzer and an AC switching circuit to turn on/off an electrical appliance.
  There is also an OLED display in the MYOSA kit.
  Detailed Information about MYOSA platform and usage is provided in the link below.
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

#include "myosa.h"
#include <BLEDevice.h>
#include <BLE2902.h>

#include <iostream>

#define NUM_SERVICES          6
#define MAX_CHARACTERISTICS   5
#define MAX_EVENTS            2

BLECharacteristic *pCharacteristics[NUM_SERVICES][MAX_CHARACTERISTICS];

struct eventData {
  bool isEnable = false;
  int serviceNumber, charNumber, paraNumber;
  double min, max;
  bool isInclusive, isStrict;
  char action;
};

int eventCnt = 0;
eventData eventArr[MAX_EVENTS];

class MyServerCallbacks: public BLEServerCallbacks {
    void onConnect(BLEServer* pServer) {
      Serial.println("Device connected");
    }

    void onDisconnect(BLEServer* pServer) {
      Serial.println("Device disconnected");
      pServer->getAdvertising()->start();
      Serial.println("BLE server re-started!");
    }
};

char parseEventType(const String& input, String& remainingString)
{
  char separator = ',';

  int separatorIndex = input.indexOf(separator);
  char firstChar = input.charAt(0);
  remainingString = input.substring(separatorIndex+1);

  // Serial.print("First character: ");
  // Serial.println(firstChar);
  // Serial.print("Remaining string: ");
  // Serial.println(remainingString);

  return firstChar;
}

void createEvent(const String& input) 
{
  int inputIndex = 0;
  int outputIndex = 0;
  String value;
  eventData tempEvent;

  tempEvent.isEnable = true;

  while (input[inputIndex] != '\0') {
    if (input[inputIndex] != ',') {
      value += input[inputIndex];
    }
    else {
      switch (outputIndex) {
        case 0:
          tempEvent.serviceNumber = atoi(value.c_str());
          // data.serviceNumber = atoi(value.c_str());
          break;
        case 1:
          tempEvent.charNumber = atoi(value.c_str());
          // data.charNumber = atoi(value.c_str());
          break;
        case 2:
          tempEvent.paraNumber = atoi(value.c_str());
          // data.paraNumber = atoi(value.c_str());
          break;
        case 3:
          tempEvent.min = atof(value.c_str());
          // data.min = atof(value.c_str());
          break;
        case 4:
          tempEvent.max = atof(value.c_str());
          // data.max = atof(value.c_str());
          break;
        case 5:
          tempEvent.isInclusive = (value[0] == '1');
          // data.isInclusive = (value[0] == '1');
          break;
        case 6:
          tempEvent.isStrict = (value[0] == '1');
          // data.isStrict = (value[0] == '0');
          break;
      }
      outputIndex++;
      value = "";
    }
    inputIndex++;
  }
  if(outputIndex == 7) 
  {
    tempEvent.action = value.c_str()[0];
    // data.action = value.c_str()[0];
  }

  if (tempEvent.action == 'b')
  {
    eventArr[0] = tempEvent;
  }
  else
  {
    eventArr[1] = tempEvent;
  }
}

void deleteEvent(const String& input)
{
  if (input[0] == 'b')
  {
    eventArr[0].isEnable = false;
  }
  else
  {
    eventArr[1].isEnable = false;
  }
}

void printData(const eventData& data) {
  Serial.print("isEnable: ");
  Serial.println(data.isEnable);
  Serial.print("serviceNumber: ");
  Serial.println(data.serviceNumber);
  Serial.print("charNumber: ");
  Serial.println(data.charNumber);
  Serial.print("paraNumber: ");
  Serial.println(data.paraNumber);
  Serial.print("min: ");
  Serial.println(data.min);
  Serial.print("max: ");
  Serial.println(data.max);
  Serial.print("isInclusive: ");
  Serial.println(data.isInclusive);
  Serial.print("isStrict: ");
  Serial.println(data.isStrict);
  Serial.print("action: ");
  Serial.println(data.action);
  Serial.println();
}


class MyCallbacks: public BLECharacteristicCallbacks {
    void onWrite(BLECharacteristic *pCharacteristic) {
      std::string newValue = pCharacteristic->getValue().c_str();

      if (newValue.length() > 0) 
      {
        Serial.println("*********");
        Serial.print("New value: ");
        Serial.println(newValue.c_str());
        Serial.println();
        Serial.println("*********");

        String payload;
        char eventType;
        
        eventType = parseEventType(newValue.c_str(), payload);

        // Serial.println(payload);

        switch(eventType)
        {
          case 'c':
            createEvent(payload);
            eventCnt++;
            break;
          case 'u':
            createEvent(payload);
            break;
          case 'd':
            deleteEvent(payload);
            eventCnt--;
            break;
          default:
            break;
        }
        
        for (int i = 0; i < MAX_EVENTS; i++)
        {
          printData(eventArr[i]);
        }
      }
    }
};

MYOSA::MYOSA():Ag(MPU6050_ADDRESS_AD0_HIGH), /* Accelerometer and Gyroscope sensor init */
Aq(CCS811_I2C_ADDRESS1,refResitance), /* Air Quality sensor init */
Pr(ULTRA_HIGH_RESOLUTION), /* Barometric Pressure sensor init */
Lpg(), /* Light, Proximity and Gesture sensor init */
gpioExpander(), /* GPIO expander PCA9536 init */
Th(), /* temperature and Humidity sensor init */
display(SCREEN_WIDTH, SCREEN_HEIGHT, &Wire, OLED_RESET)  /* OLED display */
{
}

/**
 *
 */
bool MYOSA::begin(void)
{
  BLEDevice::init("MYOSA_1");
  BLEServer *pServer = BLEDevice::createServer();
  pServer->setCallbacks(new MyServerCallbacks());

  for (int i = 0; i < NUM_SERVICES; i++)
  {
    char serviceUUID[37];
    int charIterator = 0;
    
    sprintf(serviceUUID, "4fafc201-1fb5-459e-8fcc-c5c9c33191b%d", i);
    Serial.println(serviceUUID);

    BLEService *pService = pServer->createService(serviceUUID);

    if(i == 0)
    {
      charIterator = 4;
    }
    else if(i == 1)
    {
      charIterator = 2;
    }
    else if(i == 5)
    {
      charIterator = 1;
    }
    else
    {
      charIterator = 3;
    }

    for (int j = 0; j < charIterator; j++)
    {
      char characteristicUUID[37];
      sprintf(characteristicUUID, "beb5483e-36e1-4688-b7f5-ea07361b2b%d%d", i, j);
      Serial.println(characteristicUUID);
      if(i == 5 && j == 0)
      {
        pCharacteristics[i][j] = pService->createCharacteristic(
                      characteristicUUID,
                      BLECharacteristic::PROPERTY_READ |
                      BLECharacteristic::PROPERTY_WRITE
                    );
        pCharacteristics[i][j]->setCallbacks(new MyCallbacks());

        pCharacteristics[i][j]->setValue("Hello from MYOSA");
      }
      else
      {
        pCharacteristics[i][j] = pService->createCharacteristic(
                      characteristicUUID,
                      BLECharacteristic::PROPERTY_NOTIFY
                    );
        pCharacteristics[i][j]->addDescriptor(new BLE2902()); 
      }
    }

    pService->start();
  }

  pServer->getAdvertising()->start();
  Serial.println("BLE server started!");

  if(display.begin() == true)
  {
    Serial.println("OLED initializated");
  }
  if(Ag.begin() == true)
  {
    Serial.println("AccelAndGyro initializated");
  }
  if(Aq.begin() == true)
  {
    Serial.println("AirQuality initializated");
  }
  if(Pr.begin() == true)
  {
    Serial.println("BarometricPressure initializated");
  }
  if(Lpg.begin() == true)
  {
    Serial.println("LightProximityAndGesture initializated");
    if( Lpg.enableAmbientLightSensor(DISABLE) )
    {
      Serial.println("Light sensor is now running");
    }
    if( Lpg.enableProximitySensor(DISABLE) )
    {
      Serial.println("Proximity sensor is now running");
    }
    /* Adjust the Proximity sensor gain */
    if ( !Lpg.setProximityGain(PGAIN_2X) )
    {
      Serial.println("Something went wrong trying to set PGAIN");
    }
  }
  if(gpioExpander.ping() == true)
  {
    /* Set relay IO as output */
    gpioExpander.setMode(RELAY_IO, IO_OUTPUT);
    gpioExpander.setState(RELAY_IO, IO_LOW);
    /* Set buzzer IO as output */
    gpioExpander.setMode(BUZZER_IO, IO_OUTPUT);
    gpioExpander.setState(BUZZER_IO, IO_LOW);
    Serial.println("gpioExpander initializated");
  }
  if(Th.begin() == true)
  {
    Serial.println("TempAndHumidity initializated");
  }

  return true;
}

/**
 *
 */
void MYOSA::turnOnRelay(void)
{
  gpioExpander.setMode(RELAY_IO, IO_OUTPUT);
  gpioExpander.setState(RELAY_IO, IO_HIGH);
}

/**
 *
 */
void MYOSA::turnOffRelay(void)
{
  gpioExpander.setMode(RELAY_IO, IO_OUTPUT);
  gpioExpander.setState(RELAY_IO, IO_LOW);
}

/**
 *
 */
void MYOSA::turnOnBuzzer(int print)
{
  gpioExpander.setMode(BUZZER_IO, IO_OUTPUT);
  gpioExpander.setState(BUZZER_IO, IO_HIGH);

  if (print)
  { 
    display.clearDisplay();
    display.setTextSize(1);
    display.setTextColor(WHITE);
    display.setFont(&VeraMonoBold7pt7b);
    display.setCursor(0, 9);
    drawCentreString("Actuator");
    if(gpioExpander.ping())
    {
      display.setFont(&VeraMono7pt7b);
      display.print("Buzzer: Turn On");
      display.println();
      display.display();
    }
  }
}

/**
 *
 */
void MYOSA::turnOffBuzzer(int print)
{
  gpioExpander.setMode(BUZZER_IO, IO_OUTPUT);
  gpioExpander.setState(BUZZER_IO, IO_LOW);


  // display.clearDisplay();
  display.setTextSize(1);
  display.setTextColor(WHITE);
  display.setFont(&VeraMonoBold7pt7b);
  // display.setCursor(0, 9);
  // drawCentreString("Actuator");
  if(gpioExpander.ping())
  {
    display.setFont(&VeraMono7pt7b);
  display.print("Buzzer: Turn Off");
    display.println();
    display.display();
  }

}
/**
 *
 */
void MYOSA::drawCentreString(const String &buf)
{
    int16_t x1, y1;
    uint16_t w, h;
    display.getTextBounds(buf, display.getCursorX(), display.getCursorY(), &x1, &y1, &w, &h); //calc width of new string
    display.setCursor((SCREEN_WIDTH - w) / 2, y1+h);
    display.println(buf);
}

/**
 *
 */
void MYOSA::drawSubscriptSymbol(const String &buf)
{
  display.setFont(nullptr);
  display.setCursor(display.getCursorX(), display.getCursorY()+3);
  display.print(buf);
  display.setCursor(display.getCursorX(), display.getCursorY()-3);
  display.setFont(&VeraMono7pt7b);
}

/**
 *
 */
void MYOSA::drawSuperscriptSymbol(const String &buf)
{
  display.setFont(nullptr);
  display.setCursor(display.getCursorX(), display.getCursorY()-6);
  display.print(buf);
  display.setCursor(display.getCursorX(), display.getCursorY()+6);
  display.setFont(&VeraMono7pt7b);
}

/**
 *
 */
void MYOSA::drawDegreeSymbol(void)
{
  display.drawCircle(display.getCursorX()+4,display.getCursorY()-8,2,1);
  display.setCursor(display.getCursorX()+6, display.getCursorY());
}

/**
 * 
 */
// void MYOSA::findIndex(void)
// {

// }

/**
 * 
 */
void MYOSA::sendBleData(void)
{
  for (int i = 0; i < NUM_SERVICES - 1; i++)
  {
    int charIterator = 0;
    char payload[25];

    payload[0] = '*';

    if(i == 0)
    {
      charIterator = 4;
    }
    else if(i == 1)
    {
      charIterator = 2;
    }
    else
    {
      charIterator = 3;
    }

    for (int j = 0; j < charIterator; j++)
    {
      if (Ag.ping())
      {
        if (i == 0 && j == 0)
        {
          sprintf(payload, "%0.2f, %0.2f, %0.2f", Ag.getAccelX(), Ag.getAccelY(), Ag.getAccelZ());
        }
        else if (i == 0 && j == 1)
        {
          sprintf(payload, "%0.2f, %0.2f, %0.2f", Ag.getGyroX(), Ag.getGyroY(), Ag.getGyroZ());
        }
        else if (i == 0 && j == 2)
        {
          sprintf(payload, "%0.2f, %0.2f, %0.2f", Ag.getTiltX(), Ag.getTiltY(), Ag.getTiltZ());
        }
        else if (i == 0 && j == 3)
        {
          sprintf(payload, "%0.2f, %0.2f", Ag.getTempC(), Ag.getTempF());
        } 
      }
      if (Aq.ping())
      {
        if (i == 1 && j == 0)
        {
          sprintf(payload, "%d", Aq.getCO2());
        }
        else if (i == 1 && j == 1)
        {
          sprintf(payload, "%d", Aq.getTVOC());
        } 
      }
      if (Pr.ping())
      {
        if (i == 2 && j == 0)
        {
          sprintf(payload, "%0.2f, %0.2f", Pr.getTempC(), Pr.getTempF());
        }
        else if (i == 2 && j == 1)
        {
          sprintf(payload, "%0.2f, %0.2f, %0.2f", Pr.getPressurePascal(), Pr.getPressureHg(), Pr.getPressureBar());
        }
        else if (i == 2 && j == 2)
        {
          sprintf(payload, "%0.2f", Pr.getAltitude(SEA_LEVEL_AVG_PRESSURE));
        } 
      }
      if (Lpg.ping())
      {
        if (i == 3 && j == 0)
        {
          sprintf(payload, "%d", Lpg.getAmbientLight());
        }
        else if (i == 3 && j == 1)
        {
          sprintf(payload, "%0.2f", Lpg.getProximity());
        }
        else if(i == 3 && j == 2)
        {
          sprintf(payload, "%d, %d, %d", Lpg.getRedProportion(), Lpg.getGreenProportion(), Lpg.getBlueProportion());
        } 
      }
      if (Th.ping())
      {
        if (i == 4 && j == 0)
        {
          sprintf(payload, "%0.2f, %0.2f", Th.getTempC(), Th.getTempF());
        }
        else if (i == 4 && j == 1)
        {
          sprintf(payload, "%0.2f", Th.getRelativeHumdity());
        }
        else if (i == 4 && j == 2)
        {
          sprintf(payload, "%0.2f, %0.2f", Th.getHeatIndexC(), Th.getHeatIndexF());
        } 
      }

      if (payload[0] != '*')
      {
        Serial.print("Notifying value of characteristic ");
        Serial.print(j);
        Serial.print(" in service ");
        Serial.print(i);
        Serial.print(": ");
        Serial.println(payload);
        pCharacteristics[i][j]->setValue(payload);
        pCharacteristics[i][j]->notify(); 
      }
    }
    if (payload[0] != '*')
    {
      Serial.println();
    }
  }

  //Check for events 
  for (int i = 0; i < MAX_EVENTS; i++)
  {
    if (eventArr[i].isEnable)
    {
      double val;
      int cnt = 0;
    std::string value = pCharacteristics[eventArr[i].serviceNumber][eventArr[i].charNumber]->getValue().c_str();
    Serial.println(value.c_str());


      char* token = strtok((char*)value.c_str(), ",");

      while (token != NULL)
      {
        val = atof(token);
        if (eventArr[i].paraNumber == cnt)
        {
          break;
        }
        else
        {
          token = strtok(NULL, ",");
        }
        cnt++;
      }
      Serial.println(val);

      if (eventArr[i].isInclusive)
      {
        if (eventArr[i].isStrict)
        {
          if (val >= eventArr[i].min && val <= eventArr[i].max)
          {
            if (eventArr[i].action == 'b')
            {
              turnOnBuzzer(0);
            }
            else
            {
              turnOnRelay();
            }
          }
          else
          {
            if (eventArr[i].action == 'b')
            {
              turnOffBuzzer(0);
            }
            else
            {
              turnOffRelay();
            } 
          }
        }
        else
        {
          if (val > eventArr[i].min && val < eventArr[i].max)
          {
            if (eventArr[i].action == 'b')
            {
              turnOnBuzzer(0);
            }
            else
            {
              turnOnRelay();
            }
          }
          else
          {
            if (eventArr[i].action == 'b')
            {
              turnOffBuzzer(0);
            }
            else
            {
              turnOffRelay();
            } 
          } 
        }
      }
      else
      {
        if (eventArr[i].isStrict)
        {
          if (val <= eventArr[i].min && val >= eventArr[i].max)
          {
            if (eventArr[i].action == 'b')
            {
              turnOnBuzzer(0);
            }
            else
            {
              turnOnRelay();
            }
          }
          else
          {
            if (eventArr[i].action == 'b')
            {
              turnOffBuzzer(0);
            }
            else
            {
              turnOffRelay();
            } 
          }
        }
        else
        {
          if (val < eventArr[i].min && val > eventArr[i].max)
          {
            if (eventArr[i].action == 'b')
            {
              turnOnBuzzer(0);
            }
            else
            {
              turnOnRelay();
            }
          }
          else
          {
            if (eventArr[i].action == 'b')
            {
              turnOffBuzzer(0);
            }
            else
            {
              turnOffRelay();
            } 
          } 
        } 
      } 
    }
  }
}

/**
 *
 */
void MYOSA::printAceelAndGyro(void)
{
  static uint8_t nCnt=0;
  display.clearDisplay();
  display.setTextSize(1);
  display.setTextColor(WHITE);
  display.setFont(&VeraMonoBold7pt7b);
  display.setCursor(0, 9);
  drawCentreString("AccelGyro: Raw");
  if(Ag.ping())  {
   display.setFont(&VeraMono7pt7b);
    if(nCnt == 0u)
    {
      display.print("aX:");
      display.print(Ag.getAccelX(),1);
      display.print("cm/s");
      drawSuperscriptSymbol("2");
      display.println();

      display.print("aY:");
      display.print(Ag.getAccelY(),1);
      display.print("cm/s");
      drawSuperscriptSymbol("2");
      display.println();

      display.print("aZ:");
      display.print(Ag.getAccelZ(),1);
      display.print("cm/s");
      drawSuperscriptSymbol("2");
    }
    else if(nCnt == 1u)
    {
      display.print("gX:");
      display.print(Ag.getGyroX(),1);
      drawDegreeSymbol();
      display.println("/s");
      display.print("gY:");
      display.print(Ag.getGyroY(),1);
      drawDegreeSymbol();
      display.println("/s");
      display.print("gZ:");
      display.print(Ag.getGyroZ(),1);
      drawDegreeSymbol();
      display.println("/s");
    }
    else if(nCnt == 2u)
    {
      display.print("tiltX:");
      display.print(Ag.getTiltX(),1);
      drawDegreeSymbol();
      display.println();
      display.print("tiltY:");
      display.print(Ag.getTiltY(),1);
      drawDegreeSymbol();
      display.println();
      display.print("tiltZ:");
      display.print(Ag.getTiltZ(),1);
      drawDegreeSymbol();
      display.println();
    }
    else
    {
      display.print("Temp:");
      display.print(Ag.getTempC(),1);
      drawDegreeSymbol();
      display.println("C");
      display.print("Temp:");
      display.print(Ag.getTempF(),1);
      drawDegreeSymbol();
      display.println("F");
    }
  }
  display.display();
  nCnt = (nCnt+1)&3;
}

/**
 *
 */
void MYOSA::printAirQuality(void)
{
  display.clearDisplay();
  display.setTextSize(1);
  display.setTextColor(WHITE);
  display.setFont(&VeraMonoBold7pt7b);
  display.setCursor(0, 9);
  drawCentreString("Air Quality");
  if(Aq.ping())
  {
    /* Check if data is ready or not */
    if(Aq.isDataAvailable())
    {
      if(Aq.readAlgorithmResults() == SENSOR_SUCCESS)
      {
        display.setFont(&VeraMono7pt7b);
        display.print("eCO2 :");
        display.print(Aq.getCO2());
        display.println("ppm");
        display.print("TVOC :");
        display.print(Aq.getTVOC());
        display.println("ppb");
      }
    }
  }
  display.display();
}

/**
 *
 */
void MYOSA::printBarometricPressure(void)
{
  static uint8_t nCnt=0;
  display.clearDisplay();
  display.setTextSize(1);
  display.setTextColor(WHITE);
  display.setFont(&VeraMonoBold7pt7b);
  display.setCursor(0, 9);
  drawCentreString("Pressure");
  if(Pr.ping())
  {
    display.setFont(&VeraMono7pt7b);
    if(nCnt==0u)
    {
      display.print("Temp:");
      display.print(Pr.getTempC(),1);
      drawDegreeSymbol();
      display.println("C");
      display.print("Temp:");
      display.print(Pr.getTempF(),1);
      drawDegreeSymbol();
      display.println("F");
      display.print("Alti:");
      display.print(Pr.getAltitude(SEA_LEVEL_AVG_PRESSURE),1);
      display.println("m");
    }
    else
    {
      display.print("Pres:");
      display.print(Pr.getPressurePascal(),1);
      display.println("kPa");
      display.print("Pres:");
      display.print(Pr.getPressureHg(),1);
      display.println("mmHg");
      display.print("Pres:");
      display.print(Pr.getPressureBar(),1);
      display.println("mbar");
    }
  }
  display.display();
  nCnt = (nCnt+1)&1;
}

/**
 *
 */
void MYOSA::printLightProximityAndGesture(void)
{
  static uint8_t nCnt=0;
  display.clearDisplay();
  display.setTextSize(1);
  display.setTextColor(WHITE);
  display.setFont(&VeraMonoBold7pt7b);
  display.setCursor(0, 9);
  drawCentreString("Light Prox RGB");
  if(Lpg.ping())
  {
    display.setFont(&VeraMono7pt7b);
    if(nCnt==0u)
    {
      display.print("Ambient:");
      display.print(Lpg.getAmbientLight());
      display.println("Lux");
      display.print("Proximity:");
      display.print(Lpg.getProximity(),1);
      display.println();
    }
    else
    {
      display.print("Red  :");
      display.print(Lpg.getRedProportion());
      display.println("%");
      display.print("Green:");
      display.print(Lpg.getGreenProportion());
      display.println("%");
      display.print("Blue :");
      display.print(Lpg.getBlueProportion());
      display.println("%");
    }
  }
  display.display();
  nCnt = (nCnt+1)&1;
}

/**
 *
 */
void MYOSA::printTempAndHumidity(void)
{
  static uint8_t nCnt=0;
  display.clearDisplay();
  display.setTextSize(1);
  display.setTextColor(WHITE);
  display.setFont(&VeraMonoBold7pt7b);
  display.setCursor(0, 9);
  drawCentreString("Temp Humidity");
  if(Th.ping())
  {
    display.setFont(&VeraMono7pt7b);
    if(nCnt==0u)
    {
      display.print("RH  :");
      display.print(Th.getRelativeHumdity(),1);
      display.println("%");
      display.print("Temp:");
      display.print(Th.getTempC(),1);
      drawDegreeSymbol();
      display.println("C");
      display.print("Temp:");
      display.print(Th.getTempF(),1);
      drawDegreeSymbol();
      display.println("F");
    }
    else
    {
      display.print("HI :");
      display.print(Th.getHeatIndexC(),1);
      drawDegreeSymbol();
      display.println("C");
      display.print("HI :");
      display.print(Th.getHeatIndexF(),1);
      drawDegreeSymbol();
      display.println("F");
    }
  }
  display.display();
  nCnt = (nCnt+1)&1;
}
