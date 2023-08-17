import json
import time
from jwtsign import PyJwtEncoder, PyJwtDecoder

def read_bin(path: str) -> bytes:
    with open(path, "rb") as file:
        return file.read()

# create an instance of PyJwtEncoderDecoder
private_key = read_bin("res/private_key.pem")
public_key = read_bin("res/public_key.pem")
encoder = PyJwtEncoder(private_key)
decoder = PyJwtDecoder(public_key, leeway=60)

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
