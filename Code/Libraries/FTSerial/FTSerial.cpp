#include "FTSerial.h"

FTSerial::FTSerial(Stream &serial, byte bufferSize)
    : _serial(serial), _bufSize(bufferSize), _ndx(0), _reading(false)
{
    _buf = new char[bufferSize];
}

FTSerial::~FTSerial() {
    delete[] _buf;
}

String FTSerial::readUntilNewline() {
    while (_serial.available() > 0) {
        char c = _serial.read();

        if (c == '\n') {
            _buf[_ndx] = '\0';
            _ndx = 0;
            return String(_buf);
        }

        if (c != '\r') {
            _buf[_ndx++] = c;
            if (_ndx >= _bufSize) {
                _ndx = _bufSize - 1;
            }
        }
    }
    return "";
}

String FTSerial::readWithMarkers(char startMarker, char endMarker) {
    while (_serial.available() > 0) {
        char c = _serial.read();

        if (_reading) {
            if (c != endMarker) {
                _buf[_ndx++] = c;
                if (_ndx >= _bufSize) {
                    _ndx = _bufSize - 1;
                }
            } else {
                _buf[_ndx] = '\0';
                _reading = false;
                _ndx = 0;
                return String(_buf);
            }
        } else if (c == startMarker) {
            _reading = true;
            _ndx = 0;
        }
    }
    return "";
}

bool FTSerial::readFloat(float &result) {
    String line = readUntilNewline();
    if (line.length() == 0) return false;

    result = line.toFloat();
    return true;
}
