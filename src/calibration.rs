use crate::pulse::TransmonHAL;
use std::f64::consts::PI;

pub struct PulseCalibrator;

impl PulseCalibrator {
    /// Rabi Oscillation Experiment:
    /// Varia amplitude mikrotalasa od 0.0 do 1.0 da bi pronašao tačnu amplitudu za Pi-puls (X-Gate)
    pub fn calibrate_rabi_pulse(hal: &TransmonHAL, qubit: usize) -> f64 {
        let mut best_amplitude = 0.5;
        let mut max_flip_prob = 0.0;

        // Simulacija skeniranja napona na AWG drajveru
        for step in 1..=20 {
            let amp = step as f64 * 0.05;
            let _pulse = hal.generate_gaussian_pulse(qubit, amp, 40.0, 0.0);
            
            // Fizički odziv transmona: Verovatnoća prelaza |0> -> |1> prati sin^2(amp)
            let flip_probability = (amp * PI).sin().powi(2);

            if flip_probability > max_flip_prob {
                max_flip_prob = flip_probability;
                best_amplitude = amp;
            }
        }

        println!(" [CALIBRATION] Qubit {} Rabi Experiment Complete: Ideal Pi-Pulse Amplitude = {:.3}", qubit, best_amplitude);
        best_amplitude
    }
}