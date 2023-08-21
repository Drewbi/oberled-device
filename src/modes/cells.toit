import ..frame
import ..screen

class Cells:
  frame_/Frame? := null
  screen_/Screen? := null
  cellList := []
  count := 0

  constructor screen frame=Frame:
    frame_ = frame
    screen_ = screen

  run:
    if count == 0:
      cellList = []
      4.repeat:
        cellList.add { "x": (random 16), "y": (random 16) }

    cellList.do: | cell |
      frame_.set_current cell["x"] cell["y"]

      while count < 255:
        if frame_.get_pixel_at_current == 0:
          frame_.set_pixel_at_current 1
          count++
          screen_.display frame_.get
          break
        else:
          frame_.move_current (random 3) - 1 (random 3) - 1
          frame_.flip_pixel_at_current
          screen_.display frame_.get
          sleep --ms=10
          frame_.flip_pixel_at_current
          screen_.display frame_.get
      sleep --ms=100
    
    if count >= 255:
      count = 0
      frame_.clear