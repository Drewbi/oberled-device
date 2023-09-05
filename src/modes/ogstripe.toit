import ..frame
import ..screen
import ..mode

class OGStripe:
  frame_/Frame? := null
  screen_/ScreenLayout? := null
  offset := 0
  src/Deque := Deque

  constructor screen frame=Frame:
    frame_ = frame
    screen_ = screen
  
  run:
    frame_.set_current 0 0
    
    if offset % 4 == 0 or offset % 4 == 1:
      src.add 1
    else:
      src.add 0
    offset++
    if src.size > 16:
      src.remove_first
    src.do --reversed=true: |val|
      16.repeat: |line|
        if (mode_changed): return
        frame_.set_pixel_at_current val
        frame_.move_current 1 0
      frame_.move_current -15 1
    screen_.display frame_.get
    sleep --ms=100