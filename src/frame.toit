EMPTY_FRAME ::= #[
  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
]

class Frame:
  data_ := null

  constructor:
    data_ = EMPTY_FRAME.copy

  current_x := 0
  current_y := 0

  current_pixel:
    return current_y * 16 + current_x

  set data/ByteArray:
    data_ = data
  
  get: return data_

  clear:
    data_ = EMPTY_FRAME.copy
      
  copy: return data_.copy

  set_pixel x/int y/int bit/int:
    if x >= 0 and x < 16 and y >= 0 and y < 16:
      data_[y * 16 + x] = bit

  set_pixel_at_current bit/int:
    data_[current_pixel] = bit
  
  get_pixel_at_current:
    return data_[current_pixel]
  
  flip_pixel_at_current:
    data_[current_pixel] = data_[current_pixel] == 0 ? 1 : 0

  set_current x/int y/int:
    if x >= 0 and x < 16 and y >= 0 and y < 16:
      current_x = x
      current_y = y

  move_current x/int y/int:
    if x + current_x >= 0 and x + current_x < 16 and y + current_y >= 0 and y + current_y < 16:
      current_x += x
      current_y += y

  can_move x/int y/int:
    return x + current_x >= 0 and x + current_x < 16 and y + current_y >= 0 and y + current_y < 16