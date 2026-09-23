use crate::bench::{
    DEFAULT_STRESS_QUBITS, DEFAULT_STRESS_SECONDS, KernelBenchmark, MAX_STRESS_QUBITS,
    MIN_STRESS_QUBITS,
};
use crate::instruction::QasmExecutor;
use crate::scheduler::QuantumScheduler;
use std::io::{self, Write};
use std::time::Duration;

pub struct KernelCLI {
    scheduler: QuantumScheduler,
}

impl KernelCLI {
    pub fn new(total_qubits: usize) -> Self {
        Self {
            scheduler: QuantumScheduler::new(total_qubits),
        }
    }

    /// Pokreće interaktivnu CLI školjku kernela
    pub fn start_shell(&mut self) {
        println!("Quantum OS Interactive Shell v1.0.0");
        println!(
            "Total physical qubits on the system: {}",
            self.scheduler.hardware_register.num_qubits
        );
        println!("Type 'help' for a list of commands or 'exit' to exit.\n");

        loop {
            print!("qos_kernel> ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_err() {
                break;
            }

            let line = input.trim();
            if line.is_empty() {
                continue;
            }

            let tokens: Vec<&str> = line.split_whitespace().collect();
            match tokens[0] {
                "exit" | "quit" => {
                    println!("Shutting down the Quantum OS kernel...");
                    break;
                }
                "help" => {
                    println!("Available commands:");
                    println!("  alloc <pid> <count>  - Allocate 'count' cubits to be assigned PID");
                    println!(
                        "  run <qasm_string>    -  Executes a QASM command (exapmle. 'run h 0')"
                    );
                    println!("  status               - Shows active processes and resource usage");
                    println!(
                        "  stress [secs] [qbits]- Sustained state-vector stress test (defaults: 60s, 18 qubits)"
                    );
                    println!("  exit                 - Stops the kernel");
                }
                "alloc" => {
                    if tokens.len() < 3 {
                        println!("Error: Use 'alloc <pid> <count>'");
                        continue;
                    }
                    let pid: usize = tokens[1].parse().unwrap_or(0);
                    let count: usize = tokens[2].parse().unwrap_or(0);

                    match self.scheduler.allocate_qubits(pid, count) {
                        Ok(qubits) => println!(
                            "Proces PID {} successfully allocated qubits: {:?}",
                            pid, qubits
                        ),
                        Err(e) => println!("Allocation failed: {}", e),
                    }
                }
                "run" => {
                    if tokens.len() < 2 {
                        println!("Error: Enter QASM instruction.");
                        continue;
                    }
                    let qasm_code = tokens[1..].join(" ");
                    let results = QasmExecutor::execute_program(
                        &mut self.scheduler.hardware_register,
                        &qasm_code,
                    );
                    println!("Completed. Measurement results: {:?}", results);
                }
                "status" => {
                    println!(
                        "Active processes: {}",
                        self.scheduler.active_processes.len()
                    );
                    for (pid, proc) in &self.scheduler.active_processes {
                        println!(
                            " - PID {}: Allocated qubits {:?}",
                            pid, proc.allocated_qubits
                        );
                    }
                }
                "stress" => match parse_stress_args(&tokens[1..]) {
                    Ok((seconds, qubits)) => {
                        KernelBenchmark::run_timed_stress_test(
                            Duration::from_secs(seconds),
                            qubits,
                        );
                    }
                    Err(error) => println!("Error: {error}"),
                },
                _ => println!("Unknown command. Type 'help' for help."),
            }
        }
    }
}

pub fn parse_stress_args(args: &[&str]) -> Result<(u64, usize), String> {
    if args.len() > 2 {
        return Err("use 'stress [seconds] [qubits]'".to_string());
    }

    let seconds = args
        .first()
        .map(|value| value.parse::<u64>())
        .transpose()
        .map_err(|_| "seconds must be a positive whole number".to_string())?
        .unwrap_or(DEFAULT_STRESS_SECONDS);
    if seconds == 0 {
        return Err("seconds must be greater than zero".to_string());
    }

    let qubits = args
        .get(1)
        .map(|value| value.parse::<usize>())
        .transpose()
        .map_err(|_| "qubits must be a whole number".to_string())?
        .unwrap_or(DEFAULT_STRESS_QUBITS);
    if !(MIN_STRESS_QUBITS..=MAX_STRESS_QUBITS).contains(&qubits) {
        return Err(format!(
            "qubits must be between {MIN_STRESS_QUBITS} and {MAX_STRESS_QUBITS}"
        ));
    }

    Ok((seconds, qubits))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stress_args_have_sixty_second_defaults() {
        assert_eq!(parse_stress_args(&[]), Ok((60, 18)));
    }

    #[test]
    fn stress_args_accept_overrides() {
        assert_eq!(parse_stress_args(&["5", "12"]), Ok((5, 12)));
    }

    #[test]
    fn stress_args_reject_unsafe_values() {
        assert!(parse_stress_args(&["0"]).is_err());
        assert!(parse_stress_args(&["5", "30"]).is_err());
        assert!(parse_stress_args(&["wat"]).is_err());
    }
}
