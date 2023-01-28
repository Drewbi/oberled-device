import .frame

test_animation:
  frame := Frame
  frame.set_current (random 16) (random 16)
  list := []
  16.repeat:
    frame.set_pixel_at_current 0
    frame.move_current (random 3 - 1) (random 3 - 1)
    frame.set_pixel_at_current 1
    list.add frame.copy
  return list

ANIMATION ::= test_animation