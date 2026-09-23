use crate::quantum::QubitRegister;

pub struct DynamicExecutor;

impl DynamicExecutor {
    /// Izvršava skokove i klasično uslovljene kapije tokom samog kvantnog izvršavanja
    pub fn execute_dynamic_branch(reg: &mut QubitRegister, measure_qubit: usize, target_qubit: usize) -> bool {
        // 1. Merenje kubita u sredini kola (Mid-circuit measurement)
        let outcome = reg.measure(measure_qubit);
        println!(" [DYNAMIC ENGINE] Mid-Circuit Measurement on Qubit {}: Outcome = {}", measure_qubit, outcome);

        // 2. Feed-forward logika u nanosekundama: Ako je izmereno |1>, primeni X kapiju na target
        if outcome == 1 {
            println!(" [DYNAMIC ENGINE] Fast Feedback Triggered! Applying Feed-Forward X-Gate on Qubit {}", target_qubit);
            let x_gate = [
                num_complex::Complex64::new(0.0, 0.0), num_complex::Complex64::new(1.0, 0.0),
                num_complex::Complex64::new(1.0, 0.0), num_complex::Complex64::new(0.0, 0.0),
            ];
            reg.apply_gate_1q(target_qubit, &x_gate);
            true
        } else {
            println!(" [DYNAMIC ENGINE] Feedback Condition False. Bypassing Target Gate.");
            false
        }
    }
}