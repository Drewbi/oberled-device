import ..frame
import ..screen

class Tree:
  frame_/Frame? := null
  screen_/ScreenLayout? := null

  constructor screen frame=Frame:
    frame_ = frame
    screen_ = screen
    screen_.fill 1
    frame_.set_current 8 15

  run:
    frame_.set_pixel_at_current 1
    frame_.move_current -1 0
    frame_.set_pixel_at_current 1
    frame_.move_current 1 -1
    screen_.display frame_.get
