import init, { validate, sort, trace_get_step } from "./pkg/trazalab_wasm.js";

const inputEl = document.getElementById("input-array");
const validateBtn = document.getElementById("btn-validate");
const sortBtn = document.getElementById("btn-sort");
const resetBtn = document.getElementById("btn-reset");
const validationOutput = document.getElementById("validation-output");
const traceOutput = document.getElementById("trace-output");
const visualization = document.getElementById("visualization");
const wasmConsole = document.getElementById("wasm-console");
const currentStepEl = document.getElementById("current-step");
const totalStepsEl = document.getElementById("total-steps");

const navFirst = document.getElementById("btn-first");
const navFirstSwap = document.getElementById("btn-first-swap");
const navPrev = document.getElementById("btn-prev");
const navNext = document.getElementById("btn-next");
const navLast = document.getElementById("btn-last");

let wasmModule = null;
let currentTrace = null;
let currentStepIndex = 0;

function logWasm(msg) {
  const timestamp = new Date().toISOString();
  wasmConsole.textContent = `[${timestamp}] ${msg}\n` + wasmConsole.textContent;
}

function setValidationOutput(data, isError = false) {
  validationOutput.textContent = JSON.stringify(data, null, 2);
  validationOutput.className = 'output ' + (isError ? 'error' : 'success');
}

function setTraceOutput(step) {
  if (!step) {
    traceOutput.textContent = 'Sin paso seleccionado';
    return;
  }
  traceOutput.textContent = JSON.stringify(step, null, 2);
}

function renderVisualization(array) {
  visualization.innerHTML = '';
  if (!array || array.length === 0) return;

  const maxVal = Math.max(...array);
  array.forEach((val, idx) => {
    const bar = document.createElement('div');
    bar.className = 'bar';
    bar.style.height = `${(val / maxVal) * 100}%`;
    bar.style.minWidth = '20px';
    bar.title = `Índice ${idx}: ${val}`;
    visualization.appendChild(bar);
  });
}

function enableControls(validated = false, hasTrace = false) {
  validateBtn.disabled = false;
  sortBtn.disabled = !validated;
  resetBtn.disabled = !validated && !hasTrace;
  
  const hasSteps = hasTrace && currentTrace && currentTrace.steps.length > 0;
  navFirst.disabled = !hasSteps;
  navFirstSwap.disabled = !hasSteps;
  navPrev.disabled = !hasSteps || currentStepIndex === 0;
  navNext.disabled = !hasSteps || currentStepIndex >= currentTrace.steps.length - 1;
  navLast.disabled = !hasSteps;
}

function updateStepDisplay() {
  if (!currentTrace) {
    currentStepEl.textContent = '0';
    totalStepsEl.textContent = '0';
    return;
  }
  currentStepEl.textContent = currentStepIndex.toString();
  totalStepsEl.textContent = (currentTrace.steps.length - 1).toString();
}

async function initWasm() {
  try {
    await init("./pkg/trazalab_wasm_bg.wasm");
    wasmModule = { validate, sort, trace_get_step };
    logWasm("Módulo Wasm inicializado correctamente");
    inputEl.value = "5, 2, 9, 1, 7, 3";
    enableControls(false, false);
  } catch (e) {
    logWasm(`Error inicializando Wasm: ${e}`);
    validationOutput.textContent = `Error cargando Wasm: ${e}`;
    validationOutput.className = 'output error';
  }
}

function parseInput() {
  const text = inputEl.value.trim();
  if (!text) return [];
  return text.split(',').map(s => s.trim()).filter(s => s !== '');
}

function inputToJsonValues() {
  const parts = parseInput();
  return parts.map(p => {
    const num = Number(p);
    if (Number.isInteger(num) && p !== '') return num;
    return p; // devolver string si no es entero válido
  });
}

validateBtn.addEventListener("click", () => {
  if (!wasmModule) return;
  const values = inputToJsonValues();
  const result = wasmModule.validate(values);
  const data = result;
  setValidationOutput(data, !data.valid);
  
  if (data.valid) {
    logWasm(`Validación OK: [${data.values.join(', ')}]`);
    enableControls(true, false);
  } else {
    logWasm(`Validación fallida: ${data.error?.code} - ${data.error?.message}`);
    enableControls(false, false);
  }
});

sortBtn.addEventListener("click", () => {
  if (!wasmModule) return;
  const values = inputToJsonValues();
  logWasm(`Ejecutando ordenamiento...`);
  const result = wasmModule.sort(values);
  const data = result;
  
  if (data.trace && data.trace.steps.length > 0) {
    currentTrace = data.trace;
    currentStepIndex = 0;
    setTraceOutput(currentTrace.steps[0]);
    renderVisualization(currentTrace.steps[0].array);
    updateStepDisplay();
    enableControls(true, true);
    logWasm(`Ordenamiento completado: ${data.trace.swaps} intercambios, ${data.trace.comparisons} comparaciones`);
  }
});

resetBtn.addEventListener("click", () => {
  currentTrace = null;
  currentStepIndex = 0;
  validationOutput.textContent = '';
  validationOutput.className = 'output';
  traceOutput.textContent = '';
  visualization.innerHTML = '';
  wasmConsole.textContent = '';
  updateStepDisplay();
  enableControls(false, false);
  logWasm("Reset completado");
});

// Navegación de traza
function navigateToStep(stepIndex) {
  if (!currentTrace || stepIndex < 0 || stepIndex >= currentTrace.steps.length) return;
  currentStepIndex = stepIndex;
  const step = currentTrace.steps[stepIndex];
  setTraceOutput(step);
  renderVisualization(step.array);
  updateStepDisplay();
  navPrev.disabled = currentStepIndex === 0;
  navNext.disabled = currentStepIndex >= currentTrace.steps.length - 1;
}

navFirst.addEventListener("click", () => navigateToStep(0));

navFirstSwap.addEventListener("click", () => {
  if (!currentTrace) return;
  const firstSwapIdx = currentTrace.steps.findIndex(s => s.swap !== null);
  if (firstSwapIdx !== -1) navigateToStep(firstSwapIdx);
});

navPrev.addEventListener("click", () => navigateToStep(currentStepIndex - 1));
navNext.addEventListener("click", () => navigateToStep(currentStepIndex + 1));
navLast.addEventListener("click", () => {
  if (currentTrace) navigateToStep(currentTrace.steps.length - 1);
});

// Inicializar
initWasm();
