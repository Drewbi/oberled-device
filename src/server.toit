import net
import http
import .mode
import .ui

NETWORK ::= net.open --name="oberled"

init_server -> int:
  server_socket := NETWORK.tcp_listen 6868
  port := server_socket.local_address.port
  server := http.Server
  address := "http://$NETWORK.address:$port/"
  task --background::
    print "Server on $address"
    server.listen server_socket:: | request/http.Request response/http.ResponseWriter |
      if request.path == "/":
        send_homepage response
      else if VALID_MODES.contains request.path[1..]:
        set_mode request.path[1..]
        send_homepage response
      else:
        send_404 response
  return port

send_homepage response/http.ResponseWriter:
  response.headers.set "Content-Type" "text/html"
  response.write MAIN-PAGE

send_404 response/http.ResponseWriter:
  response.headers.set "Content-Type" "text/html"
  response.write NOT-FOUND-PAGE
