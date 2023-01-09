import .constants
import .io

segment_sequence := [ 1, 4, -1, 4 ]

class Screen:
  fill x/int:
    NUM_PIXELS.repeat:  
      write_dot x
    push_segment
  
  display data/ByteArray:
    for i := 0; i < NUM_PIXELS; i++:
      write_dot data[SCREEN_MAP[i]]
    push_segment
      