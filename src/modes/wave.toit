import ..frame
import ..screen

class Tree:
  frame_/Frame? := null
  screen_/Screen? := null

  constructor screen frame=Frame:
    frame_ = frame
    screen_ = screen

  run:
