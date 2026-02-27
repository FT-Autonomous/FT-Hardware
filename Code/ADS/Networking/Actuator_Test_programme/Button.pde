
class Button {
  int x, y, w, h;

  //default color
  color colour = color(150, 100, 255);
  color togglecolor = color(0,50,200);
  color txtColor = color(255);

  String name = new String(" ");
  int txtSize;

  boolean customImage = false;
  boolean noFill = false;
  boolean toggle = false;
  boolean prevToggle = false;
  PImage display;


  //-------------------------------------------------------------------------------

  Button(int X, int Y, int W, int H) {
    x = X;
    y = Y;
    w = W;
    h = H;
    txtSize = floor(h/2);
  }

  Button(int X, int Y, int W, int H, String title) {
    x = X;
    y = Y;
    w = W;
    h = H;
    setText(title);
    txtSize = floor(w/3);
  }

  //-------------------------------------------------------------------------------

  void show() {

    fill(colour);
    if(mouseOver())
      fill(color(170,100,255));

    if (customImage) {
      image(display, x, y, w, h);
    } else {
      stroke(0);
      if(noFill){
        noFill(); 
        noStroke();
      }
      if(toggle){
        fill(togglecolor);
      }
      rect(x, y, w, h);
      fill(txtColor);
      textSize(txtSize);
      textAlign(CENTER, CENTER);
      text(name, x + floor(w/2), y + floor(h/2));
    }
  }

  boolean mouseOver() {
    if (x <= mouseX && mouseX <= x + w) {
      if (y <= mouseY && mouseY <= y + h) {
        return true;
      }
    }
    return false;
  }

  void toggle(){
    toggle = !toggle;
  }//needs to be a seperate function from click() to be used publicly

  void click(){
    if(mouseOver())
      toggle();
  }
  
  boolean toggled(){
    //for buttons which stay on until turned off
    // and we need to detect when the on/off state changes
    if(toggle != prevToggle){
      prevToggle = toggle;
      return true;
    }
    
    return false;
  }

  boolean ready(){
    if(toggle){
      toggle();
      return !toggle;
    }
    return false;
  }

  //-------------------------------------------------------------------------------
  


  void setTextSize(int temp) {
    txtSize = temp;
  }

  void setTextColor(color temp) {
    txtColor = temp;
  }

  void setNoFill(){
    noFill = true;
  }

  void setText(String title) {
    name = title;
  }

  void setColor(color temp) {
    colour = temp;
  }

  void setImage(PImage temp) {
    customImage = true;
    display = temp;
  }
}

class Slider {
  int value = 0;
  int x, y, w, h;
  boolean editing = false;
  boolean named = false;
  String val = new String("");

  Button increase;
  Button display;
  Button decrease;
  Button name;

  Slider(int X, int Y, int W, int H) {
    x = X;
    y = Y;
    w = W;
    h = H;

    increase = new Button(x, y, w, h, "↑");
    display = new Button(x + w, y, w, h, str(value));
    decrease = new Button(x + 2 * w, y, w, h, "↓");

    setTextSize(h/2);
  }

  Slider(int X, int Y, int W, int H, String label) {
    x = X;
    y = Y;
    w = W;
    h = H;

    increase = new Button(x, y, w, h, "↑");
    display = new Button(x + w, y, w, h, str(value));
    decrease = new Button(x + 2 * w, y, w, h, "↓");

    setName(label);
  }

  void click(){
    //function that runs during mousepressed()

    editing = false;

    if(increase.mouseOver()){
      value++;
      limitValue();
    }
    else if(decrease.mouseOver()){
      value--;
      limitValue();
    }
    else if( display.mouseOver()){
      editing = true;
      val = "";
    }
  
  }

  void type(){
    //function that runs during keypressed()
    if(editing){
      if( 48 <= key && key <= 57 && editing){ // only interested in digits
        if(val.length() < 3){
          val = val + key;
        }
      }
      else if ( keyCode == BACKSPACE && val.length() > 0 ){
       val = val.substring(0, val.length()-1); // remove latest char
      }
      else if (keyCode == ENTER){
        editing = false;
      }
    }
    if (val.length() > 0) {
      value = int(val);
      limitValue();
    } else {
      value = 0;
      val = str(value);
    }
  }

  void show(){
    //function that runs during draw()
    increase.show();
    display.setText(str(value));
    if(editing)
      display.toggle = true;
    else
      display.toggle = false;
    display.show();
    decrease.show();

    if(named)
      name.show();

  }

