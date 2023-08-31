import net

import .io
import .server
import .mode

main:
  init_io
  init_modes
  init_server

  while true:
    current-mode.run
    mode_changed = false
    sleep --ms=33
