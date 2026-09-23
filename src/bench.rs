use crate::quantum::QubitRegister;
use std::time::Instant;

pub struct KernelBenchmark;

impl KernelBenchmark {
    /// Pokreće stres-test nad registrima različitih veličina i meri latenciju
    pub fn run_stress_test(max_qubits: usize) {
        println!("\n==================================================");
        println!("  QUANTUM OS KERNEL BENCHMARK & STRESS-TEST MODE  ");
        println!("==================================================");

        for qubits in 2..=max_qubits {
            let num_states = 1 << qubits;
            let memory_bytes = num_states * std::mem::size_of::<num_complex::Complex64>();
            let memory_kb = memory_bytes as f64 / 1024.0;

            let start = Instant::now();
            let mut reg = QubitRegister::new(qubits);

            // Stresiranje procesora paralelnim Hadamardovim i CNOT kapijama
            for target in 0..qubits {
                reg.apply_hadamard_parallel(target);
            }
            
            for i in 0..(qubits - 1) {
                reg.apply_cnot(i, i + 1);
            }

            let duration = start.elapsed();

            println!(
                "Qbits: {:2} | States (2^n): {:8} | RAM usage: {:9.2} KB | Time: {:?}",
                qubits, num_states, memory_kb, duration
            );
        }
        println!("==================================================\n");
    }
}