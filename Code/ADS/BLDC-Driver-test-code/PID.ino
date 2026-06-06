// currently the Serial takes a PWM value and send it to the motor
// now we want to send an RPM to the arduino, for it to send PWM values to the motor to try and attain that RPM

void PID() {
  //just going to make a basic Proportional controller.
  int pVal = 15;
  int margin = 200;

  updateRPM();
  int curr = floor(currentRPM);

  if (targetRPM == 0) {
    pwmValue = 0;
    return;
  }

  if (abs(targetRPM - curr) > margin) {  // if the difference between target and curr is greater than acceptable margin
    if (targetRPM > curr) {
      pwmValue = pwmValue + pVal;
    } else
      pwmValue = pwmValue - pVal / 15;
    
    limPWM();
  }

  if (abs(targetRPM - curr) > margin / 2) {
    if (targetRPM > curr) {
      pwmValue = pwmValue + pVal / 2;
    } else
      pwmValue = pwmValue - pVal / 30;
  }
  limPWM();
}

void limPWM() {
  pwmValue = constrain(pwmValue, 0, 255);
  Serial.print("PWM set to: ");
  Serial.println(pwmValue);
}  // Constrain value to valid PWM range