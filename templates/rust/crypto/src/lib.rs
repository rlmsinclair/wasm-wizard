// {{name}} - High-Performance WebAssembly Cryptography
// Created with WASM Wizard - npm install -g wasm-wizard

use wasm_bindgen::prelude::*;
use sha2::{Sha256, Digest};

#[wasm_bindgen]
pub struct PerformanceTimer {
    start: f64,
}

#[wasm_bindgen]
impl PerformanceTimer {
    #[wasm_bindgen(constructor)]
    pub fn new() -> PerformanceTimer {
        let window = web_sys::window().unwrap();
        let performance = window.performance().unwrap();
        PerformanceTimer {
            start: performance.now(),
        }
    }

    pub fn elapsed(&self) -> f64 {
        let window = web_sys::window().unwrap();
        let performance = window.performance().unwrap();
        performance.now() - self.start
    }
}

#[wasm_bindgen]
pub fn sha256_hash(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    hex::encode(hasher.finalize())
}

#[wasm_bindgen]
pub fn sha256_batch(inputs: Vec<String>) -> Vec<String> {
    inputs.into_iter()
        .map(|input| sha256_hash(&input))
        .collect()
}

#[wasm_bindgen]
pub struct HashResult {
    iteration: u32,
    hash: String,
}

#[wasm_bindgen]
impl HashResult {
    #[wasm_bindgen(getter)]
    pub fn iteration(&self) -> u32 {
        self.iteration
    }

    #[wasm_bindgen(getter)]
    pub fn hash(&self) -> String {
        self.hash.clone()
    }
}

#[wasm_bindgen]
pub fn sha256_intensive(base_text: &str, iterations: u32) -> Vec<HashResult> {
    let mut results = Vec::new();
    let mut current = base_text.to_string();
    
    for i in 0..iterations {
        current = sha256_hash(&current);
        if i % 100 == 0 || i == iterations - 1 {
            results.push(HashResult {
                iteration: i,
                hash: current.clone(),
            });
        }
    }
    
    results
}

#[wasm_bindgen]
pub fn sha256_memory_test(size_mb: usize) -> String {
    let size = size_mb * 1024 * 1024;
    let data: Vec<u8> = (0..size).map(|i| (i % 256) as u8).collect();
    
    let mut hasher = Sha256::new();
    hasher.update(&data);
    hex::encode(hasher.finalize())
}

#[wasm_bindgen]
pub fn fibonacci_wasm(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut a = 0;
            let mut b = 1;
            for _ in 2..=n {
                let temp = a + b;
                a = b;
                b = temp;
            }
            b
        }
    }
}