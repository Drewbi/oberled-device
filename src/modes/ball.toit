import ..frame
import ..screen

class Ball:
  frame_/Frame? := null
  screen_/ScreenLayout? := null
  ballPos := { "x": 8, "y": 8 }

  constructor screen frame=Frame:
    frame_ = frame
    screen_ = screen

  run:
    frame_.set_current ballPos["x"] ballPos["y"]
    frame_.set_pixel_at_current 1
    
    screen_.display frame_.get
    sleep --ms=5000
    frame_.set EMPTY_FRAME


    


