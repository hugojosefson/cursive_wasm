use crate::backends;
use cursive_core::{backend, Cursive, CursiveRunner};
use wasm_bindgen::prelude::*;

type Initializer = dyn FnMut() -> Result<Box<dyn backend::Backend>, Box<dyn std::error::Error>>;

/// A runnable wrapper around `Cursive`, bundling the backend initializer.
///
/// This struct embeds both `Cursive` and a backend-initializer
/// (`FnMut() -> Result<dyn Backend>`), to provide a simple `.run()` method.
///
/// This lets you pick the backend when creating the Cursive root, rather than
/// when running it.
///
/// It implements `DerefMut<Target=Cursive>`, so you can use it just like a
/// regular `Cursive` object.
#[wasm_bindgen]
pub struct CursiveRunnable {
    siv: Cursive,
    backend_init: Box<Initializer>,
}

impl std::ops::Deref for CursiveRunnable {
    type Target = Cursive;

    fn deref(&self) -> &Cursive {
        &self.siv
    }
}

impl std::ops::DerefMut for CursiveRunnable {
    fn deref_mut(&mut self) -> &mut Cursive {
        &mut self.siv
    }
}

impl std::borrow::Borrow<Cursive> for CursiveRunnable {
    fn borrow(&self) -> &Cursive {
        self
    }
}

impl std::borrow::BorrowMut<Cursive> for CursiveRunnable {
    fn borrow_mut(&mut self) -> &mut Cursive {
        self
    }
}

/// Helper function to help type inference when `Box::new` would not work.
fn boxed(e: impl std::error::Error + 'static) -> Box<dyn std::error::Error> {
    Box::new(e)
}

impl CursiveRunnable {
    /// Creates a new Cursive wrapper using the given boxed backend initializer.
    fn with_initializer(backend_init: Box<Initializer>) -> Self {
        let siv = Cursive::new();
        Self { siv, backend_init }
    }

    /// Creates a new Cursive wrapper, using the given backend.
    pub fn new<E, F>(mut backend_init: F) -> Self
    where
        E: std::error::Error + 'static,
        F: FnMut() -> Result<Box<dyn backend::Backend>, E> + 'static,
    {
        Self::with_initializer(Box::new(move || backend_init().map_err(boxed)))
    }

    /// Runs the event loop with the registered backend initializer.
    ///
    /// # Panics
    ///
    /// If the backend initialization fails.
    pub fn run(&mut self) {
        self.try_run().unwrap();
    }

    /// Runs the event loop with the registered backend initializer.
    pub fn try_run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.siv.try_run_with(&mut self.backend_init)
    }

    /// Gets a runner with the registered backend.
    ///
    /// Used to manually control the event loop. In most cases, running
    /// `run()` will be easier.
    ///
    /// The runner will borrow `self`; when dropped, it will clear out the
    /// terminal, and the cursive instance will be ready for another run if
    /// needed.
    pub fn try_runner(
        &mut self,
    ) -> Result<CursiveRunner<&mut Cursive>, Box<dyn std::error::Error>> {
        Ok(self.siv.runner((self.backend_init)()?))
    }

    /// Gets a runner with the registered backend.
    ///
    /// # Panics
    ///
    /// If the backend initialization fails.
    pub fn runner(&mut self) -> CursiveRunner<&mut Cursive> {
        self.try_runner().unwrap()
    }

    /// Returns a new runner on the registered backend.
    ///
    /// Used to manually control the event loop. In most cases, running
    /// `run()` will be easier.
    ///
    /// The runner will embed `self`; when dropped, it will clear out the
    /// terminal, and the cursive instance will be dropped as well.
    pub fn try_into_runner(mut self) -> Result<CursiveRunner<Self>, Box<dyn std::error::Error>> {
        let backend = (self.backend_init)()?;
        Ok(CursiveRunner::new(self, backend))
    }

    /// Returns a new runner on the registered backend.
    ///
    /// Used to manually control the event loop. In most cases, running
    /// `run()` will be easier.
    ///
    /// The runner will embed `self`; when dropped, it will clear out the
    /// terminal, and the cursive instance will be dropped as well.
    ///
    /// # Panics
    ///
    /// If the backend initialization fails.
    pub fn into_runner(self) -> CursiveRunner<Self> {
        self.try_into_runner().unwrap()
    }

    /// Creates a new Cursive wrapper using the dummy backend.
    ///
    /// Nothing will actually be output when calling `.run()`.
    pub fn dummy() -> Self {
        Self::new::<std::convert::Infallible, _>(|| Ok(cursive_core::backend::Dummy::init()))
    }

    /// Creates a new Cursive wrapper using the wasm backend.
    pub fn wasm() -> Self {
        Self::new::<std::convert::Infallible, _>(|| Ok(backends::wasm::Backend::init()))
    }
}
