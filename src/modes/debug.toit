import ..frame
import ..screen
import ..mode

class Debug:
  frame_/Frame? := null
  screen_/ScreenLayout? := null

  constructor screen frame=Frame:
    frame_ = frame
    screen_ = screen

  run:
    16.repeat: |j|
        16.repeat: |i|
          if (mode_changed): return
          frame_.set_current i j
          frame_.set_pixel_at_current 1
          screen_.display frame_.get
          sleep --ms=100
          frame_.set_pixel_at_current 0