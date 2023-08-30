import http
import net
import websocket
import encoding.json as json
import ..frame
import ..screen

class Sockey:
  frame_/Frame? := null
  screen_/Screen? := null
  pixelList/List

  constructor screen frame=Frame:
    frame_ = frame
    screen_ = screen
    pixelList = List
    network := net.open
    client := http.Client network
    host := "oberled-socket.drub.workers.dev"
    path := "/ob"
    print "Connecting to socket"
    session := websocket.Session.connect client host path

    task::
      while msg := session.receive:
        sleep --ms=1
        screen.display frame_.get
        if msg == "ping":
          session.send "pong"
        else:
          data := parse msg
          if data != null and data.contains "positions" and data["positions"] is List:
            data["positions"].do: |item|
              if item.contains "x" and item.contains "y":
                frame_.set_pixel 15 - item["x"] item["y"] 1
                screen_.display frame_.get
        
  parse test/string -> Map:
    return json.parse test.to_string

  run:
    frame_.clear
    sleep --ms=50