use crate::instruction::QuantumOp;

pub struct QuantumIROptimizer;

impl QuantumIROptimizer {
    /// Pass za optimizaciju: Ukida uzastopne inverzne kapije (npr. H + H = Identity)
    pub fn optimize(pipeline: Vec<QuantumOp>) -> Vec<QuantumOp> {
        let mut optimized = Vec::new();

        for op in pipeline {
            if let Some(last) = optimized.last() {
                match (last, &op) {
                    (QuantumOp::Hadamard(t1), QuantumOp::Hadamard(t2)) if t1 == t2 => {
                        // H * H = I (Ukida se)
                        optimized.pop();
                        continue;
                    }
                    (
                        QuantumOp::CNOT { control: c1, target: t1 },
                        QuantumOp::CNOT { control: c2, target: t2 },
                    ) if c1 == c2 && t1 == t2 => {
                        // CNOT * CNOT = I (Ukida se)
                        optimized.pop();
                        continue;
                    }
                    _ => {}
                }
            }
            optimized.push(op);
        }

        optimized
    }
}