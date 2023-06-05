import gpio

ENABLE_PIN ::= gpio.Pin 26
DATA_PIN ::= gpio.Pin 27
CLOCK_PIN ::= gpio.Pin 14
LATCH_PIN ::= gpio.Pin 12
BUTTON_PIN := gpio.Pin 25

init_io:
  ENABLE_PIN.configure --output
  DATA_PIN.configure --output
  CLOCK_PIN.configure --output
  LATCH_PIN.configure --output
  BUTTON_PIN.configure --input --pull_up

write_dot x/int:
  DATA_PIN.set x
  CLOCK_PIN.set 1
  CLOCK_PIN.set 0

display_frame:
  LATCH_PIN.set 1
  LATCH_PIN.set 0