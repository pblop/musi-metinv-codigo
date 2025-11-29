import init, {fibonacci, sort_in_memory, eratostenes, alloc, dealloc, montecarlo,
  nqueens, matrix_multiply_in_memory, allocf64, deallocf64, mandelbrot_in_memory, output_matrix_size,
  init_panic_hook
} from './rust_benchs_pkg/rust_benchs.js';

const {memory} = await init();

init_panic_hook();

const WARMUP_ITERS = 100;
const WARMUP_TIME = 30000; // ms

function benchmark(executions, name, fun, ...args) {
  console.warn(`Benchmarking ${name}, executing ${executions} times...`);
  document.title = `${name} (${executions})...`;
  const times = [];
  let accumulator = 0;
  let i = 0;

  // Warmup phase, to avoid initial jit/wasm costs, max 100 iterations or 3 seconds
  const warmup_beginning = performance.now();
  while (true) {
    const result = fun(...args);
    const end = performance.now();
    
    // Accumulate result to avoid being optimized out
    accumulator += result;
    i++;
    if (i === WARMUP_ITERS || end - warmup_beginning >= WARMUP_TIME) break
  }
  const warmup_end = performance.now();
  console.warn(`Warmup done, executed ${i} iterations (${(warmup_end - warmup_beginning).toFixed(2)} ms)`);

  for (i = 0; i < executions; i++) {
    const start = performance.now();
    const result = fun(...args);
    const end = performance.now();

    // Accumulate result to avoid being optimized out
    accumulator += result;

    times.push(end - start);
    document.title = `${name} (${i}/${executions})...`;
  }

  console.warn(`Checksum: ${accumulator}`);
  const avg = times.reduce((a, b) => a + b, 0) / times.length;
  const min = times.reduce((a, b) => Math.min(a, b), Number.POSITIVE_INFINITY);
  const max = times.reduce((a, b) => Math.max(a, b), Number.NEGATIVE_INFINITY);
  console.warn(`Benchmark of ${name} done, executed ${i} times, checksum ${accumulator}
    avg: ${(avg * 1e6).toFixed(2)}ns, min: ${(min * 1e6).toFixed(2)}ns, max: ${(max * 1e6).toFixed(2)}ns`);

  console.log(JSON.stringify({ fun: name, times}));
  console.warn(`${name} benchmark done.`);
}

function benchmark_2(executions, name, fun, ...args) {
  console.warn(`Benchmarking ${name}, executing ${executions} times...`);
  document.title = `${name} (${executions})...`;
  const times = [];
  let i = 0;

  // Warmup phase, to avoid initial jit/wasm costs, max 100 iterations or 3 seconds
  let warmup_time = 0;
  while (true) {
    // Accumulate result to avoid being optimized out
    warmup_time += fun(...args);
    
    i++;
    if (i === WARMUP_ITERS || warmup_time >= WARMUP_TIME) break
  }

  console.warn(`Warmup time: ${warmup_time}`);
  let time_acc = 0;
  let accumulator = 0;
  for (i = 0; i < executions; i++) {
    const [iter_time, sorted_array] = fun(...args);
    time_acc += iter_time;

    times.push(iter_time);
    for (const v of sorted_array) {
      accumulator += v;
    }
    document.title = `${name} (${i}/${executions})...`;
  }

  const avg = times.reduce((a, b) => a + b, 0) / times.length;
  const min = times.reduce((a, b) => Math.min(a, b), Number.POSITIVE_INFINITY);
  const max = times.reduce((a, b) => Math.max(a, b), Number.NEGATIVE_INFINITY);
  console.warn(`Benchmark of ${name} done, executed ${i} times, time: ${time_acc},
    avg: ${(avg * 1e6).toFixed(2)}ns, min: ${(min * 1e6).toFixed(2)}ns, max: ${(max * 1e6).toFixed(2)}ns`);
  console.warn(`Checksum: ${accumulator}`);

  console.log(JSON.stringify({ fun: name, times}));
  console.warn(`${name} benchmark done.`);
}

function qsort_with_alloc(arr) {
  const jsData = new Int32Array(arr);
  const pointer = alloc(jsData.length);
  const wasmArray = new Int32Array(memory.buffer, pointer, jsData.length);
  wasmArray.set(jsData);
  const start = performance.now();
  sort_in_memory(pointer, jsData.length);
  const end = performance.now();
  const sortedJsArray = Array.from(wasmArray);
  dealloc(pointer, jsData.length);
  // Print first 10 elements of sorted array to prevent optimization
  return [end - start, sortedJsArray.slice(0, 10)];
}

