import ..frame
import ..screen

class Debug:
  frame_/Frame? := null
  screen_/Screen? := null

  constructor screen frame=Frame:
    frame_ = frame
    screen_ = screen

  run:
    16.repeat: |j|
        16.repeat: |i|
          frame_.set_current i j
          frame_.set_pixel_at_current 1
          screen_.display frame_.get
          sleep --ms=100
          frame_.set_pixel_at_current 0