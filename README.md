# netpipe
UDP and WebSocket integration into Unix pipe

### overview

forwarding data channel <IN> to channel <OUT>

channels can be stdio, websocket, udp.


### how to use

syntax includes:
```
cargo run -- <IN> <OUT>
cargo run -- <IN> <OUT1> <OUT2> <OUT3>

```

examples includes: 
```
cargo run -- stdin stdout
cargo run -- ws://192.168.100.30 localhost:8080
```
