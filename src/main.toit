import .constants
import .io
import .screen
import .button
import .power
import .frame
import .modes.cells

main:
  init_io
  power := Power
  screen := Screen "landscape"
  button := Button
  screen.fill 0
  mode := Cells screen

  task::
    while true:
      if power.off:
        screen.fill 0
      sleep --ms=10

  task::
    while true:
      if power.on:
        mode.run
