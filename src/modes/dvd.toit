import ..frame
import ..screen
import ..mode

max_count ::= 100

class DVD:
  frame_/Frame? := null
  screen_/ScreenLayout? := null
  currX := null
  currY := null
  velX := null
  velY := null
  count := max_count

  constructor screen frame=Frame:
    frame_ = frame
    screen_ = screen
    currX = random 3 12
    currY = random 3 12
    velX = 2
    velY = 1
    
  run:
    if((currX <= 0 or currX >= 15) and (currY <= 0 or currY >= 15)):
      runWin frame_ screen_
      currX = random 3 12
      currY = random 3 12
      count = max_count
    currX += velX
    currY += velY

    if currX <= 0 or currX >= 15:
      velX *= -1
    if currY <= 0 or currY >= 15:
      velY *= -1
    frame_.clear
    drawDisc currX currY frame_
    screen_.display frame_.get
    sleep --ms=200
    count -= 1
    if(count < 0):
      count = max_count
      currX = random 0 16
      currY = random 0 16

drawDisc x/int y/int frame/Frame:
  shapeArr := [
    [1, 0], [0, 1],
    [-1, 0], [0, -1],
  ]

  shapeArr.do: |point|
    if frame.can-move point[0] + x point[1] + y:
      frame.set-pixel point[0] + x point[1] + y 1

runWin frame/Frame screen/ScreenLayout:
  16.repeat: |i|
    frame.set-pixel 0 i 1
    frame.set-pixel i 0 1
    frame.set-pixel i 15 1
    frame.set-pixel 15 i 1
  screen.display frame.get
