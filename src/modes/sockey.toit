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
        screen.display frame_.get
        print "Received $msg"
        if msg == "ping":
          session.send "pong"
        else:
          data := parse msg
          if data != null and data.contains "positions" and data["positions"] is List:
            data["positions"].do: |item|
              if (item.get item.first) != null and (item.get item.last) != null:
                pixelList.add item
                task::
                  sleep --ms=250
                  pixelList.remove item
        
  parse test/string -> Map:
    return json.parse test.to_string

  run:
    frame_.clear
    pixelList.do: |pos|
      frame_.set_pixel 15 - (pos.get pos.first) (pos.get pos.last) 1
    screen_.display frame_.get
    sleep --ms=10