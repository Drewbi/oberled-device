import .button

class Power:
  state_/bool := true
  button_ := Button
  state_has_changed_/bool := false
  hold_increment_/int := 0

  constructor:
    task::
      while true:
        if button_.pressed:
          hold_increment_++
        else:
          hold_increment_ = 0
          state_has_changed_ = false
        if hold_increment_ > 5 and not state_has_changed_:
          state_ = not state_
          state_has_changed_ = true
        sleep --ms=10

  on -> bool: return state_
  off -> bool: return not state_

