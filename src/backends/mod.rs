pub mod wasm;

#[allow(dead_code)]
fn boxed(e: impl std::error::Error + 'static) -> Box<dyn std::error::Error> {
    Box::new(e)
}
