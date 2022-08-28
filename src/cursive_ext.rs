/// Extension trait for the `Cursive` root to simplify initialization.
///
/// It brings backend-specific methods to initialize a `Cursive` root.
///
/// # Examples
///
/// ```rust,no_run
/// use cursive_wasm::{Cursive, CursiveExt};
///
/// let mut siv = Cursive::new();
///
/// // Use `CursiveExt::run()` to pick one of the enabled backends,
/// // depending on cargo features.
/// siv.run();
///
/// // Or explicitly use a specific backend
/// siv.run_wasm();
/// ```
pub trait CursiveExt {
    /// Tries to use one of the enabled backends.
    ///
    /// Will fallback to the dummy backend if no other backend feature is enabled.
    ///
    /// # Panics
    ///
    /// If the backend initialization fails.
    fn run(&mut self);

    /// Creates a new Cursive root using a wasm backend.
    fn run_wasm(&mut self);
}

impl CursiveExt for cursive_core::Cursive {
    fn run(&mut self) {
        self.run_wasm()
    }

    fn run_wasm(&mut self) {
        self.run_with(crate::backends::wasm::Backend::init)
    }
}
