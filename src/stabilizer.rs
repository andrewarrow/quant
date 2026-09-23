#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Pauli {
    I,
    X,
    Y,
    Z,
}

/// Tableau reprezentacija za Gottesman-Knill simulaciju miliona kubita
pub struct StabilizerTableau {
    pub num_qubits: usize,
    pub table: Vec<Vec<Pauli>>, // Matrixdim: 2N x N za Stabilizers + Destabilizers
    pub phases: Vec<bool>,       // Phases (+1 ili -1)
}

impl StabilizerTableau {
    pub fn new(num_qubits: usize) -> Self {
        let mut table = vec![vec![Pauli::I; num_qubits]; 2 * num_qubits];
        
        // Inicijalizacija |00...0> stanja gde su stabilizatori Z_i
        for i in 0..num_qubits {
            table[i + num_qubits][i] = Pauli::Z; // Stabilizatori
            table[i][i] = Pauli::X;             // Destabilizatori
        }

        Self {
            num_qubits,
            table,
            phases: vec![false; 2 * num_qubits],
        }
    }

    /// Hadamard kapija nad stabilizatorskim tableau-om: X -> Z, Z -> X
    pub fn apply_h(&mut self, qubit: usize) {
        for row in 0..(2 * self.num_qubits) {
            match self.table[row][qubit] {
                Pauli::X => self.table[row][qubit] = Pauli::Z,
                Pauli::Z => self.table[row][qubit] = Pauli::X,
                _ => {}
            }
        }
    }

    /// CNOT kapija nad stabilizatorskim tableau-om
    pub fn apply_cnot(&mut self, control: usize, target: usize) {
        for row in 0..(2 * self.num_qubits) {
            let p_ctrl = self.table[row][control].clone();
            let p_targ = self.table[row][target].clone();

            if p_ctrl == Pauli::X && p_targ == Pauli::I {
                self.table[row][target] = Pauli::X;
            } else if p_ctrl == Pauli::I && p_targ == Pauli::Z {
                self.table[row][control] = Pauli::Z;
            }
        }
    }
}