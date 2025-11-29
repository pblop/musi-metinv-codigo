use std::slice;

use rand::Rng;
use wasm_bindgen::prelude::*;

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
use console_error_panic_hook;

#[wasm_bindgen]
pub fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

#[wasm_bindgen]
pub fn eratostenes(limit: u32) -> u32 {
    let mut primes = Vec::new();
    let lim: usize = limit.try_into().unwrap();
    let mut is_prime = vec![true; lim + 1];
    is_prime[0] = false;
    if lim >= 1 {
        is_prime[1] = false;
    }

    // let mut donotoptimizepls = 0;
    for num in 2..=lim {
        if is_prime[num] {
            primes.push(num);
            let mut multiple = num * num;
            while multiple <= lim {
                is_prime[multiple] = false;
                multiple += num;
            }
            // donotoptimizepls *= num + i % 1234567; // Just to avoid optimizations
        }
    }

    // primes.len() + donotoptimizepls
    primes.len().try_into().unwrap()
}

#[wasm_bindgen]
pub fn quicksort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }

    let pivot_index = partition(arr);

    let (left_slice, right_slice) = arr.split_at_mut(pivot_index);

    quicksort(left_slice);
    quicksort(&mut right_slice[1..]);
}

fn partition(arr: &mut [i32]) -> usize {
    let pivot_index = arr.len() - 1;
    let mut i = 0;

    for j in 0..pivot_index {
        if arr[j] <= arr[pivot_index] {
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, pivot_index);

    i
}

#[wasm_bindgen]
pub fn montecarlo(points: f64) -> f64 {
    let mut rng = rand::rng();
    let mut inside_circle = 0;

    for _ in 0..points as u64 {
        let x: f64 = rng.random();
        let y: f64 = rng.random();

        if x * x + y * y <= 1.0 {
            inside_circle += 1;
        }
    }

    (inside_circle as f64) / (points as f64) * 4.0
}

#[wasm_bindgen]
pub fn matrix_multiply(a: &[f64], b: &[f64], c: &mut [f64], a_width: usize) {
    let a_height = a.len() / a_width;
    let b_width = a_height;
    let b_height = b.len() / b_width;
    assert_eq!(
        a_width, b_height,
        "incompatible matrix dimensions: a: {}x{}, b: {}x{}",
        a_height, a_width, b_height, b_width
    );
    assert_eq!(
        c.len() >= a_height * b_width,
        true,
        "output matrix has incorrect size: expected at least {}, got {}",
        a_height * b_width,
        c.len()
    );

    for i in 0..a_height {
        for j in 0..b_width {
            let mut sum = 0.0;
            for k in 0..a_width {
                // index = row * width + col
                sum += a[i * a_width + k] * b[k * b_width + j];
            }
            c[i * b_width + j] = sum;
        }
    }
}

#[wasm_bindgen]
pub fn output_matrix_size(a_width: usize, a_len: usize) -> usize {
    let a_height = a_len / a_width;
    let b_len = a_len;

    let b_width = a_height;

    a_height * b_width
}

// Another wrapper to handle raw pointers for WASM interop
#[wasm_bindgen]
pub fn matrix_multiply_in_memory(
    a_ptr: *const f64,
    b_ptr: *const f64,
    c_ptr: *mut f64,
    a_len: usize,
    b_len: usize,
    c_len: usize,
    width: usize,
) {
    let a = unsafe { slice::from_raw_parts(a_ptr, a_len) };
    let b = unsafe { slice::from_raw_parts(b_ptr, b_len) };
    let c = unsafe { slice::from_raw_parts_mut(c_ptr, c_len) };
    assert_ne!(
        c.len(),
        0,
        "Output matrix length must be greater than zero, got {}, args: a_len {}, b_len {} c_len {} width {}",
        c.len(),
        a_len,
        b_len,
        c_len,
        width
    );

    matrix_multiply(a, b, c, width);
}

#[wasm_bindgen]
pub fn nqueens(n: u32) -> u32 {
    let mut board = vec![-1; n as usize]; // board[row] = col
    let mut count: u32 = 0;

    // Check if placing a queen at (row, col) is safe
    fn is_safe(board: &[i32], row: u32, col: i32) -> bool {
        for prev_row in 0..row {
            let prev_col = board[prev_row as usize];
            // For each row before the current one, get the column of the placed
            // queen. If the current column matches (horizontal conflict), or if
            // there's a diagonal conflict (distance in rows equals distance in
            // columns), there's a conflict.
            if prev_col == col || (prev_col - col).abs() == (prev_row as i32 - row as i32).abs() {
                return false;
            }
        }
        true
    }

    // Recursive solver
    fn place_queen(n: u32, row: u32, board: &mut Vec<i32>, count: &mut u32) {
        // If we've placed queens in all rows, we've found a solution
        if row == n {
            *count += 1;
            return;
        }
        for col in 0..n as i32 {
            if is_safe(board, row, col) {
                board[row as usize] = col;
                place_queen(n, row + 1, board, count);
                // No explicit backtrack needed as we overwrite board[row] next iter
            }
        }
    }

    place_queen(n, 0, &mut board, &mut count);
    count
}

#[wasm_bindgen]
pub fn mandelbrot(width: u32, height: u32, max_iter: u32) -> Vec<u32> {
    let mut output = vec![0; (width * height) as usize];

    for y in 0..height as usize {
        for x in 0..width as usize {
            // Map pixel to complex plane
            let cx = (x as f64 / width as f64) * 3.5 - 2.5;
            let cy = (y as f64 / height as f64) * 2.0 - 1.0;

            let mut zx = 0.0;
            let mut zy = 0.0;
            let mut iter_count = 0;

            // z = z*z + c check
            while (zx * zx + zy * zy) <= 4.0 && iter_count < max_iter {
                let temp = zx * zx - zy * zy + cx;
                zy = 2.0 * zx * zy + cy;
                zx = temp;
                iter_count += 1;
            }
            output[y * width as usize + x] = iter_count;
        }
    }
    output
}

#[wasm_bindgen]
pub fn mandelbrot_in_memory(output_ptr: *mut u32, width: u32, height: u32, max_iter: u32) {
    let output = unsafe { slice::from_raw_parts_mut(output_ptr, (width * height) as usize) };
    let mandelbrot_data = mandelbrot(width, height, max_iter);
    output.copy_from_slice(&mandelbrot_data);
}

// ---- WASM-EXPORTED WRAPPER ----
// This is the function we will call from JavaScript.
#[wasm_bindgen]
pub fn sort_in_memory(ptr: *mut i32, len: usize) {
    // Unsafely create a mutable slice from the raw pointer and length.
    // This is a "bridge" from the C-like world (pointers) to the Rust world (slices).
    // It's `unsafe` because Rust can't guarantee the pointer and length are valid.
    let arr = unsafe { slice::from_raw_parts_mut(ptr, len) };

    // Now we can call our safe, idiomatic quicksort function.
    quicksort(arr);
}

// ---- MEMORY MANAGEMENT HELPERS ----
// It's good practice to manage memory from the same side it's allocated.

/// Allocates a memory buffer of `i32`s in Rust's memory and returns a pointer to it.
#[wasm_bindgen]
pub fn alloc(len: usize) -> *mut i32 {
    // Create a vector with the given capacity.
    let mut buf = Vec::with_capacity(len);
    // Get a pointer to the vector's data.
    let ptr = buf.as_mut_ptr();
    // "Forget" about the vector so Rust doesn't deallocate it when this function ends.
    // We are giving ownership of this memory to the JavaScript caller.
    std::mem::forget(buf);
    // Return the pointer.
    ptr
}
#[wasm_bindgen]
pub fn allocf64(len: usize) -> *mut f64 {
    // Create a vector with the given capacity.
    let mut buf = Vec::with_capacity(len);
    // Get a pointer to the vector's data.
    let ptr = buf.as_mut_ptr();
    // "Forget" about the vector so Rust doesn't deallocate it when this function ends.
    // We are giving ownership of this memory to the JavaScript caller.
    std::mem::forget(buf);
    // Return the pointer.
    ptr
}

/// Frees the memory buffer that was allocated by `alloc`.
#[wasm_bindgen]
pub fn dealloc(ptr: *mut i32, len: usize) {
    unsafe {
        // Re-create the Vec from the pointer and length, allowing Rust to reclaim the memory.
        let _ = Vec::from_raw_parts(ptr, 0, len);
    }
}

#[wasm_bindgen]
pub fn deallocf64(ptr: *mut f64, len: usize) {
    unsafe {
        // Re-create the Vec from the pointer and length, allowing Rust to reclaim the memory.
        let _ = Vec::from_raw_parts(ptr, 0, len);
    }
}

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}
