use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum ValidationError {
    #[error("Entrada vacía: se requiere al menos un elemento")]
    Empty,
    #[error("Elemento en índice {index} no es entero: '{value}'")]
    NonInteger { index: usize, value: String },
    #[error("Longitud de entrada excede máximo de 100 elementos: {len}")]
    TooLong { len: usize },
    #[error("Desbordamiento numérico en elemento índice {index}: {value}")]
    Overflow { index: usize, value: String },
}

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum SortError {
    #[error("Error de validación previo: {0}")]
    Validation(#[from] ValidationError),
}
