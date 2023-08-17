from typing import TypeVar, Generic

ClaimsModel = TypeVar("ClaimsModel")

class PyJwtEncoder(Generic[ClaimsModel]):
    def __init__(self, private_key: bytes) -> None: ...
    
    def encode_claims_json_str(self, claims_str: str) -> str: ...
    
    def encode_claims_json_obj(self, claims: ClaimsModel) -> str: ...
    

class PyJwtDecoder(Generic[ClaimsModel]):

    def __init__(self, bytes, public_key: bytes, leeway: int) -> None: ...

    def decode(self, token: str) -> ClaimsModel: ...

