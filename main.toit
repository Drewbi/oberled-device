import .constants
import .io
import .screen
import .button
import .animations

main:
  init_io
  screen := Screen
  button := Button
  screen.fill 0
  index := 0
  while true:
    frame := BASE_IMAGE.copy
    frame[index] = 1
    screen.display frame
    index = (index + 1) % NUM_PIXELS
    sleep --ms=100
