use std::f64::consts::PI;

/// Gausov mikrotalasni puls koji se šalje u kriogeni frižider do čipa
#[derive(Debug, Clone)]
pub struct MicrowavePulse {
    pub channel: usize,      // Fizički kanal za dati kubit
    pub frequency_ghz: f64,  // Frekvencija transmona (npr. 5.12 GHz)
    pub amplitude: f64,      // Napon/Snaga pulsa
    pub duration_ns: f64,    // Trajanje pulsa u nanosekundama
    pub phase_rad: f64,      // Faza mikrotalasa
    pub waveform: Vec<f64>,  // Vremenski uzorci oblika talasa (Envelope)
}

pub struct TransmonHAL {
    pub base_frequencies: Vec<f64>,
}

impl TransmonHAL {
    pub fn new(num_qubits: usize) -> Self {
        // Svaki transmonski kubit ima svoju specifičnu frekvenciju (5.0 - 5.5 GHz)
        let freqs = (0..num_qubits)
            .map(|i| 5.0 + (i as f64 * 0.05))
            .collect();
        Self { base_frequencies: freqs }
    }

    /// Generiše Gausov puls koji realizuje rotaciju stanja za ugao theta (npr. X/2 ili Hadamard)
    pub fn generate_gaussian_pulse(
        &self,
        qubit: usize,
        amplitude: f64,
        duration_ns: f64,
        phase: f64,
    ) -> MicrowavePulse {
        let freq = self.base_frequencies.get(qubit).cloned().unwrap_or(5.0);
        let sample_rate_ghz = 2.0; // 2 Giga-sample po sekundi AWG-a
        let num_samples = (duration_ns * sample_rate_ghz) as usize;

        let mut waveform = Vec::with_capacity(num_samples);
        let sigma = duration_ns / 4.0;
        let center = duration_ns / 2.0;

        for s in 0..num_samples {
            let t = s as f64 / sample_rate_ghz;
            // Gausova kriva: A * exp(-(t - t0)^2 / (2 * sigma^2))
            let val = amplitude * (-((t - center).powi(2)) / (2.0 * sigma.powi(2))).exp();
            waveform.push(val);
        }

        MicrowavePulse {
            channel: qubit,
            frequency_ghz: freq,
            amplitude,
            duration_ns,
            phase_rad: phase,
            waveform,
        }
    }

    /// Prevođenje visokonivojske instrukcije u niz mikrotalasnih pulseva na drajverskom nivou
    pub fn compile_op_to_pulses(&self, op: &crate::instruction::QuantumOp) -> Vec<MicrowavePulse> {
        let mut pulses = Vec::new();

        match op {
            crate::instruction::QuantumOp::Hadamard(target) => {
                // Hadamard se na hardveru realizuje kombinacijom Y/2 i X pulseva
                pulses.push(self.generate_gaussian_pulse(*target, 0.5, 20.0, PI / 2.0));
                pulses.push(self.generate_gaussian_pulse(*target, 1.0, 40.0, 0.0));
            }
            crate::instruction::QuantumOp::CNOT { control, target: _ } => {
                // CNOT koristi Cross-Resonance (CR) mikrotalasni impuls izmedju dva kubita
                pulses.push(self.generate_gaussian_pulse(*control, 0.8, 80.0, 0.0));
            }
            _ => {}
        }

        pulses
    }
}