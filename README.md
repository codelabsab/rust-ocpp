# rust-ocpp

this repo contains implementations of `ocpp`

## paths

`ocpp/` : contains library implementations of 1.6 and 2.0.1

`ws/` : contains demo websockets server

`docs/` : official ocpp specification

`schemas/` : official json schema's

## ws

Start server by running `ws` and you can test the 
server by connecting with `websocat` and send some json
like:

Install websocat: 

```shell
$ cargo install websocat
```

Build and run ws, example with debug build

```shell
$ cargo build && target/debug/ws
```

Connect and send some json:
```shell
$ echo '[2,"19223201","BootNotification",{"reason":"PowerUp","chargingStation":{"model":"SingleSocketCharger","vendorName":"VendorX"}}]' | websocat -k ws://localhost:3040/ws
```