use serde::{Deserialize, Serialize};

/// Información de un intercambio realizado durante el ordenamiento.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SwapInfo {
    /// Índice del primer elemento intercambiado (i e i+1)
    pub index: usize,
    /// Valor que estaba en index antes del swap
    pub a: i32,
    /// Valor que estaba en index+1 antes del swap
    pub b: i32,
}

/// Un paso individual en la traza de ordenamiento.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceStep {
    /// Número de paso (0 = estado inicial, sin intercambio)
    pub step: usize,
    /// Estado completo del arreglo tras este paso
    pub array: Vec<i32>,
    /// Intercambio realizado en este paso (None si no hubo intercambio)
    pub swap: Option<SwapInfo>,
}

/// Traza completa de una ejecución de ordenamiento.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Trace {
    pub steps: Vec<TraceStep>,
    pub comparisons: usize,
    pub swaps: usize,
}

impl Trace {
    /// Crea una traza vacía para un array inicial.
    pub fn new(initial_array: Vec<i32>) -> Self {
        Self {
            steps: vec![TraceStep {
                step: 0,
                array: initial_array,
                swap: None,
            }],
            comparisons: 0,
            swaps: 0,
        }
    }

    /// Registra un paso con intercambio.
    pub fn record_swap(&mut self, array: Vec<i32>, index: usize, a: i32, b: i32) {
        self.comparisons += 1;
        self.swaps += 1;
        self.steps.push(TraceStep {
            step: self.steps.len(),
            array,
            swap: Some(SwapInfo { index, a, b }),
        });
    }

    /// Registra un paso sin intercambio (comparación solamente).
    pub fn record_compare(&mut self, array: Vec<i32>) {
        self.comparisons += 1;
        self.steps.push(TraceStep {
            step: self.steps.len(),
            array,
            swap: None,
        });
    }

    /// Obtiene un paso específico por índice (para navegación VISU-T03).
    pub fn get_step(&self, step: usize) -> Option<&TraceStep> {
        self.steps.get(step)
    }

    /// Último paso (estado final ordenado).
    pub fn final_step(&self) -> &TraceStep {
        self.steps.last().expect("Trace always has at least initial step")
    }

    /// Primer intercambio (step 1 si existe).
    pub fn first_swap_step(&self) -> Option<&TraceStep> {
        self.steps.iter().find(|s| s.swap.is_some())
    }

    /// Número total de pasos registrados.
    pub fn len(&self) -> usize {
        self.steps.len()
    }
}
