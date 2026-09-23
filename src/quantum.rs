use num_complex::Complex64;
use std::f64::consts::FRAC_1_SQRT_2;
use rayon::prelude::*;

/// Struktura koja predstavlja kvantni registar
pub struct QubitRegister {
    pub num_qubits: usize,
    pub state_vector: Vec<Complex64>,
}

impl QubitRegister {
    /// Inicijalizuje registar sa `n` kubita u stanju |00...0>
    pub fn new(num_qubits: usize) -> Self {
        let size = 1 << num_qubits; // 2^n stanja
        let mut state_vector = vec![Complex64::new(0.0, 0.0); size];
        state_vector[0] = Complex64::new(1.0, 0.0); // Početno stanje |0>

        Self {
            num_qubits,
            state_vector,
        }
    }

    pub fn apply_gate_1q(&mut self, target: usize, gate: &[num_complex::Complex64; 4]) {
        let step = 1 << target;
        for i in (0..self.state_vector.len()).step_by(2 * step) {
            for j in 0..step {
                let u = self.state_vector[i + j];
                let v = self.state_vector[i + j + step];

                self.state_vector[i + j] = gate[0] * u + gate[1] * v;
                self.state_vector[i + j + step] = gate[2] * u + gate[3] * v;
            }
        }
    }

    pub fn apply_hadamard_parallel(&mut self, target: usize) {
        let step = 1 << target;
        let scale = FRAC_1_SQRT_2;
        let chunk_size = 2 * step;

        // Paralelna obrada nezavisnih blokova vektora stanja
        self.state_vector
            .par_chunks_mut(chunk_size)
            .for_each(|chunk| {
                for j in 0..step {
                    let u = chunk[j];
                    let v = chunk[j + step];

                    chunk[j] = Complex64::new(scale, 0.0) * (u + v);
                    chunk[j + step] = Complex64::new(scale, 0.0) * (u - v);
                }
            });
    }

    /// Primena Hadamardove kapije (H-gate) za pravljenje superpozicije na target kubitu
    pub fn apply_hadamard(&mut self, target: usize) {
        let step = 1 << target;
        let scale = FRAC_1_SQRT_2;

        for i in (0..self.state_vector.len()).step_by(2 * step) {
            for j in 0..step {
                let u = self.state_vector[i + j];
                let v = self.state_vector[i + j + step];

                self.state_vector[i + j] = Complex64::new(scale, 0.0) * (u + v);
                self.state_vector[i + j + step] = Complex64::new(scale, 0.0) * (u - v);
            }
        }
    }
    pub fn apply_cnot(&mut self, control: usize, target: usize) {
        let size = self.state_vector.len();
        for i in 0..size {
            // Proveravamo da li je control bit postavljen na 1
            if (i & (1 << control)) != 0 {
                let paired_idx = i ^ (1 << target);
                if i < paired_idx {
                    self.state_vector.swap(i, paired_idx);
                }
            }
        }
    }

    /// Merenje određenog kubita: urušava talasnu funkciju u stanje 0 ili 1
    pub fn measure(&mut self, target: usize) -> u8 {
        let mut prob_zero = 0.0;
        let size = self.state_vector.len();

        // Izračunavanje verovatnoće da je kubit u stanju |0>
        for i in 0..size {
            if (i & (1 << target)) == 0 {
                prob_zero += self.state_vector[i].norm_sqr();
            }
        }

        let _rng = rand::thread_rng();
        let random_val: f64 = rand::random();
        let outcome = if random_val < prob_zero { 0 } else { 1 };

        // Urušavanje talasne funkcije (renormalizacija preostalih stanja)
        let norm_factor = if outcome == 0 { prob_zero.sqrt() } else { (1.0 - prob_zero).sqrt() };

        for i in 0..size {
            let bit_is_set = (i & (1 << target)) != 0;
            if (outcome == 0 && bit_is_set) || (outcome == 1 && !bit_is_set) {
                self.state_vector[i] = Complex64::new(0.0, 0.0);
            } else {
                self.state_vector[i] /= norm_factor;
            }
        }

        outcome
    }
}