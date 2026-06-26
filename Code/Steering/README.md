Steering Code Folder
This folder contains the steering, motor, sensor, arming, and some test code used for Rigby
Folder Structure
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

Sections
Steering-Final-Version/
Contains the Up to date working steering code.
Main folder:
    • WorkingS_PID_Final/
This contains the current working PID steering implementation.
Main files:
    • WorkingS_PID_Final.ino - main working steering PID sketch
    • Arming_seq_Steering_working.ino – working arming seq logic for ESP-32 

Test-Steering-Conor/
Contains Conor's steering code version (The one found on the FT docs steering section)
Main folder:
    • SteeringCode/
Subfolders:
    • GetPotentiometerReadings/ -> tests potentiometer values.
    • SteeringWithPotCode/ -> Working steering code using potentiometer feedback.
    • TestSteeringCode/ -> steering test sketch.
    • new_steering_code/ → Older steering code setup.
This folder is kept as a separate reference/development version.

Motor/
Contains The main code for checking motor speed, encoder, TOF, and demo-run code.
Subfolders:
    • Encoder-Code/ - encoder reading code, This controls speed and checks current speed.
    • FT_TOF/
    • FT_Demo_Run/ - demo run code for motor/steering testing

tests/
Contains small test sketches.
Subfolders:
    • LED_Button_Check/ - Tests RGB LED + Button for state change

arming_seq/
Contains standalone arming sequence code.
Subfolders:
    • arming_sequence/ - raw arming sequence sketch
This is separate from the steering versions so the arming logic can be tested independently.
