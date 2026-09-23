use crate::quantum::QubitRegister;
use num_complex::Complex64;
use rand::Rng;

pub enum NoiseModel {
    /// Nasumična rotacija faze/stanja (Greška pri operacijama)
    Depolarizing { probability: f64 },
    /// Relaksacija kubita u osnovno stanje |0> tokom vremena (T1 vreme)
    AmplitudeDamping { gamma: f64 },
}

pub struct EnvironmentalNoise {
    pub model: NoiseModel,
}

impl EnvironmentalNoise {
    pub fn new(model: NoiseModel) -> Self {
        Self { model }
    }

    /// Primenjuje šum na određeni kubit u registru
    pub fn apply(&self, register: &mut QubitRegister, target: usize) {
        let mut rng = rand::thread_rng();

        match &self.model {
            NoiseModel::Depolarizing { probability } => {
                // Ispravljeno: rng.gen::<f64>()
                if rng.gen_bool(*probability) {
                    let size = register.state_vector.len();
                    for i in 0..size {
                        if (i & (1 << target)) != 0 {
                            register.state_vector[i] *= Complex64::new(-1.0, 0.0);
                        }
                    }
                }
            }
            NoiseModel::AmplitudeDamping { gamma } => {
                let size = register.state_vector.len();
                let scale = (1.0 - gamma).sqrt();

                // Ispravljeno: for i in 0..size
                for i in 0..size {
                    if (i & (1 << target)) != 0 {
                        let paired_idx = i ^ (1 << target);
                        let energy_loss = register.state_vector[i] * Complex64::new(gamma.sqrt(), 0.0);

                        register.state_vector[i] *= Complex64::new(scale, 0.0);
                        register.state_vector[paired_idx] += energy_loss;
                    }
                }
            }
        }
    }
}