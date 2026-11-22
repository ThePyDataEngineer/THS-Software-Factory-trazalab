//! visu-core: Motor de ordenamiento burbuja determinista con traza navegable.
//!
//! Este crate contiene la lógica pura de validación, ordenamiento y generación de traza
//! sin dependencias de WebAssembly, WASM-bindgen, ni APIs de navegador.

pub mod error;
pub mod sorting;
pub mod trace;
pub mod validation;

pub use error::{SortError, ValidationError};
pub use sorting::{bubble_sort, SortResult};
pub use trace::{Trace, TraceStep, SwapInfo};
pub use validation::{validate_input, ValidatedInput};
