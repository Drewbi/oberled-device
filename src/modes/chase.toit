import ..frame
import ..screen

class Chase:
  frame_/Frame? := null
  screen_/Screen? := null
  dotPos_ := null

  constructor screen frame=Frame:
    frame_ = frame
    screen_ = screen
    
  
  run:
    dotPos_ = []
    16.repeat:
      dotPos_.add (random 16)
    32.repeat: |line|
      16.repeat: |i|
        frame_.set_current (line+i)%16 dotPos_[i]
        frame_.flip_pixel_at_current
        screen_.display frame_.get
        sleep --ms=(random 20 100)


