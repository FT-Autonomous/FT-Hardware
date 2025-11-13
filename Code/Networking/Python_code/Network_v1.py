#I used this don't sue me: https://forum.arduino.cc/t/establish-connection-to-send-serial-data-to-arduino-using-a-little-python-script/613997
# another useful link: https://stackoverflow.com/questions/27183378/sending-serial-communication-using-python-on-ubuntu-to-arduino
#note to self: you could import time library to give the ardiuno a chance to chill

import serial  
import time

ser = serial.Serial('/dev/ttyACM1', 9600, timeout=1) 

def ASSI_Mode(x):
    # Hex and Char  (moved the case statement from arduino to here)
    command_map = {
        "O": '0',  # Off (0000)
        "R": '1',  # Ready  (0001)
        "D": '2',  # Drive (0011)
        "F": '3',  # Finished (0100)
        "E": '4'  # Emergency (1000)
        #"M": '5',  # Manual Overide (ignore these 2 for now)
        #"N": '6'   # Not Ready
    }

    #If im right (Char -> hex -> int -> binary -> mode -> W)


    if x in command_map:
        hex_char = command_map[x]   # Map out the char that was just sent (we convert char -> hex in python and then it fucks off to the arduino)
        #print("Sending:", x,"In char", hex_char, "In int:", int(hex_char))
        ser.write(b'hex_char')
        res = []
        while ser.in_waiting: #while arduino fuckes with the byte
            res.append(ser.readline().strip())
        return res #Arduino responds
    else:
        print("Pick a right command dingus")
    
    time.sleep(0.1)
    #data = ser.readline().decode().strip()  
    #return data

while True:
    mode = input("Enter Mode (O,R,D,F,E): ").upper().strip()
    value = ASSI_Mode(mode) 
    #ser.write(value.encode())
    print("Response:", value)
    time.sleep(0.1)
    #for line in value:
       # print("Arduino: ", line)
  


#To DO
# Big one: Kill myself
# Another one: Ensure that the value entred is consatnly updated so the arduino can

#if ser.is_open:
#  print("Rigby controller active") #safety check
#else:
# print("Try again") 
# exit()

#ser.write(b'Sup Bruz\n')
#print("Message sent.")  
#ser.close()


