import sys
import os
import json
import time
current_dir = os.getcwd()
sys.path.append(os.path.join(current_dir, "target", "release"))
from libjwtsign import PyJwtEncoderDecoder

def read_bin(path: str) -> bytes:
    with open(path, "rb") as file:
        return file.read()

# create an instance of PyJwtEncoderDecoder
private_key = read_bin("res/private_key.pem")
public_key = read_bin("res/public_key.pem")
encoder_decoder = PyJwtEncoderDecoder(private_key, public_key, leeway=60)

claims = {
    "id": "example",
    "exp": int(time.time() + 60),
}

token = encoder_decoder.encode_claims_json_obj(claims)
decoded_claims_obj = encoder_decoder.decode(token)
token = encoder_decoder.encode_claims_json_str(json.dumps(claims))
decoded_claims_str = encoder_decoder.decode(token)

assert decoded_claims_obj == decoded_claims_str
print("Passed")
