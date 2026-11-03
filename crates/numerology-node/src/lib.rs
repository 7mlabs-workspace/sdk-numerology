use napi_derive::napi;
#[napi]
pub fn calculate_json(request: String) -> String {
    numerology_core::calculate_json(&request)
}
#[napi]
pub fn engine_version() -> &'static str {
    numerology_core::VERSION
}
