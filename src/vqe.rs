use crate::quantum::QubitRegister;
use num_complex::Complex64;

pub struct VQEOptimizer;

impl VQEOptimizer {
    /// VQE Ansatz: Pravi parametrizovano kvantno stanje |psi(theta)>
    pub fn build_ansatz(reg: &mut QubitRegister, theta: f64) {
        // Primena parametrizovane rotacije Ry(theta) nad kubitom 0
        let cos = (theta / 2.0).cos();
        let sin = (theta / 2.0).sin();
        let ry_gate = [
            Complex64::new(cos, 0.0),
            Complex64::new(-sin, 0.0),
            Complex64::new(sin, 0.0),
            Complex64::new(cos, 0.0),
        ];

        reg.apply_gate_1q(0, &ry_gate);
    }

    /// Izračunava očekivanu vrednost Hamiltonian operatora <psi|H|psi>
    pub fn measure_energy(reg: &QubitRegister) -> f64 {
        // Primer jednostavnog Hamiltonijana H = Z_0 (energija stanja |1> je +1, stanja |0> je -1)
        let prob_0 = reg.state_vector[0].norm_sqr();
        let prob_1 = reg.state_vector[1].norm_sqr();
        
        // Očekivana energija E = <Z>
        -1.0 * prob_0 + 1.0 * prob_1
    }

    /// Hibridna optimizaciona petlja (Gradient Descent u Rustu)
    pub fn run_vqe_ground_state_search() -> (f64, f64) {
        let mut theta = 0.1; // Početni ugao
        let learning_rate = 0.2;
        let mut min_energy = 1.0;

        for _step in 0..15 {
            let mut reg = QubitRegister::new(1);
            Self::build_ansatz(&mut reg, theta);
            let energy = Self::measure_energy(&reg);

            if energy < min_energy {
                min_energy = energy;
            }

            // Izračunavanje numeričkog gradijenta dE/dTheta
            let shift = 0.01;
            let mut reg_plus = QubitRegister::new(1);
            Self::build_ansatz(&mut reg_plus, theta + shift);
            let energy_plus = Self::measure_energy(&reg_plus);
            let grad = (energy_plus - energy) / shift;

            // Gradient descent korak
            theta -= learning_rate * grad;
        }

        (theta, min_energy)
    }
}