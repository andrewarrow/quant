use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QubitCoord {
    pub x: usize,
    pub y: usize,
}

pub struct SurfaceCode17 {
    pub data_qubits: Vec<QubitCoord>,
    pub ancilla_x: Vec<QubitCoord>, // Stabilizatori za Bit-Flip (X)
    pub ancilla_z: Vec<QubitCoord>, // Stabilizatori za Phase-Flip (Z)
}

impl SurfaceCode17 {
    pub fn new() -> Self {
        // Inicijalizacija 3x3 rešetke Data kubita
        let mut data = Vec::new();
        for x in (0..5).step_by(2) {
            for y in (0..5).step_by(2) {
                data.push(QubitCoord { x, y });
            }
        }

        // Ancilla kubiti postavljeni između Data kubita
        let ancilla_x = vec![
            QubitCoord { x: 1, y: 0 }, QubitCoord { x: 3, y: 2 },
            QubitCoord { x: 1, y: 4 }, QubitCoord { x: 3, y: 4 },
        ];
        let ancilla_z = vec![
            QubitCoord { x: 0, y: 1 }, QubitCoord { x: 2, y: 1 },
            QubitCoord { x: 2, y: 3 }, QubitCoord { x: 4, y: 3 },
        ];

        Self {
            data_qubits: data,
            ancilla_x,
            ancilla_z,
        }
    }

    /// Ekstrakcija sindroma grešaka sa Ancilla kubita
    pub fn measure_syndromes(&self, noisy_data: &HashMap<QubitCoord, bool>) -> Vec<QubitCoord> {
        let mut triggered_syndromes = Vec::new();

        for ancilla in self.ancilla_x.iter().chain(self.ancilla_z.iter()) {
            // Meri parnost (parity) susednih Data kubita sa kojima je Ancilla povezana
            let mut parity = false;
            for data in &self.data_qubits {
                // Susedstvo po Menhetn distanci d = 1
                if (data.x as isize - ancilla.x as isize).abs() + (data.y as isize - ancilla.y as isize).abs() == 1 {
                    if *noisy_data.get(data).unwrap_or(&false) {
                        parity = !parity;
                    }
                }
            }

            if parity {
                triggered_syndromes.push(*ancilla);
            }
        }

        triggered_syndromes
    }

    /// Simplified MWPM (Minimum Weight Perfect Matching) Dekoder
    /// Pronalazi Data kubit koji najverovatnije uzrokuje izmerene sindrome
    pub fn mwpm_decode(&self, syndromes: &[QubitCoord]) -> Option<QubitCoord> {
        if syndromes.is_empty() {
            return None;
        }

        // Pronalaženje Data kubita sa najkraćom distancijom do okidačkih sindroma
        let mut best_candidate = None;
        let mut min_total_dist = usize::MAX;

        for data in &self.data_qubits {
            let mut total_dist = 0;
            for syn in syndromes {
                let dist = (data.x as isize - syn.x as isize).unsigned_abs() 
                         + (data.y as isize - syn.y as isize).unsigned_abs();
                total_dist += dist;
            }

            if total_dist < min_total_dist {
                min_total_dist = total_dist;
                best_candidate = Some(*data);
            }
        }

        best_candidate
    }
}