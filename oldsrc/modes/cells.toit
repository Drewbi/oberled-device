import ..frame
import ..screen

class Cells:
  frame_/Frame? := null
  screen_/Screen? := null
  cellList := []

  constructor screen frame=Frame:
    frame_ = frame
    screen_ = screen

  run:
    cellList = []
    7.repeat:
      cellList.add { "x": (random 16), "y": (random 16) }
    cellList.do: | cell |
      frame_.set_current cell["x"] cell["y"]
      frame_.set_pixel_at_current 1
    
    screen_.display frame_.get
    sleep --ms=5000
    frame_.set EMPTY_FRAME


    


