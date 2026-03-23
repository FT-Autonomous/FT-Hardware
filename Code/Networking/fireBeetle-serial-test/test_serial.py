# FireBeetle ESP32 Serial Test
# The FireBeetle uses a CH340 USB-to-serial chip, which shows up as /dev/ttyUSB0 on Linux
# (not /dev/ttyACM0 like native-USB Arduino boards).
#
# If you have multiple USB-serial devices, check with: ls /dev/ttyUSB*
# You may need to install CH340 drivers: sudo apt install ch341-uart-dkms
#
# Refs:
#   https://www.dfrobot.com/product-1590.html
#   https://forum.arduino.cc/t/establish-connection-to-send-serial-data-to-arduino-using-a-little-python-script/613997

import serial
import time

SERIAL_PORT = '/dev/ttyUSB0'
BAUD_RATE = 115200

ser = serial.Serial(SERIAL_PORT, BAUD_RATE, timeout=1)
time.sleep(2)  # ESP32 resets on serial connect — give it time to boot

if ser.is_open:
    print("FireBeetle serial connection active")
else:
    print("Connection failed — check port with: ls /dev/ttyUSB*")
    exit()

ser.write(b'Sup Bruz\n')
print("Message sent.")

# Wait for and print the response
time.sleep(0.1)
while ser.in_waiting:
    response = ser.readline().decode().strip()
    if response:
        print("Response:", response)

ser.close()
