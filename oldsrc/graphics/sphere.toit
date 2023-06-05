import ..frame

class Sphere:
  _size/int
  _x/int
  _y/int

  constructor size=3 x=8 y=8:
    _size = size
    _x = x
    _y = y

  display frame/Frame:
    frame
