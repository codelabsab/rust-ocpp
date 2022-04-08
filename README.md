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
cargo install websocat
```

Build and run ws, example with debug build

```shell
cargo build && target/debug/ws
```

Connect and send some json:

```shell
websocat -k ws://localhost:3040/ws
```

Call

```json
[2,"1","BootNotification",{"reason":"PowerUp","chargingStation":{"model":"SingleSocketCharger","vendorName":"VendorX"}}]
```

CallResult

```json
[3,"2","BootNotification",{"reason":"PowerUp","chargingStation":{"model":"SingleSocketCharger","vendorName":"VendorX"}}]
```

CallError

```json
[4,"2","error_code","error_description","error_details"]
```
