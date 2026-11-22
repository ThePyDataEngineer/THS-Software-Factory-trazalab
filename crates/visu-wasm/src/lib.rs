//! visu-wasm: Enlace WebAssembly para visu-core.
//!
//! Expone la funcionalidad de validación, ordenamiento y traza a JavaScript
//! mediante wasm-bindgen.

use visu_core::{bubble_sort, validate_input, SortResult, Trace, ValidatedInput, ValidationError};
use wasm_bindgen::prelude::*;
use serde_wasm_bindgen::{to_value, from_value};
use serde::{Serialize, Deserialize};
use serde_json;

/// Inicializa el hook de panic para mostrar errores en la consola del navegador.
#[wasm_bindgen(start)]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

/// Entrada para la API Wasm: array de valores JSON.
#[derive(Serialize, Deserialize)]
pub struct WasmInput {
    pub values: Vec<serde_json::Value>,
}

/// Salida de validación para Wasm.
#[derive(Serialize, Deserialize)]
pub struct WasmValidationResult {
    pub valid: bool,
    pub values: Option<Vec<i32>>,
    pub error: Option<WasmValidationError>,
}

/// Error de validación serializable para Wasm.
#[derive(Serialize, Deserialize)]
pub struct WasmValidationError {
    pub code: String,
    pub index: Option<usize>,
    pub value: Option<String>,
    pub message: String,
}

/// Paso de traza serializable para Wasm.
#[derive(Serialize, Deserialize)]
pub struct WasmTraceStep {
    pub step: usize,
    pub array: Vec<i32>,
    pub swap: Option<WasmSwapInfo>,
}

/// Intercambio serializable para Wasm.
#[derive(Serialize, Deserialize)]
pub struct WasmSwapInfo {
    pub index: usize,
    pub a: i32,
    pub b: i32,
}

/// Traza completa serializable para Wasm.
#[derive(Serialize, Deserialize)]
pub struct WasmTrace {
    pub steps: Vec<WasmTraceStep>,
    pub comparisons: usize,
    pub swaps: usize,
}

/// Resultado completo de ordenamiento para Wasm.
#[derive(Serialize, Deserialize)]
pub struct WasmSortResult {
    pub sorted: Vec<i32>,
    pub trace: WasmTrace,
}

/// Valida una entrada desde JavaScript.
#[wasm_bindgen]
pub fn validate(input: JsValue) -> JsValue {
    let wasm_input: WasmInput = from_value(input).unwrap_or_else(|_| WasmInput { values: vec![] });
    let json_values = serde_json::Value::Array(wasm_input.values);

    match validate_input(&json_values) {
        Ok(validated) => to_value(&WasmValidationResult {
            valid: true,
            values: Some(validated.values),
            error: None,
        }).unwrap(),
        Err(e) => to_value(&WasmValidationResult {
            valid: false,
            values: None,
            error: Some(validation_error_to_wasm(e)),
        }).unwrap(),
    }
}

/// Convierte ValidationError a WasmValidationError.
fn validation_error_to_wasm(e: ValidationError) -> WasmValidationError {
    match e {
        ValidationError::Empty => WasmValidationError {
            code: "empty".to_string(),
            index: None,
            value: None,
            message: e.to_string(),
        },
        ValidationError::NonInteger { index, ref value } => WasmValidationError {
            code: "non_integer".to_string(),
            index: Some(index),
            value: Some(value.clone()),
            message: e.to_string(),
        },
        ValidationError::TooLong { len } => WasmValidationError {
            code: "too_long".to_string(),
            index: None,
            value: Some(len.to_string()),
            message: e.to_string(),
        },
        ValidationError::Overflow { index, ref value } => WasmValidationError {
            code: "overflow".to_string(),
            index: Some(index),
            value: Some(value.clone()),
            message: e.to_string(),
        },
    }
}

/// Ejecuta ordenamiento burbuja y devuelve resultado con traza completa.
#[wasm_bindgen]
pub fn sort(input: JsValue) -> JsValue {
    let wasm_input: WasmInput = from_value(input).unwrap_or_else(|_| WasmInput { values: vec![] });
    let json_values = serde_json::Value::Array(wasm_input.values);

    match validate_input(&json_values) {
        Ok(validated) => {
            match bubble_sort(validated) {
                Ok(result) => to_value(&sort_result_to_wasm(result)).unwrap(),
                Err(_e) => to_value(&WasmSortResult {
                    sorted: vec![],
                    trace: WasmTrace {
                        steps: vec![],
                        comparisons: 0,
                        swaps: 0,
                    },
                }).unwrap(),
            }
        }
        Err(_e) => to_value(&WasmSortResult {
            sorted: vec![],
            trace: WasmTrace {
                steps: vec![],
                comparisons: 0,
                swaps: 0,
            },
        }).unwrap(),
    }
}

/// Obtiene un paso específico de la traza (para navegación VISU-T03).
#[wasm_bindgen]
pub fn trace_get_step(trace: JsValue, step: usize) -> JsValue {
    let wasm_trace: WasmTrace = from_value(trace).unwrap_or_else(|_| WasmTrace {
        steps: vec![],
        comparisons: 0,
        swaps: 0,
    });

    if step < wasm_trace.steps.len() {
        to_value(&wasm_trace.steps[step]).unwrap()
    } else {
        JsValue::NULL
    }
}

/// Convierte SortResult a WasmSortResult.
fn sort_result_to_wasm(result: SortResult) -> WasmSortResult {
    let steps = result.trace.steps.iter().map(|s| WasmTraceStep {
        step: s.step,
        array: s.array.clone(),
        swap: s.swap.as_ref().map(|sw| WasmSwapInfo {
            index: sw.index,
            a: sw.a,
            b: sw.b,
        }),
    }).collect();

    WasmSortResult {
        sorted: result.sorted,
        trace: WasmTrace {
            steps,
            comparisons: result.trace.comparisons,
            swaps: result.trace.swaps,
        },
    }
}
