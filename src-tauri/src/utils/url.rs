use url::form_urlencoded;

pub fn encode_uri_component(s: &str) -> String {
    form_urlencoded::byte_serialize(s.as_bytes()).collect()
}
