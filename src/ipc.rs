use crate::quantum::QubitRegister;

pub struct QuantumIPCChannel;

impl QuantumIPCChannel {
    /// Teleportuje stanje kubita `src_qubit` iz procesa A na `dest_qubit` u procesu B.
    /// Koristi `entangled_a` i `entangled_b` kao EPR par podeljen između procesa.
    pub fn teleport(
        register: &mut QubitRegister,
        src_qubit: usize,
        entangled_a: usize,
        _entangled_b: usize,
        dest_qubit: usize,
    ) -> Result<(), String> {
        // 1. CNOT između izvornog kubita i EPR para procesa A
        register.apply_cnot(src_qubit, entangled_a);

        // 2. Hadamardova kapija na izvornom kubitu
        register.apply_hadamard(src_qubit);

        // 3. Merenje oba kubita na strani pošiljaoca (Bell measurement)
        let m1 = register.measure(src_qubit);
        let m2 = register.measure(entangled_a);

        // 4. Klasični prenos 2 bita i korekcija stanja na strani primaoca
        if m2 == 1 {
            // Pauli-X korekcija (CNOT sa referentnim stanjem 1)
            let size = register.state_vector.len();
            for i in 0..size {
                if (i & (1 << dest_qubit)) == 0 {
                    let paired_idx = i ^ (1 << dest_qubit);
                    register.state_vector.swap(i, paired_idx);
                }
            }
        }

        if m1 == 1 {
            // Pauli-Z korekcija (faza)
            let size = register.state_vector.len();
            for i in 0..size {
                if (i & (1 << dest_qubit)) != 0 {
                    register.state_vector[i] *= num_complex::Complex64::new(-1.0, 0.0);
                }
            }
        }

        Ok(())
    }
}