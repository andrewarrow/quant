
/// Zero-Noise Extrapolation (ZNE) Engine
pub struct ErrorMitigation;

impl ErrorMitigation {
    /// Digitalno skaliranje šuma: Duplira CNOT kapije (CNOT * CNOT = Identity)
    /// čime zadržava identičnu matematiku ali povećava izloženost fizičkom šumu.
    pub fn scale_noise_digital(program: &[crate::instruction::QuantumOp], scale_factor: usize) -> Vec<crate::instruction::QuantumOp> {
        let mut scaled_program = Vec::new();

        for op in program {
            match op {
                crate::instruction::QuantumOp::CNOT { control, target } => {
                    // Za faktor npr. 3, ubacujemo CNOT -> CNOT -> CNOT
                    for _ in 0..scale_factor {
                        scaled_program.push(crate::instruction::QuantumOp::CNOT {
                            control: *control,
                            target: *target,
                        });
                    }
                }
                other => scaled_program.push(other.clone()),
            }
        }

        scaled_program
    }

    /// Ekstrapolacija do nultog šuma pomoću linearne regresije nad rezultatima merenja
    /// (scale_1_result, scale_3_result) -> izračunava vrednost za scale = 0
    pub fn extrapolate_zero_noise(scale_1_expectation: f64, scale_3_expectation: f64) -> f64 {
        // Linearna funkcija: E(scale) = a * scale + b
        // E(1) = a + b
        // E(3) = 3a + b
        // 2a = E(3) - E(1)  =>  a = (E(3) - E(1)) / 2
        // b = E(1) - a      =>  Zero-Noise Expectation E(0)
        let a = (scale_3_expectation - scale_1_expectation) / 2.0;
        let zero_noise_val = scale_1_expectation - a;
        zero_noise_val
    }
}