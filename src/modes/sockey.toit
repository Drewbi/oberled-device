import http
import net
import encoding.json as json
import ..frame
import ..screen

class Sockey:
  frame_/Frame? := null
  screen_/Screen? := null

  constructor screen frame=Frame:
    frame_ = frame
    screen_ = screen
    network := net.open
    client := http.Client network

    websocket := client.web_socket --host="oberled-socket.drub.workers.dev"

    task --background::
      while reader := websocket.start_receiving:
        size := 0
        data := #[]
        while chunk := reader.read:
          frame_.clear
          data += chunk
          if reader.is_text:
            failed_parse := catch:
              object := json.parse data.to_string
              if object is not Map and object.first != "error":
                object.do: |item|
                  if item != null:
                    if (item.get item.first) != null and (item.get item.last) != null:
                      frame_.set-pixel (item.get item.last) 15 - (item.get item.first) 1
  run:
    screen_.display frame_.get
    sleep --ms=10