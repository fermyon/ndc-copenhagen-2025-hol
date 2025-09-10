use crate::exports::ndc::copenhagen::transformer::{self, Guest, Header};

wit_bindgen::generate!({
    world: "guest",
    path: "../../wit/world.wit"
});

struct Transformer;

impl Guest for Transformer {
    fn transform(
        mut headers: _rt::Vec<transformer::Header>,
        body: _rt::Vec<u8>,
    ) -> (_rt::Vec<transformer::Header>, _rt::Vec<u8>) {
        headers.push(Header {
            key: "x-transformed".to_string(),
            value: "true".to_string(),
        });
        headers.push(Header {
            key: "x-transformer".to_string(),
            value: "rust".to_string(),
        });
        (headers, body)
    }
}

export!(Transformer);
