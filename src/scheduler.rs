use crate::quantum::QubitRegister;
use std::collections::HashMap;
use num_complex::Complex64;

pub struct QuantumProcess {
    pub pid: usize,
    pub allocated_qubits: Vec<usize>,
}

pub struct QuantumScheduler {
    pub hardware_register: QubitRegister,
    pub active_processes: HashMap<usize, QuantumProcess>,
    free_qubit_map: Vec<bool>,
}

#[derive(Clone, Debug)]
pub struct QuantumContextFrame {
    pub pid: usize,
    pub num_qubits: usize,
    /// Sparse reprezentacija: (Indeks stanja, Amplituda)
    pub sparse_state: Vec<(usize, Complex64)>,
}

impl QuantumScheduler {
    pub fn new(total_qubits: usize) -> Self {
        Self {
            hardware_register: QubitRegister::new(total_qubits),
            active_processes: HashMap::new(),
            free_qubit_map: vec![true; total_qubits],
        }
    }

    pub fn freeze_process(&mut self, pid: usize) -> Result<QuantumContextFrame, String> {
        let process = self.active_processes.get(&pid)
            .ok_or_else(|| format!("Proces PID {} not found", pid))?;

        let size = self.hardware_register.state_vector.len();
        let mut sparse_state = Vec::new();

        // Čuvamo samo amplitude čija je verovatnoća značajna (> 1e-9)
        for idx in 0..size {
            let amp = self.hardware_register.state_vector[idx];
            if amp.norm_sqr() > 1e-9 {
                sparse_state.push((idx, amp));
            }
        }

        let frame = QuantumContextFrame {
            pid,
            num_qubits: process.allocated_qubits.len(),
            sparse_state,
        };

        // Oslobađamo resurse registara u scheduluer-u
        for &qubit in &process.allocated_qubits {
            self.free_qubit_map[qubit] = true;
        }
        self.active_processes.remove(&pid);

        Ok(frame)
    }

    /// Restore (Vraćanje zamrznutog procesa nazad u izvršni registar)
    pub fn restore_process(&mut self, frame: QuantumContextFrame) -> Result<(), String> {
        let _allocated = self.allocate_qubits(frame.pid, frame.num_qubits)?;

        // Resetujemo odgovarajući deo vektora stanja i učitavamo iz frame-a
        for idx in 0..self.hardware_register.state_vector.len() {
            self.hardware_register.state_vector[idx] = Complex64::new(0.0, 0.0);
        }

        for (idx, amp) in frame.sparse_state {
            if idx < self.hardware_register.state_vector.len() {
                self.hardware_register.state_vector[idx] = amp;
            }
        }

        Ok(())
    }


    /// Alocira određeni broj kvantnih bitova za proces (PID)
    pub fn allocate_qubits(&mut self, pid: usize, count: usize) -> Result<Vec<usize>, String> {
        let mut allocated = Vec::new();

        for (idx, is_free) in self.free_qubit_map.iter_mut().enumerate() {
            if *is_free {
                *is_free = false;
                allocated.push(idx);
                if allocated.len() == count {
                    break;
                }
            }
        }

        if allocated.len() < count {
            return Err("Not enough free quantum resources!".to_string());
        }

        let process = QuantumProcess {
            pid,
            allocated_qubits: allocated.clone(),
        };

        self.active_processes.insert(pid, process);
        Ok(allocated)
    }
}