import .constants
import .io
import .screen
import .button
import .power
import .frame
import .modes.tree

main:
  init_io
  power := Power
  screen := Screen "landscape"
  button := Button
  screen.fill 0

  task::
    while true:
      if power.off:
        screen.fill 0
      sleep --ms=10


  task::
    frame := Frame
    while true:
      if power.on:
        16.repeat: |j|
          16.repeat: |i|
            frame.set_current i j
            frame.set_pixel_at_current 1
            screen.display frame.get
            sleep --ms=100
            frame.set_pixel_at_current 0
