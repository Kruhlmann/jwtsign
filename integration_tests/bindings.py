import json
import time
from jwtsign import PyJwtEncoder, PyJwtDecoder

def read_bin(path: str) -> bytes:
    with open(path, "rb") as file:
        return file.read()

invalid_private_key = read_bin("res/private_key.invalid.pem")
invalid_public_key = read_bin("res/public_key.invalid.pem")

try:
    PyJwtEncoder(invalid_private_key)
    raise AssertionError("Initialized encoder with invalid private key")
except ValueError:
    pass

try:
    PyJwtDecoder(invalid_public_key, leeway=0)
    raise AssertionError("Initialized decoder with invalid public key")
except ValueError:
    pass

private_key = read_bin("res/private_key.pem")
public_key = read_bin("res/public_key.pem")
encoder = PyJwtEncoder(private_key)
decoder = PyJwtDecoder(public_key, leeway=60)

expired_claims = {
    "id": "example",
    "exp": 0,
}

try:
    invalid_token = encoder.encode_claims_json_obj(expired_claims)
    decoded_claims_obj = decoder.decode(invalid_token)
    raise AssertionError("Decoded expired claims")
except ValueError:
    pass

claims = {
    "id": "example",
    "exp": int(time.time() + 60),
}

token_claims_obj = encoder.encode_claims_json_obj(claims)
decoded_claims_obj = decoder.decode(token_claims_obj )
token_claims_str = encoder.encode_claims_json_str(json.dumps(claims))
decoded_claims_str = decoder.decode(token_claims_str)

assert decoded_claims_obj == decoded_claims_str

print("Passed")
print("Encoded/decoded", json.dumps(claims))
print("With tokens:")
print(f"\tobj:{token_claims_obj}")
print(f"\tstr:{token_claims_str}")
