# Steering Code Folder

This folder contains the steering, motor, sensor, arming, and test code used for **Rigby**.

## Folder Structure

```text
Code/Steering/
├── Steering-Final-Version/
│   └── WorkingS_PID_Final/
├── Test-Steering-Conor/
│   └── SteeringCode/
├── Motor/
│   ├── Encoder-Code/
│   ├── FT_TOF/
│   └── FT_Demo_Run/
├── tests/
│   └── LED_Button_Check/
└── arming_seq/
    └── arming_sequence/
```

## Sections

### `Steering-Final-Version/`

Contains the up-to-date working steering code.

Main folder:

* `WorkingS_PID_Final/`

This contains the current working PID steering implementation.

Main files:

* `WorkingS_PID_Final.ino` - main working steering PID sketch
* `Arming_seq_Steering_working.ino` - working arming sequence logic for the ESP32

---

### `Test-Steering-Conor/`

Contains Conor's steering code version, this is the version found in the FT Docs steering section.

Main folder:

* `SteeringCode/`

Subfolders:

* `GetPotentiometerReadings/` - tests potentiometer values
* `SteeringWithPotCode/` - steering code using pot 
* `TestSteeringCode/` - steering test sketch
* `new_steering_code/` - older steering code setup

---

### `Motor/`

Contains the main code for checking motor speed, encoder feedbacd & TOF code.

Subfolders:

* `Encoder-Code/` - encoder reading code used to monitor/check motor speed
* `FT_TOF/` - time-of-flight sensor code
* `FT_Demo_Run/` - demo run code for motor/steering testing

---

### `tests/`

Contains small test files.

Subfolders:

* `LED_Button_Check/` - tests RGB LED and button state changes

---

### `arming_seq/`

Contains standalone arming sequence code.

Subfolders:

* `arming_sequence/` - raw standalone arming sequence sketch

This is separate from * `Arming_seq_Steering_working.ino` that one is setup to work with steering and Pot 
