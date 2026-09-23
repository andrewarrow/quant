use num_complex::Complex64;

/// Tenzorski čvor koji predstavlja lokalno stanje jednog kubita u MPS lancu
#[derive(Clone, Debug)]
pub struct TensorNode {
    pub qubit_id: usize,
    /// Fizicke i virtuelne dimenzije tenzora: [left_bond, physical_dim (2), right_bond]
    pub data: Vec<Complex64>,
    pub bond_dimension: usize,
}

pub struct MatrixProductState {
    pub num_qubits: usize,
    pub nodes: Vec<TensorNode>,
    pub max_bond_dimension: usize, // Ograničenje spregnutosti radi kompresije
}

impl MatrixProductState {
    /// Inicijalizuje MPS u stanju |00...0> sa minimalnom bond dimenzijom (1)
    pub fn new(num_qubits: usize, max_bond: usize) -> Self {
        let mut nodes = Vec::with_capacity(num_qubits);

        for i in 0..num_qubits {
            // Stanje |0> predstavljeno kao tenzor [1, 2, 1]
            let data = vec![Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0)];
            nodes.push(TensorNode {
                qubit_id: i,
                data,
                bond_dimension: 1,
            });
        }

        Self {
            num_qubits,
            nodes,
            max_bond_dimension: max_bond,
        }
    }

    /// Primena 1-qubit kapije lokalno na MPS čvor (bez eksponencijalnog rasta)
    pub fn apply_local_gate(&mut self, target: usize, gate: &[Complex64; 4]) {
        if target >= self.num_qubits { return; }

        let node = &mut self.nodes[target];
        let mut new_data = vec![Complex64::new(0.0, 0.0); node.data.len()];

        // Množenje lokalnog tenzora sa 2x2 matricom kapije
        for b in 0..node.bond_dimension {
            let idx_0 = b * 2;
            let idx_1 = b * 2 + 1;

            let a0 = node.data[idx_0];
            let a1 = node.data[idx_1];

            new_data[idx_0] = gate[0] * a0 + gate[1] * a1;
            new_data[idx_1] = gate[2] * a0 + gate[3] * a1;
        }

        node.data = new_data;
    }

    /// Truncation (Aproksimativno odsecanje malih singularnih vrednosti za uštedu RAM-a)
    pub fn compress_bonds(&mut self) {
        for node in self.nodes.iter_mut() {
            if node.bond_dimension > self.max_bond_dimension {
                node.bond_dimension = self.max_bond_dimension;
                node.data.truncate(node.bond_dimension * 2);
            }
        }
    }
}