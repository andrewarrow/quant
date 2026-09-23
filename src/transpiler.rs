use crate::instruction::QuantumOp;
use std::collections::{HashMap, HashSet, VecDeque};

pub struct HardwareCouplingMap {
    /// Fizičke veze između kubita na čipu (susedni kubiti)
    pub connections: HashMap<usize, Vec<usize>>,
}

impl HardwareCouplingMap {
    /// Pravi linearni ili grid raspored kubita
    pub fn new_grid(rows: usize, cols: usize) -> Self {
        let mut connections = HashMap::new();
        for r in 0..rows {
            for c in 0..cols {
                let id = r * cols + c;
                let mut neighbors = Vec::new();

                if r > 0 { neighbors.push((r - 1) * cols + c); }
                if r + 1 < rows { neighbors.push((r + 1) * cols + c); }
                if c > 0 { neighbors.push(r * cols + (c - 1)); }
                if c + 1 < cols { neighbors.push(r * cols + (c + 1)); }

                connections.insert(id, neighbors);
            }
        }
        Self { connections }
    }

    /// Pronalazi najkraći put između dva kubita na čipu (BFS algoritam)
    pub fn shortest_path(&self, start: usize, end: usize) -> Vec<usize> {
        if start == end { return vec![start]; }
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut parent = HashMap::new();

        queue.push_back(start);
        visited.insert(start);

        while let Some(curr) = queue.pop_front() {
            if curr == end { break; }
            if let Some(neighbors) = self.connections.get(&curr) {
                for &next in neighbors {
                    if !visited.contains(&next) {
                        visited.insert(next);
                        parent.insert(next, curr);
                        queue.push_back(next);
                    }
                }
            }
        }

        let mut path = Vec::new();
        let mut curr = end;
        while let Some(&p) = parent.get(&curr) {
            path.push(curr);
            curr = p;
        }
        path.push(start);
        path.reverse();
        path
    }
}

pub struct QuantumTranspiler;

impl QuantumTranspiler {
    /// Ubacuje SWAP kapije ako CNOT operacija pokušava da poveže nefizički susedne kubite
    pub fn transpile(program: Vec<QuantumOp>, map: &HardwareCouplingMap) -> Vec<QuantumOp> {
        let mut physical_ops = Vec::new();

        for op in program {
            match op {
                QuantumOp::CNOT { control, target } => {
                    let path = map.shortest_path(control, target);
                    if path.len() <= 2 {
                        // Kubiti su direktni susedi na čipu
                        physical_ops.push(QuantumOp::CNOT { control, target });
                    } else {
                        // Ubacujemo SWAP lanac da dovedemo stanje do target kubita
                        for i in 0..(path.len() - 2) {
                            let q1 = path[i];
                            let q2 = path[i + 1];
                            // SWAP kapija se sastoji od 3 naizmenična CNOT-a
                            physical_ops.push(QuantumOp::CNOT { control: q1, target: q2 });
                            physical_ops.push(QuantumOp::CNOT { control: q2, target: q1 });
                            physical_ops.push(QuantumOp::CNOT { control: q1, target: q2 });
                        }

                        let last_control = path[path.len() - 2];
                        let last_target = path[path.len() - 1];
                        physical_ops.push(QuantumOp::CNOT {
                            control: last_control,
                            target: last_target,
                        });
                    }
                }
                other => physical_ops.push(other),
            }
        }

        physical_ops
    }
}