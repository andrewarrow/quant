use crate::instruction::QuantumOp;

#[derive(Debug, Default)]
pub struct ResourceReport {
    pub total_gates: usize,
    pub t_count: usize,             // T-gates su najskuplje za fault-tolerant računare
    pub cnot_count: usize,
    pub circuit_depth: usize,
    pub estimated_duration_us: f64, // Procenjeno vreme u mikrosekundama
    pub estimated_fidelity: f64,
}

pub struct ResourceEstimator;

impl ResourceEstimator {
    pub fn analyze(program: &[QuantumOp], single_gate_time_ns: f64, cnot_time_ns: f64, gate_error_rate: f64) -> ResourceReport {
        let mut report = ResourceReport::default();
        report.total_gates = program.len();
        report.estimated_fidelity = 1.0;

        for op in program {
            match op {
                QuantumOp::CNOT { .. } => {
                    report.cnot_count += 1;
                    report.estimated_duration_us += cnot_time_ns / 1000.0;
                    report.estimated_fidelity *= 1.0 - (gate_error_rate * 10.0); // CNOT je 10x bučniji
                }
                QuantumOp::Hadamard { .. } => {
                    report.estimated_duration_us += single_gate_time_ns / 1000.0;
                    report.estimated_fidelity *= 1.0 - gate_error_rate;
                }
                _ => {
                    report.t_count += 1;
                    report.estimated_duration_us += single_gate_time_ns / 1000.0;
                    report.estimated_fidelity *= 1.0 - gate_error_rate;
                }
            }
        }

        // Procena dubine kola
        report.circuit_depth = (report.total_gates as f64 / 2.0).ceil() as usize;
        report
    }
}