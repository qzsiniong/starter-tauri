use url::form_urlencoded;

#[allow(dead_code)]
pub fn encode_uri_component(s: &str) -> String {
    form_urlencoded::byte_serialize(s.as_bytes()).collect()
}
