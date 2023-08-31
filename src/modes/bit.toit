import ..io
import ..mode

class Bit:
  run:
    256.repeat:
      write-dot 1
      display-frame
      sleep --ms=30
      if (mode_changed): return
    256.repeat:
      write-dot 0
      display-frame
      sleep --ms=5
      if (mode_changed): return