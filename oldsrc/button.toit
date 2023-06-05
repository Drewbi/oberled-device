import .io

class Button:
  current_state_ := false
  last_state_ := false

  constructor:
    task:: update
      
  update:
    while true:
      last_state_ = current_state_
      current_state_ = BUTTON_PIN.get == 0
      sleep --ms=10

  clicked:
    return current_state_ and not last_state_
  
  pressed:
    return current_state_

  