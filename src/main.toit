import .constants
import .io
import .screen
import .button
import .frame
import .power

main:
  init_io
  power := Power
  screen := Screen
  button := Button
  screen.fill 0
  frame := Frame
  frame.set_current 8 8


  task::
    while true:
      if power.off:
        screen.fill 0
      else:
        screen.display frame.get  
      sleep --ms=10

  task::
    while true:
      if power.on:
        frame.move_current ((random 3) - 1) ((random 3) - 1)
        frame.flip_pixel_at_current
      sleep --ms=60000
    
