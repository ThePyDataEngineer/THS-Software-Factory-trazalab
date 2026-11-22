use crate::error::ValidationError;
use serde::{Deserialize, Serialize};

/// Entrada validada y lista para ordenamiento.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidatedInput {
    pub values: Vec<i32>,
}

/// Valida una entrada JSON genérica y la convierte a enteros.
///
/// Reglas:
/// - Debe ser un array no vacío (máx 100 elementos)
/// - Todos los elementos deben ser enteros (i32)
pub fn validate_input(input: &serde_json::Value) -> Result<ValidatedInput, ValidationError> {
    let arr = input.as_array().ok_or(ValidationError::Empty)?;

    if arr.is_empty() {
        return Err(ValidationError::Empty);
    }

    if arr.len() > 100 {
        return Err(ValidationError::TooLong { len: arr.len() });
    }

    let mut values = Vec::with_capacity(arr.len());
    for (idx, val) in arr.iter().enumerate() {
        match val {
            serde_json::Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    if i < i32::MIN as i64 || i > i32::MAX as i64 {
                        return Err(ValidationError::Overflow {
                            index: idx,
                            value: n.to_string(),
                        });
                    }
                    values.push(i as i32);
                } else {
                    return Err(ValidationError::NonInteger {
                        index: idx,
                        value: n.to_string(),
                    });
                }
            }
            _ => {
                return Err(ValidationError::NonInteger {
                    index: idx,
                    value: val.to_string(),
                });
            }
        }
    }

    Ok(ValidatedInput { values })
}

/// Valida directamente desde un slice de enteros (para tests internos).
pub fn validate_slice(input: &[i32]) -> Result<ValidatedInput, ValidationError> {
    if input.is_empty() {
        return Err(ValidationError::Empty);
    }
    if input.len() > 100 {
        return Err(ValidationError::TooLong { len: input.len() });
    }
    Ok(ValidatedInput {
        values: input.to_vec(),
    })
}
