use crate::quantum::QubitRegister;

#[derive(Debug, Clone)]
pub enum QuantumOp {
    Hadamard (usize),
    CNOT { control: usize, target: usize },
    Measure(usize),
}

pub struct QasmExecutor;

impl QasmExecutor {
    /// Parsira jednostavne QASM instrukcije u izvršne operacije
    pub fn parse_line(line: &str) -> Option<QuantumOp> {
        let tokens: Vec<&str> = line.trim().split_whitespace().collect();
        if tokens.is_empty() { return None; }

        match tokens[0] {
            "h" | "H" => {
                let target = tokens.get(1)?.parse::<usize>().ok()?;
                Some(QuantumOp::Hadamard(target))
            }
            "cnot" | "CNOT" => {
                let control = tokens.get(1)?.parse::<usize>().ok()?;
                let target = tokens.get(2)?.parse::<usize>().ok()?;
                Some(QuantumOp::CNOT { control, target })
            }
            "measure" | "MEASURE" => {
                let target = tokens.get(1)?.parse::<usize>().ok()?;
                Some(QuantumOp::Measure(target))
            }
            _ => None,
        }
    }

    /// Izvršava niz QASM komandi nad zadatim registrom
    pub fn execute_program(register: &mut QubitRegister, program: &str) -> Vec<(usize, u8)> {
        let mut results = Vec::new();

        for line in program.lines() {
            if let Some(op) = Self::parse_line(line) {
                match op {
                    QuantumOp::Hadamard(target) => {
                        // Koristimo paralelnu verziju za optimalne performanse
                        register.apply_hadamard_parallel(target);
                    }
                    QuantumOp::CNOT { control, target } => {
                        register.apply_cnot(control, target);
                    }
                    QuantumOp::Measure(target) => {
                        let outcome = register.measure(target);
                        results.push((target, outcome));
                    }
                }
            }
        }

        results
    }
}