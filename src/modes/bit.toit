import ..io

class Bit:
  constructor screen:
  run:
    256.repeat:
      write-dot 1
      display-frame
      sleep --ms=30
    256.repeat:
      write-dot 0
      display-frame
      sleep --ms=5