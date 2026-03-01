#ifndef FT_SERIAL_H
#define FT_SERIAL_H

#include <Arduino.h>

class FTSerial {
public:
    FTSerial(Stream &serial, byte bufferSize = 32);
    ~FTSerial();

    // read until '\n' (ignores '\r'). returns "" if no complete line yet
    String readUntilNewline();

    // read between start/end markers (default < >). returns "" if no complete message yet
    String readWithMarkers(char startMarker = '<', char endMarker = '>');

    // reads a line via readUntilNewline() and parses it as a float
    // returns true if a new value was received, and sets result to the parsed float
    bool readFloat(float &result);

private:
    Stream &_serial;
    char  *_buf;
    byte   _bufSize;
    byte   _ndx;
    bool   _reading; // state for marker mode
};

#endif
