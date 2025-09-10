from typing import List, Tuple
from guest.wit_world.exports import transformer

class Transformer():
    def transform(self, headers: List[transformer.Header], body: bytes) -> Tuple[List[transformer.Header], bytes]:
        headers.append(transformer.Header("X-Transformed", "true"))
        headers.append(transformer.Header("X-Transformer", "Python"))
        return headers, body
