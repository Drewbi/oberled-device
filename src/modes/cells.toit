import ..frame
import ..screen
import ..mode

class Cells:
  frame_/Frame? := null
  screen_/ScreenLayout? := null
  cellList := []

  constructor screen frame=Frame:
    frame_ = frame
    screen_ = screen
    4.repeat:
      cellList.add { "x": (random 16), "y": (random 16) }

  run:
    cellList.do: | cell |
      frame_.set_current cell["x"] cell["y"]

      while true:
        if (mode_changed): return
        if frame_.get_pixel_at_current == 0:
          frame_.set_pixel_at_current 1
          screen_.display frame_.get
          break
        else:
          if (random 2) == 0:
            frame_.move_current (random 3) - 1 0
          else:
            frame_.move_current 0 (random 3) - 1
            
          frame_.flip_pixel_at_current
          screen_.display frame_.get
          sleep --ms=10
          frame_.flip_pixel_at_current
          screen_.display frame_.get
      sleep --ms=100