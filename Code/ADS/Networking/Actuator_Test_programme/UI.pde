
int sliderX = 50;
int sliderY = 50;
int gap = 75;
int buttonW = 50;

Slider A = new Slider(sliderX, sliderY, buttonW, buttonW, "A");
Slider B = new Slider(sliderX, sliderY + gap, buttonW, buttonW, "B");
Slider t = new Slider(sliderX, sliderY + 2* gap, buttonW, buttonW, "t");

Button Send = new Button(sliderX + 4*buttonW, sliderY, buttonW, buttonW, "Send");
Button Pause = new Button(sliderX + floor(5.5*buttonW), sliderY, buttonW, buttonW, "Pause");

Button Reset = new Button(sliderX + 7*buttonW, sliderY, buttonW, buttonW, "Reset");

Button Loop = new Button(sliderX + 4*buttonW, sliderY + gap, buttonW, buttonW, "Loop"); 
Button aButton = new Button(sliderX + floor(5.5*buttonW), sliderY + gap, buttonW, buttonW, "A"); 
Button bButton = new Button(sliderX + 7*buttonW, sliderY + gap, buttonW, buttonW, "B"); 

//------------------------------------- operational functions

void showUI(){
    A.show();
    B.show();
    t.show();

    Send.show();
    Pause.show();
    Reset.show();

    Loop.show();
    aButton.show();
    bButton.show();

    reset();
    send();
    pause();

    loop();
    sendA();
    sendB();
}//runs at the start of draw()

void clickUI(){
    A.click();
    B.click();
    t.click();

    Pause.click();
    Send.click();
    Reset.click();

    Loop.click();
    aButton.click();
    bButton.click();

}//runs inside mousePressed()

void typeUI(){
    A.type();
    B.type();
    t.type();
}//Runs inside keyPressed()

//------------------------------------- Functions for buttons 

void reset(){
    if(Reset.ready()){
        count = 0;
        command('r');
    }
}// function to run when reset button is clicked

void send(){
    if(Send.ready()){
        command('s');
        //send A, B, T values to arduino
        data(A.value);  //send A
        data(B.value);
        data(t.value);
    }
}// function to run when SEND button is clicked

void sendA(){
    if(aButton.ready()){
        command('a');
        data(A.value);  //send A
    }
}// function to run when A button is clicked

void sendB(){
    if(bButton.ready()){
        //command('b');
        println('B');
        data(B.value);  //send A
    }
}// function to run when B button is clicked

void pause(){
    if(Pause.toggled()){
        //tell actuator to pause / unpause
        command('p');
    }
}// function to run when Pause button is clicked

void loop(){
    if(Loop.toggled()){
        command('l');
    }
}// function to run when loop button is clicked

//---------------------------------- Networking functions
void command(char temp){
    R4.write(temp);
}//send Char

void data( int temp){
    R4.write(temp);
}//send Int
