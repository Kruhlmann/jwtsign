# JWTSign

Rust jwt signing bindings for python

Usage:

```py
from jwtsign import PyJwtEncoder, PyJwtDecoder

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
