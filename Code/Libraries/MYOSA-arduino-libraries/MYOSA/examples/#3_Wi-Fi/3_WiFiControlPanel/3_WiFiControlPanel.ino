/*
  This code is developed under the MYOSA (LearnTheEasyWay) initiative of MakeSense EduTech and Pegasus Automation.
  Code has been derived from internet sources and component datasheets.
  Existing readily-available libraries would have been used "AS IS" and modified for ease of learning purpose.
  
  WiFi Control Panel
  Connection: Connect the "Actuator" board from the MYOSA kit with the "Controller" board power them up.
  Working: This example is intended to demonstrate the capabilities of WiFi (Controller) board. Here the controller board hosts to a  WebApp (viz a Control Panel for controlling the Actuator Board) on existing WiFi network. Hence, it lets users have a few controls to interact with the board from Mobile phone or any Digital Device through WebApp.

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

  You can design N number of such utility examples as a part of your learning from this kit.
  
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

/* Library Inclusion - WiFi.h is generic ESP32 library available */
#include <WiFi.h>
#include <Actuator.h>

/* Creating Object of Actuator class */
Actuator gpioExpander;

/* Enter your WiFi credentials here so that MYOSA Controller Board can connect to Internet. */
const char* ssid     = "MYOSAbyMakeSense";
const char* password = "LearnTheEasyWay";

/* Creating an Object of the WiFiServer class */
WiFiServer server(80);

/* Setup Function */
void setup()
{
  
  /* Setting up the communication */
  Serial.begin(115200);
  Wire.begin();
  Wire.setClock(100000);

  /* Initializing Actuator Board */
  for (;;)
  {
    if (gpioExpander.ping())
    {
      Serial.println("4bit IO Expander Actautor (PCA9536) is connected");
      break;
    }
    Serial.println("4bit IO Expander Actuator (PCA9536) is disconnected");
    delay(500u);
  }
  /* Set buzzer IO as output */
  gpioExpander.setMode(BUZZER_IO, IO_OUTPUT);
  gpioExpander.setState(BUZZER_IO, IO_LOW);

  pinMode(2, OUTPUT);      // set the LED pin mode

  delay(1000);


  /* Connecting to WiFi Network */
  Serial.println();
  Serial.println();
  Serial.print("Connecting to ");
  Serial.println(ssid);

  WiFi.begin(ssid, password);

  while (WiFi.status() != WL_CONNECTED) {
    delay(500);
    Serial.print(".");
  }

  Serial.println("");
  Serial.println("WiFi connected.");
  Serial.println("IP address: ");
  Serial.println(WiFi.localIP());

  server.begin();

}

/* Global Constants */
int value = 0;
bool clientPrinting = false;

/* Loop Function */
void loop() {

  /* Loop Function constantly check the availability of commands from the clients and takes desired action */
  WiFiClient client = server.available();   // listen for incoming clients

  if (client)
  {
    Serial.println("New Client.");           // print a message out the serial port
    String currentLine = "";                // make a String to hold incoming data from the client
    while (client.connected()) {            // loop while the client's connected
      if (client.available()) {             // if there's bytes to read from the client,
        char c = client.read();             // read a byte, then
        if (clientPrinting)
        {
          Serial.write(c);                    // print it out the serial monitor
        }
        if (c == '\n') {                    // if the byte is a newline character

          // if the current line is blank, you got two newline characters in a row.
          // that's the end of the client HTTP request, so send a response:
          if (currentLine.length() == 0) {
            // HTTP headers always start with a response code (e.g. HTTP/1.1 200 OK)
            // and a content-type so the client knows what's coming, then a blank line:
            client.println("HTTP/1.1 200 OK");
            client.println("Content-type:text/html");
            client.println();

            // the content of the HTTP response follows the header:
            client.println("<p style=\"font-size: 50px;text-align:center\">Welcome to the MYSOA Control Room</p>");

            client.println("\n\n<p style=\"font-size: 40px;text-align:center\">LED Control </p>\n\n");
            client.println("");
            client.println("");
            client.print(" <form action=\"H\" method=\"get\">  <p align=\"center\"> <button type=\"submit\" style=\"font-size:30px;height:200px;width:200px\" >Turn On</button> <button type=\"submit\" style=\"font-size:30px;height:200px;width:200px\" formaction=\"L\">Turn Off</button></form> </p>");

            client.println("");
            client.println("\n\n");
            client.println("\n\n<p style=\"font-size: 40px;text-align:center\">Buzzer Control </p>\n\n");
            client.println("");
            client.println("");
            client.print(" <form action=\"X\" method=\"get\">  <p align=\"center\"> <button type=\"submit\" style=\"font-size:30px;height:200px;width:200px\" >Turn On</button> <button type=\"submit\" style=\"font-size:30px;height:200px;width:200px\" formaction=\"Y\">Turn Off</button></form> </p>");

            //            client.print("<br> <form class=\"form-inline\" method=\"GET\" action=\"/H\"> <button type=\"submit\">Turn ON</button> method=\"GET\" action=\"/L\"> <button type=\"submit\">Turn OFF</button></form> </br>");
            //            client.print("<form method=\"GET\" action=\"/L\"> <button type=\"submit\">Turn OFF</button></form> ");

            // The HTTP response ends with another blank line:
            client.println();
            // break out of the while loop:
            break;
          } else {    // if you got a newline, then clear currentLine:
            currentLine = "";
          }
        } else if (c != '\r') {  // if you got anything else but a carriage return character,
          currentLine += c;      // add it to the end of the currentLine
        }

        // Check to see if the client request was "GET /H" or "GET /L":
        if (currentLine.endsWith("GET /H")) {
          Serial.println("Requested LED to turn ON!");
          digitalWrite(2, HIGH);               // GET /H turns the LED on
        }
        if (currentLine.endsWith("GET /L")) {
          Serial.println("Requested LED to turn OFF!");
          digitalWrite(2, LOW);                // GET /L turns the LED off
        }

        // Check to see if the client request was "GET /H" or "GET /L":
        if (currentLine.endsWith("GET /X")) {
          Serial.println("Requested Buzzer to turn ON!");
          gpioExpander.setState(BUZZER_IO, IO_HIGH);
//          digitalWrite(2, HIGH);               // GET /H turns the LED on
        }
        if (currentLine.endsWith("GET /Y")) {
          Serial.println("Requested Buzzer to turn OFF!");
          gpioExpander.setState(BUZZER_IO, IO_LOW);
//          digitalWrite(2, LOW);                // GET /L turns the LED off
        }
        
      }
    }
    // close the connection:
    client.stop();
    Serial.println("Client Disconnected.");
  }
}