  void limitValue(){
    //loop the values around
    if(value > 100){
      value = 0; 
    }
    else if(value < 0)
      value = 100;

    val = str(value);
  }

  void setName(String label){
    name = new Button(x + 3*w, y, w, h, label);
    name.setNoFill();
    name.setTextColor(color(0));
    named = true;
    
    setTextSize(h/2);
  }

  void setTextSize(int temp){
    if(temp > 0){
      increase.setTextSize(temp);
      display.setTextSize(temp);
      decrease.setTextSize(temp);
      if(named)
        name.setTextSize(temp);
    }
  }

  int getValue(){
    return value;
  }
}

void analogNumberDisplay(int x, int y, int num) {
  int size = 3;
  boolean on = true;
  //background(150);
  // each of the grids is 6 wide and 15 tall seperated by a width of 1
  int scale = 8;
  int w = 8 * scale;
  int h = 15 * scale;
  int gap = 1 * scale;

  int zw = floor(0.5*w); // = 4 * scale
  int zh = gap; // = 1 * scale
  int thousand = 0 ;
  
  while (num >= 1000) {
    thousand++;
    num = num - 1000;
    size = 4;
  }
 
  int[] panelX = new int[size];

  for (int X = 0; X < size; X++) {
    panelX[X] = x + X*gap + X*w;  //= { x, x + gap + w, x + 2*gap + 2*w}
  }

  //cooridinates for the top left x coordinate of each panel

  int[] digits = new int[size];
  color ziffern; //ziffern is german for digits
    ziffern = #a8e61d;
  //this is the color that the number will be displayed in on the panel
  int spot = 0;
  
  if (size == 4) {
    digits[spot] = thousand;
    spot++;
  }
  if (size >= 3) {
    digits[spot] = num / 100;
    spot++;
  } 

  digits[spot] = (num / 10) % 10;
  spot++;
  digits[spot] = num % 10;

  color surfaceC = #464646;
  color darNums = #4c4c4c;
  //color green = #a8e61d;

  stroke(surfaceC);
  //draw size count surfaces to display numbers on x + gap*i + w*i
  for (int i = 0; i < size; i++) {
    fill(surfaceC);
    rect(panelX[i], y, w, h);//create the surfaces

    fill(darNums);//create the dark eights
    for (int k = 2; k <= 12; k = k + 5) {
      rect(panelX[i]+ 2*scale, y + k*scale, zw, zh);
    }
    for (int m = 1; m <= 6; m = m + 5) {
      rect(panelX[i]+ m*scale, y+ 3*scale, zh, zw);
      rect(panelX[i]+ m*scale, y+ 8*scale, zh, zw);
    }

    int val = digits[i];
    fill(ziffern); //done with 1 3

    // if the first digit == 0 or the first two digits == 0
    //search the digits array for the first val that isnt 0, if we arent at that spot yet then do nothing
    int index = 0;
    for (int g = 0; g < size; g++) {
      if (digits[g] != 0) {
        index = g;
        g = size;
      }
    }
    //now index has the first space of the number that isnt 0
    if ((val == 0 && i < index ) || on == false) {
      //do nothing
    } else {
      if ( val != 2) {// draw bottom right line
        rect(panelX[i]+ 6*scale, y+ 8*scale, zh, zw);
      }
      if (val != 5 && val !=6) {// draw top right line
        rect(panelX[i]+ 6*scale, y+ 3*scale, zh, zw);
      }
      if (val != 1 && val !=4) {// draw top line
        rect(panelX[i]+ 2*scale, y + 2*scale, zw, zh);
      }
      if (val != 1 && val !=0 && val != 7) {// draw mid line
        rect(panelX[i]+ 2*scale, y + 7*scale, zw, zh);
      }
      if (val != 1 && val !=4 && val != 7) {// draw bottom line
        rect(panelX[i]+ 2*scale, y + 12*scale, zw, zh);
      }
      if (val != 1 && val !=2 && val != 3 && val != 7) {// draw top left line
        rect(panelX[i]+ 1*scale, y+ 3*scale, zh, zw);
      }
      if (val != 1 && val !=4 && val != 3 && val != 7 && val != 5 && val != 9) {// draw bottom left line
        rect(panelX[i]+ 1*scale, y+ 8*scale, zh, zw);
      }
    }
  }
}
