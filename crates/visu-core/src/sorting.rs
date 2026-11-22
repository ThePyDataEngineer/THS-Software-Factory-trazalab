use crate::error::SortError;
use crate::trace::{Trace, TraceStep, SwapInfo};
use crate::validation::ValidatedInput;

/// Resultado completo de una ordenación.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SortResult {
    pub sorted: Vec<i32>,
    pub trace: Trace,
}

/// Ordenamiento burbuja determinista con generación de traza completa.
///
/// Variante: burbuja clásico con bandera `swapped` para salida temprana.
/// Complejidad: O(n²) peor caso, O(n) mejor caso (ya ordenado).
///
/// Formato de traza:
/// - Paso 0: estado inicial
/// - Por cada intercambio: paso con swap
/// - Al final de cada pasada con intercambios: paso sin swap (estado tras pasada)
/// - Salida temprana: paso final sin swap
pub fn bubble_sort(input: ValidatedInput) -> Result<SortResult, SortError> {
    let mut array = input.values;
    let n = array.len();
    let mut trace = Trace::new(array.clone());

    if n <= 1 {
        return Ok(SortResult { sorted: array, trace });
    }

    for i in 0..n {
        let mut swapped = false;

        for j in 0..n.saturating_sub(1).saturating_sub(i) {
            if array[j] > array[j + 1] {
                let a = array[j];
                let b = array[j + 1];
                array.swap(j, j + 1);
                trace.record_swap(array.clone(), j, a, b);
                swapped = true;
            }
        }

        if swapped {
            // Paso de fin de pasada: estado tras la pasada, sin intercambio
            trace.record_compare(array.clone());
        } else {
            // Salida temprana: array ya ordenado
            trace.record_compare(array.clone());
            break;
        }
    }

    Ok(SortResult {
        sorted: array,
        trace,
    })
}

/// Ejecuta ordenamiento burbuja directamente desde un slice (para tests).
pub fn bubble_sort_slice(input: &[i32]) -> Result<SortResult, SortError> {
    let validated = crate::validation::validate_slice(input)?;
    bubble_sort(validated)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::trace::TraceStep;

    #[test]
    fn test_bubble_sort_basic() {
        let input = vec![5, 2, 9, 1, 7, 3];
        let result = bubble_sort_slice(&input).unwrap();
        assert_eq!(result.sorted, vec![1, 2, 3, 5, 7, 9]);
        // Traza: paso 0 (inicial) + 4 swaps pass1 + 1 pass-end + 2 swaps pass2 + 1 pass-end + 2 swaps pass3 + 1 pass-end + 1 early-exit = 13 pasos
        assert_eq!(result.trace.steps.len(), 13);
        assert_eq!(result.trace.swaps, 8);
    }

    #[test]
    fn test_bubble_sort_already_sorted() {
        let input = vec![1, 2, 3, 4, 5];
        let result = bubble_sort_slice(&input).unwrap();
        assert_eq!(result.sorted, vec![1, 2, 3, 4, 5]);
        // Solo step 0 + 1 comparación (early exit)
        assert!(result.trace.steps.len() >= 2);
    }

    #[test]
    fn test_bubble_sort_duplicates() {
        let input = vec![3, 1, 3, 2];
        let result = bubble_sort_slice(&input).unwrap();
        assert_eq!(result.sorted, vec![1, 2, 3, 3]);
    }

    #[test]
    fn test_trace_step_zero_is_initial() {
        let input = vec![5, 2, 9, 1, 7, 3];
        let result = bubble_sort_slice(&input).unwrap();
        let step0 = result.trace.get_step(0).unwrap();
        assert_eq!(step0.array, vec![5, 2, 9, 1, 7, 3]);
        assert!(step0.swap.is_none());
    }

    #[test]
    fn test_trace_first_swap() {
        let input = vec![5, 2, 9, 1, 7, 3];
        let result = bubble_sort_slice(&input).unwrap();
        let first_swap = result.trace.first_swap_step().unwrap();
        assert_eq!(first_swap.array, vec![2, 5, 9, 1, 7, 3]);
        assert!(first_swap.swap.is_some());
        let swap = first_swap.swap.as_ref().unwrap();
        assert_eq!(swap.index, 0);
        assert_eq!(swap.a, 5);
        assert_eq!(swap.b, 2);
    }

    #[test]
    fn test_trace_final_step() {
        let input = vec![5, 2, 9, 1, 7, 3];
        let result = bubble_sort_slice(&input).unwrap();
        let final_step = result.trace.final_step();
        assert_eq!(final_step.array, vec![1, 2, 3, 5, 7, 9]);
        assert!(final_step.swap.is_none());
    }

    #[test]
    fn test_trace_navigation_t03() {
        let input = vec![5, 2, 9, 1, 7, 3];
        let result = bubble_sort_slice(&input).unwrap();
        let trace = &result.trace;

        // Final
        let final_step = trace.final_step();
        assert_eq!(final_step.array, vec![1, 2, 3, 5, 7, 9]);

        // Primer intercambio
        let first_swap = trace.first_swap_step().unwrap();
        assert_eq!(first_swap.array, vec![2, 5, 9, 1, 7, 3]);

        // Regreso al final - mismo estado lógico
        let final_step_again = trace.final_step();
        assert_eq!(final_step_again.array, vec![1, 2, 3, 5, 7, 9]);
    }
}
