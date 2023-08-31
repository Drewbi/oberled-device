import ..frame
import ..screen
import ..mode

class Chase:
  frame_/Frame? := null
  screen_/ScreenLayout? := null
  dotPos_ := null

  constructor screen frame=Frame:
    frame_ = frame
    screen_ = screen
    
  
  run:
    dotPos_ = []
    16.repeat:
      dotPos_.add (random 16)
    16.repeat: |line|
      16.repeat: |i|
        if (mode_changed): return
        frame_.set_current (line+i)%16 dotPos_[i]
        frame_.flip_pixel_at_current
        screen_.display frame_.get
        sleep --ms=(random 20 100)
