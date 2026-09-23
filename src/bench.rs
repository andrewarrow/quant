use crate::quantum::QubitRegister;
use std::time::{Duration, Instant};

pub const DEFAULT_STRESS_SECONDS: u64 = 60;
pub const DEFAULT_STRESS_QUBITS: usize = 18;
pub const MIN_STRESS_QUBITS: usize = 2;
pub const MAX_STRESS_QUBITS: usize = 22;

#[derive(Debug)]
pub struct StressReport {
    pub elapsed: Duration,
    pub rounds: u64,
    pub gates: u64,
    pub amplitude_updates: u128,
    pub final_norm: f64,
}

impl StressReport {
    fn print_summary(&self) {
        let throughput = self.amplitude_updates as f64 / self.elapsed.as_secs_f64() / 1e6;

        println!("------------------------------------------------------------");
        println!(
            "  COMPLETE | {:.2}s | {} circuits | {} quantum gates",
            self.elapsed.as_secs_f64(),
            self.rounds,
            self.gates
        );
        println!(
            "  {:.3} billion amplitude updates | {:.1} M amplitudes/s",
            self.amplitude_updates as f64 / 1e9,
            throughput
        );
        println!(
            "  State norm: {:.12} {}",
            self.final_norm,
            if (self.final_norm - 1.0).abs() < 1e-9 {
                "✓"
            } else {
                "!"
            }
        );
        println!("============================================================\n");
    }
}

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

    /// Sustains a reversible state-vector workload for a fixed amount of time.
    pub fn run_timed_stress_test(duration: Duration, qubits: usize) -> StressReport {
        assert!(
            (MIN_STRESS_QUBITS..=MAX_STRESS_QUBITS).contains(&qubits),
            "stress-test qubits must be between {MIN_STRESS_QUBITS} and {MAX_STRESS_QUBITS}"
        );

        let num_states = 1usize << qubits;
        let memory_mib = num_states * std::mem::size_of::<num_complex::Complex64>();
        let memory_mib = memory_mib as f64 / (1024.0 * 1024.0);
        let gates_per_round = (qubits + (qubits - 1) * 2) as u64;
        let updates_per_round = gates_per_round as u128 * num_states as u128;

        println!("\n============================================================");
        println!("  QUANTUM OS: SUSTAINED STATE-VECTOR STRESS TEST");
        println!("============================================================");
        println!(
            "  {} qubits | {:>10} amplitudes | {:>7.2} MiB state vector",
            qubits, num_states, memory_mib
        );
        println!("  Target runtime: {:.1}s", duration.as_secs_f64());
        println!("------------------------------------------------------------");

        let started = Instant::now();
        let mut next_update = Duration::from_secs(1);
        let mut rounds = 0u64;
        let mut gates = 0u64;
        let mut amplitude_updates = 0u128;
        let mut reg = QubitRegister::new(qubits);

        while started.elapsed() < duration {
            // Build a dense superposition, sweep entangling gates in both
            // directions, and keep evolving the same normalized state.
            for target in 0..qubits {
                reg.apply_hadamard_parallel(target);
            }
            for target in 1..qubits {
                reg.apply_cnot(target - 1, target);
            }
            for target in (1..qubits).rev() {
                reg.apply_cnot(target, target - 1);
            }

            rounds += 1;
            gates += gates_per_round;
            amplitude_updates += updates_per_round;

            let elapsed = started.elapsed();
            if elapsed >= next_update && elapsed < duration {
                let throughput = amplitude_updates as f64 / elapsed.as_secs_f64() / 1e6;
                println!(
                    "  {:>5.1}s | {:>6} circuits | {:>9} gates | {:>9.1} M amplitudes/s",
                    elapsed.as_secs_f64(),
                    rounds,
                    gates,
                    throughput
                );
                next_update += Duration::from_secs(1);
            }
        }

        let elapsed = started.elapsed();
        let final_norm: f64 = reg
            .state_vector
            .iter()
            .map(|amplitude| amplitude.norm_sqr())
            .sum();

        let report = StressReport {
            elapsed,
            rounds,
            gates,
            amplitude_updates,
            final_norm,
        };
        report.print_summary();
        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timed_stress_test_runs_and_preserves_normalization() {
        let report = KernelBenchmark::run_timed_stress_test(Duration::from_millis(10), 4);

        assert!(report.rounds > 0);
        assert!(report.gates > 0);
        assert!(report.amplitude_updates > 0);
        assert!(report.elapsed >= Duration::from_millis(10));
        assert!((report.final_norm - 1.0).abs() < 1e-9);
    }
}
