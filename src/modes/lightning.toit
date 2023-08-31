import ..frame
import ..screen

class Lightning:
  frame_/Frame? := null
  screen_/ScreenLayout? := null
  instruc_ := null

  constructor screen frame=Frame:
    frame_ = frame
    screen_ = screen

  run:
    instruc_ = []
    5.repeat:
      instruc_.add [(random 3), (random 5), (random 2) - 1]
    frame_.set_current 15 8
    frame_.set_pixel_at_current 1
    
    instruc_.do: |ins|
      ins[1].repeat:
        ins[0].repeat:
          frame_.move_current -1 0
          frame_.set_pixel_at_current 1
        frame_.move-current 0 ins[2]
        frame_.set_pixel_at_current 1

    frame_.set_current 15 7
    frame_.set_pixel_at_current 1

    instruc_.do: |ins|
      ins[1].repeat:
        ins[0].repeat:
          frame_.move_current -1 0
          frame_.set_pixel_at_current 1
        frame_.move-current 0 ins[2] * -1
        frame_.set_pixel_at_current 1

    screen_.display frame_.get
    sleep --ms=3000
    frame_.clear