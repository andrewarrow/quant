use num_complex::Complex64;

/// Stanje Qutrit-a predstavlja vektor amplitude [alpha_|0>, beta_|1>, gamma_|2>]
#[derive(Debug, Clone)]
pub struct QutritState {
    pub state: [Complex64; 3],
}

impl QutritState {
    pub fn new() -> Self {
        Self {
            state: [
                Complex64::new(1.0, 0.0), // |0>
                Complex64::new(0.0, 0.0), // |1>
                Complex64::new(0.0, 0.0), // |2> (Leakage state)
            ],
        }
    }

    /// Generiše DRAG (Derivative Removal by Adiabatic Gate) puls koji sprečava curenje u |2>
    /// Vraća (In-Phase I(t), Quadrature Q(t)) komponente pulsa
    pub fn generate_drag_pulse(time_steps: usize, duration: f64, alpha_anharmonicity: f64) -> (Vec<f64>, Vec<f64>) {
        let mut i_wave = Vec::with_capacity(time_steps);
        let mut q_wave = Vec::with_capacity(time_steps);
        let dt = duration / time_steps as f64;
        let sigma = duration / 4.0;

        for step in 0..time_steps {
            let t = step as f64 * dt;
            let center = duration / 2.0;
            
            // Osnovna Gausova kriva I(t)
            let gaussian = (-((t - center).powi(2)) / (2.0 * sigma.powi(2))).exp();
            
            // Derivat Gausove krive dI/dt za Q(t) kompenzaciju curenja
            let d_gaussian = -((t - center) / (sigma.powi(2))) * gaussian;

            // DRAG korekcija: Quadrature komponenta zavisi od anharmoničnosti transmona
            let drag_correction = -d_gaussian / alpha_anharmonicity;

            i_wave.push(gaussian);
            q_wave.push(drag_correction);
        }

        (i_wave, q_wave)
    }

    /// Primena 3x3 Hamiltonijana na qutrit sa curenjem
    pub fn apply_qutrit_hamiltonian(&mut self, i_amp: f64, q_amp: f64) {
        // Modelovanje 3x3 matrice sa anharmoničnošću na nivou |2>
        let leakage_factor = 0.02; // Probabilističko curenje pod običnim pulsom
        let norm_sqr = self.state[0].norm_sqr() + self.state[1].norm_sqr() + self.state[2].norm_sqr();
        
        if norm_sqr > 0.0 {
            // Simulacija prelaza ka leakage nivou |2>
            self.state[2] += Complex64::new(q_amp * leakage_factor, 0.0);
            self.state[1] += Complex64::new(i_amp * (1.0 - leakage_factor), 0.0);
            
            // Renormalizacija stanja
            let new_norm = (self.state[0].norm_sqr() + self.state[1].norm_sqr() + self.state[2].norm_sqr()).sqrt();
            self.state[0] /= new_norm;
            self.state[1] /= new_norm;
            self.state[2] /= new_norm;
        }
    }

    pub fn get_leakage_probability(&self) -> f64 {
        self.state[2].norm_sqr()
    }
}