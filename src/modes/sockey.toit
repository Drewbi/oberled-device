import http
import net
import websocket
import encoding.json as json
import ..frame
import ..screen

class Sockey:
  frame_/Frame? := null
  screen_/Screen? := null
  deque/Deque

  constructor screen frame=Frame:
    frame_ = frame
    screen_ = screen
    deque = Deque
    network := net.open
    client := http.Client network
    host := "oberled-socket.drub.workers.dev"
    path := "/ob"
    session := websocket.Session.connect client host path

    task::
      while msg := session.receive:
        data := parse msg
        if data != null and data["positions"] != null and data["positions"] is List:
          data["positions"].do: |item|
            if (item.get item.first) != null and (item.get item.last) != null:
              deque.add item
    
    task::
      while true:
        if deque.size > 0:
          deque.remove_first
        sleep --ms=40
  
  parse test/string -> Map:
    return json.parse test.to_string

  run:
    frame_.clear
    deque.do: |pos|
      frame_.set_pixel 15 - (pos.get pos.first) (pos.get pos.last) 1
    screen_.display frame_.get
    sleep --ms=10