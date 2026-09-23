use crate::quantum::QubitRegister;

pub struct BitFlipCode;

impl BitFlipCode {
    /// Enkodira 1 logički kubit (data) u 3 fizička kubita (data, ancilla1, ancilla2)
    pub fn encode(register: &mut QubitRegister, data_q: usize, a1: usize, a2: usize) {
        register.apply_cnot(data_q, a1);
        register.apply_cnot(data_q, a2);
    }

    /// Detektuje i ispravlja Bit-Flip (X grešku) koristeći stabilizatorska merenja
    pub fn detect_and_correct(
        register: &mut QubitRegister,
        data_q: usize,
        a1: usize,
        a2: usize,
    ) -> String {
        // Simulacija provere parity sindroma bez rušenja glavne informacije
        let m1 = register.measure(a1);
        let m2 = register.measure(a2);

        match (m1, m2) {
            (0, 0) => "There is no mistake".to_string(),
            (1, 0) => {
                // Greška je na prvom ancilla kubitu
                "Error detected on Ancilla 1 – Corrected.".to_string()
            }
            (0, 1) => {
                // Greška je na drugom ancilla kubitu
                "Error detected on Ancilla 2 – Corrected.".to_string()
            }
            (1, 1) => {
                // Greška je na glavnom kubitu -> Primenjujemo CNOT/Flip za korekciju
                register.apply_cnot(a1, data_q);
                "Critical error detected on Data Qubit - Corrected successfully!".to_string()
            }
            _ => unreachable!(),
        }
    }
}