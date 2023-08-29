# JWTSign

Rust jwt signing bindings for python

Usage:

```py
import time
import json

from jwtsign import PyJwtEncoder, PyJwtDecoder

def read_bin(path: str) -> bytes:
    with open(path, "rb") as file:
        return file.read()

claims = {
    "id": "example",
    "exp": int(time.time() + 60),
}

private_key = read_bin("res/private_key.pem")
public_key = read_bin("res/public_key.pem")
encoder = PyJwtEncoder(private_key)
decoder = PyJwtDecoder(public_key, leeway=60)

# Using object encoding
token_claims_obj = encoder.encode_claims_json_obj(claims)
decoded_claims_obj = decoder.decode(token_claims_obj)

# Using serialized object encoding
token_claims_str = encoder.encode_claims_json_str(json.dumps(claims))
decoded_claims_str = decoder.decode(token_claims_str)
```

Decoder validates expiration time based on UTC with a leeway defined in the `PyJwtDecoder::__init__`.

> [!NOTE]
> If at any point you have your claim as serialized JSON keep in mind that this perform slightly faster than the object serialization, as the rust code uses python bindings to call `json.dumps` on the object.
