import ..frame
import ..screen
import ..mode

winPos := [
  [1, 1], [14, 1],
  [1, 14], [14, 14]
]

class Chase:
  frame_/Frame? := null
  screen_/ScreenLayout? := null
  currX := null
  currY := null
  velX := null
  velY := null

  constructor screen frame=Frame:
    frame_ = frame
    screen_ = screen
    currX = random 1 15
    currY = random 1 15
    velX = (random 5) - 2
    velY = (random 5) - 2
    
  run:
    if(winPos.any: it[0] == currX and it[1] == currY):
      runWin frame_ screen_
    else:
      currX += velX
      currY += velY

      if currX <= 1 or currX >= 15:
        velX *= -1
      if currY <= 1 or currY >= 15:
        velY *= -1
      drawDisc currX currY frame_
      screen_.display frame_.get
      sleep --ms=100

drawDisc x/int y/int frame/Frame:
  shapeArr := [
    [1, 0], [0, 1],
    [-1, 0], [-1, 0],
    [0, -1], [0, -1],
    [1, 0], [1, 0]
  ]

  shapeArr.do: |point|
    if frame.can-move point[0] + x point[1] + y:
      frame.set-current point[0] + x point[1] + y
      frame.set-pixel-at-current 1

runWin frame/Frame screen/ScreenLayout:
  debug "You have won"