# 4. RPC framework

This module implements the RPC part of the OCPP specification

Incoming messages should contain either of the following:

- Call
- CallResult
- CallError

A `Call` might look like this:

```json
[
    2,
    "19223201",
    "BootNotification",
    {
        "reason": "PowerUp",
        "chargingStation": {
            "model": "SingleSocketCharger","vendorName": "VendorX"
            }
        }
]
```
