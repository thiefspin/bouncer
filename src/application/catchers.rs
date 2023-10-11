#[catch(500)]
pub fn internal_error() -> &'static str {
    "Well... This is embarrassing..."
}