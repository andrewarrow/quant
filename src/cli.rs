use crate::scheduler::QuantumScheduler;
use crate::instruction::QasmExecutor;
use std::io::{self, Write};

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
        println!("Total physical qubits on the system: {}", self.scheduler.hardware_register.num_qubits);
        println!("Type 'help' for a list of commands or 'exit' to exit.\n");

        loop {
            print!("qos_kernel> ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_err() {
                break;
            }

            let line = input.trim();
            if line.is_empty() { continue; }

            let tokens: Vec<&str> = line.split_whitespace().collect();
            match tokens[0] {
                "exit" | "quit" => {
                    println!("Shutting down the Quantum OS kernel...");
                    break;
                }
                "help" => {
                    println!("Available commands:");
                    println!("  alloc <pid> <count>  - Allocate 'count' cubits to be assigned PID");
                    println!("  run <qasm_string>    -  Executes a QASM command (exapmle. 'run h 0')");
                    println!("  status               - Shows active processes and resource usage");
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
                        Ok(qubits) => println!("Proces PID {} successfully allocated qubits: {:?}", pid, qubits),
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
                        &qasm_code
                    );
                    println!("Completed. Measurement results: {:?}", results);
                }
                "status" => {
                    println!("Active processes: {}", self.scheduler.active_processes.len());
                    for (pid, proc) in &self.scheduler.active_processes {
                        println!(" - PID {}: Allocated qubits {:?}", pid, proc.allocated_qubits);
                    }
                }
                _ => println!("Unknown command. Type 'help' for help."),
            }
        }
    }
}