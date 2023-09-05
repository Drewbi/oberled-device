import ..screen

class BlinkFast:
  screen_/ScreenLayout? := null

  constructor screen:
    screen_ = screen
  
  run:
    screen_.fill 1
    screen_.fill 1
    screen_.fill 0

