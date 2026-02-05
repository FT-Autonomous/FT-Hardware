#I used this don't sue me: https://forum.arduino.cc/t/establish-connection-to-send-serial-data-to-arduino-using-a-little-python-script/613997
# another useful link: https://stackoverflow.com/questions/27183378/sending-serial-communication-using-python-on-ubuntu-to-arduino
#note to self: you could import time library to give the ardiuno a chance to chill

import serial  

ser = serial.Serial('/dev/ttyACM0', 9600, timeout=1) 

if ser.is_open:
  print("Rigby controller active") #safety check
else:
 print("Try again") 
 exit()

ser.write(b'<Sup Bruz>\n')
print("Message sent.")  
ser.close()