function alloc_matrix_multiply_args(a, b, width) {
  const output_size = output_matrix_size(width, a.length);

  const jsDataA = new Float64Array(a);
  const jsDataB = new Float64Array(b);
  const pointer_a = allocf64(jsDataA.length);
  const pointer_b = allocf64(jsDataB.length);
  const pointer_c = allocf64(output_size);

  const wasmArrayA = new Float64Array(memory.buffer, pointer_a, jsDataA.length);
  const wasmArrayB = new Float64Array(memory.buffer, pointer_b, jsDataB.length);
  const wasmArrayC = new Float64Array(memory.buffer, pointer_c, output_size);
  wasmArrayA.set(jsDataA);
  wasmArrayB.set(jsDataB);
  return [pointer_a, wasmArrayA, pointer_b, wasmArrayB, pointer_c, wasmArrayC];
}

function dealloc_matrix_multiply_args(allocated_args) {
  const [pointer_a, wasmArrayA, pointer_b, wasmArrayB, pointer_c, wasmArrayC] = allocated_args;
  deallocf64(pointer_a, wasmArrayA.length);
  deallocf64(pointer_b, wasmArrayB.length);
  deallocf64(pointer_c, wasmArrayC.length);
}

function matrix_multiply_with_alloc(allocated_args, width) {
  const [pointer_a, wasmArrayA, pointer_b, wasmArrayB, pointer_c, wasmArrayC] = allocated_args;

  const start = performance.now();
  matrix_multiply_in_memory(pointer_a, pointer_b, pointer_c, wasmArrayA.length, wasmArrayB.length, wasmArrayC.length, width);
  const end = performance.now();
  const resultArray = Array.from(wasmArrayC);

  // Print random 10 elements of result array to prevent optimization
  const returnArray = [];
  for (let i = 0; i < 10; i++) {
    const index = Math.floor(Math.random() * resultArray.length);
    returnArray.push(resultArray[index]);
  }
  return [end - start, returnArray];
}

function mandelbrot_with_alloc(width, height, max_iter) {
  const pointer = alloc(width * height);

  const start = performance.now();
  mandelbrot_in_memory(pointer, width, height, max_iter);
  const end = performance.now();
  const wasmArray = new Uint32Array(memory.buffer, pointer, width * height);
  const resultArray = Array.from(wasmArray);
  dealloc(pointer, width * height);

  // Print random 10 elements of result array to prevent optimization
  const returnArray = [];
  for (let i = 0; i < 10; i++) {
    const index = Math.floor(Math.random() * resultArray.length);
    returnArray.push(resultArray[index]);
  }
  return [end - start, returnArray];
}

console.warn("Started...");
const testlist = await (await fetch('testlist')).json();
if (!testlist) {
  console.error("No testlist found!");
  throw new Error("No testlist found!");
}

console.warn(`Loaded testlist with ${testlist.length} tests.`);
const tests = [];
for (const testfile of testlist) {
  console.warn(`Loading test file "${testfile}"...`);
  const testdata = await (await fetch(`tests/${testfile}`)).json();
  tests.push(testdata);
}

console.warn(`Loaded ${tests.length} tests.`);
const functions = {
  "fibonacci": fibonacci,
  "eratostenes": eratostenes,
  "quicksort": qsort_with_alloc,
  "montecarlo": montecarlo,
  "nqueens": nqueens,
  "matrix_multiply": matrix_multiply_with_alloc,
  "mandelbrot": mandelbrot_with_alloc,
}

for (let i = 0; i < tests.length; i++) {
  const test = tests[i];
  console.warn(`Running test: ${test.fun}, idx: ${i}`);
  const func = functions[test.fun];
  const {arg, name, executions} = test;

  let benchmarking_func;
  if (test.type === 1 || test.type === 3)
    benchmarking_func = benchmark;
  else if (test.type === 2 || test.type === 4 || test.type === 5)
    benchmarking_func = benchmark_2;

  if (test.type === 4) {
    const {a, b, width} = arg;
    const allocated_args = alloc_matrix_multiply_args(a, b, width);
    benchmarking_func(executions, name, func, allocated_args, width);
    dealloc_matrix_multiply_args(allocated_args);
    
  } else if (test.type === 5) {
    const {width, height, max_iter} = arg;
    benchmarking_func(executions, name, func, width, height, max_iter);
  }else {
    benchmarking_func(executions, name, func, arg);
  }

}
console.log('DONE.');
