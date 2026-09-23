pub struct CryoThermalEngine {
    pub current_temp_mkelvin: f64,
    pub base_frequency_ghz: f64,
}

impl CryoThermalEngine {
    pub fn new() -> Self {
        Self {
            current_temp_mkelvin: 15.0, // Standardna radna temp (15 mK)
            base_frequency_ghz: 5.0,     // 5 GHz transmonska frekvencija
        }
    }

    /// Simulira mikrokolebanja temperature u frižideru i vraća korigovanu frekvenciju za mikrotalasne pulseve
    pub fn get_drift_corrected_frequency(&mut self, temp_delta_mkelvin: f64) -> f64 {
        self.current_temp_mkelvin += temp_delta_mkelvin;
        
        // Termički drift utiče na frekvenciju transmonskog prelaza (approx -0.002 GHz po mK)
        let drift = (self.current_temp_mkelvin - 15.0) * -0.002;
        let corrected_freq = self.base_frequency_ghz + drift;

        println!(" [CRYO ENGINE] Temp: {:.3} mK | Frequency Shifted to: {:.5} GHz", 
                 self.current_temp_mkelvin, corrected_freq);

        corrected_freq
    }
}