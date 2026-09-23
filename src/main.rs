mod quantum;
mod scheduler;
mod instruction;
mod noise;
mod vmm;
mod ecc;
mod ipc;
mod qir;
mod bench;
mod cli;
mod tensor;
mod transpiler;
mod pulse;
mod mitigation;
mod calibration;
mod vqe;
mod qutrit;
mod surface_code;
mod peps;
mod gpu_accel;
mod stabilizer;
mod dynamic_circuit;
mod resource_estimator;
mod security;
mod cryo_drift;
mod cluster;

use security::QuantumTenantManager;
use cryo_drift::CryoThermalEngine;
use cluster::ClusterOrchestrator;
use stabilizer::StabilizerTableau;
use dynamic_circuit::DynamicExecutor;
use resource_estimator::ResourceEstimator;
use crate::instruction::QuantumOp;
use crate::quantum::QubitRegister;
use qutrit::QutritState;
use surface_code::{SurfaceCode17, QubitCoord};
use peps::PEPSTensorGrid;
use gpu_accel::GPUComputeEngine;
use std::collections::HashMap;
use mitigation::ErrorMitigation;
use calibration::PulseCalibrator;
use vqe::VQEOptimizer;
use tensor::MatrixProductState;
use transpiler::{HardwareCouplingMap, QuantumTranspiler};
use pulse::TransmonHAL;
use ipc::QuantumIPCChannel;
use qir::QuantumIROptimizer;
use vmm::QuantumVMM;
use ecc::BitFlipCode;
use instruction::QasmExecutor;
use scheduler::QuantumScheduler;
use noise::{EnvironmentalNoise, NoiseModel};
use bench::KernelBenchmark;
use cli::KernelCLI;

fn main() {
    println!("=== Quantum OS Simulator Booting ===");

    // Inicijalizujemo registar sa 1 kubitom
    let mut reg = QubitRegister::new(1);
    println!("Initial state |0>: {:?}", reg.state_vector);

    // Stavljamo kubit u superpoziciju (Hadamard kapija)
    reg.apply_hadamard(0);
    println!("State after the Hadamard gate (superposition): {:?}", reg.state_vector);

    println!("=== Quantum OS Kernel v0.1.0 Initialized ===");

    // Inicijalizujemo OS sa 4 fizička kubita
    let mut os_scheduler = QuantumScheduler::new(4);

    // Proces 101 traži 2 kubita za spregnutost (Entanglement)
    let pid = 101;
    let qubits = os_scheduler.allocate_qubits(pid, 2).expect("Allocation failed");
    println!("Proces PID {} got hives: {:?}", pid, qubits);

    let q0 = qubits[0];
    let q1 = qubits[1];

    // Kreiranje Bell-ovog stanja: H(q0) -> CNOT(q0, q1)
    os_scheduler.hardware_register.apply_hadamard(q0);
    os_scheduler.hardware_register.apply_cnot(q0, q1);

    println!("Performed Bell state generation on qubits {}, {}", q0, q1);

    // Merenje obavljamo na oboje
    let res0 = os_scheduler.hardware_register.measure(q0);
    let res1 = os_scheduler.hardware_register.measure(q1);

    println!("Measurement result: Qubit {} = {}, Qubit {} = {}", q0, res0, q1, res1);
    println!("(Coupling guarantees that they must be equal: 0-0 ili 1-1)");

    println!("\n=== Executing QASM Program via Kernel ===");

    // Definisanje kvantnog programa u tekstualnom obliku
    let qasm_code = r#"
        h 0
        cnot 0 1
        measure 0
        measure 1
    "#;

    // Alociramo novi registar za novi proces (PID 102) sa 2 kubita
    let mut process_reg = quantum::QubitRegister::new(2);

    // Izvršavanje programa
    let results = QasmExecutor::execute_program(&mut process_reg, qasm_code);

    for (qubit, val) in results {
        println!("QASM Result -> Qubit {}: {}", qubit, val);
   }

   println!("\n=== Testing Context Switch & Environmental Noise ===");

    let pid_noise = 201;
    let allocated = os_scheduler.allocate_qubits(pid_noise, 2).unwrap();
    
    // Pravimo superpoziciju
    os_scheduler.hardware_register.apply_hadamard(allocated[0]);

    // 1. Primenjujemo depolarizujući šum na fizički kubit
    let noise_sim = EnvironmentalNoise::new(NoiseModel::Depolarizing { probability: 0.5 });
    noise_sim.apply(&mut os_scheduler.hardware_register, allocated[0]);
    println!("Noise applied to a qubit {}.", allocated[0]);

    // 2. Zamrzavanje procesa (Context Switch OUT)
    let context_snapshot = os_scheduler.freeze_process(pid_noise)
        .expect("Context Freeze Failed");
    println!("Process PID {} ​​frozen. Preserved {} of non-zero amplitudes.", pid_noise, context_snapshot.sparse_state.len());

    // 3. Vraćanje procesa (Context Switch IN)
    os_scheduler.restore_process(context_snapshot.clone()).expect("Context Restore Failed");
    println!("Proces PID {} successfully restored to Scheduler!", pid_noise);

    println!("\n=== Testing Virtual Memory Manager (Swap) & ECC ===");

    // 1. Inicijalizacija VMM-a u "qswap" direktorijumu
    let vmm = QuantumVMM::new("./qswap");

    // Uzimamo trenutni context snapshot iz prethodnog koraka i šaljemo ga na disk
    println!("We send process PID {} ​​to VMM Swap to disk...", pid_noise);
    vmm.swap_out(&context_snapshot).expect("Swap OUT unsuccessful");
    println!("Process PID {} ​​successfully written to disk at './qswap/process_{}.qswap'", pid_noise, pid_noise);

    // Vraćamo proces sa diska nazad u memoriju
    let restored_from_disk = vmm.swap_in(pid_noise).expect("Swap IN unsuccessful");
    println!("Process PID {} ​​successfully read from disk and deleted from Swap!", restored_from_disk.pid);

    // 2. Testiranje 3-Qubit Bit-Flip Kvantne Korekcije Grešaka
    println!("\n--- Testing Quantum Error Correction (3-Qubit Code) ---");
    let mut ecc_reg = quantum::QubitRegister::new(3); // 0: Data, 1: Ancilla1, 2: Ancilla2

    // Postavljamo podatak u superpoziciju
    ecc_reg.apply_hadamard(0);
    println!("Initialized logic qubit 0 into superposition.");

    // Enkodiramo stanje na 3 kubita
    BitFlipCode::encode(&mut ecc_reg, 0, 1, 2);
    println!("State encoded in 3 physical qubits (Redundancy active).");

    // Simuliramo grešku na okolini (Invertujemo Data Kubit)
    println!("We simulate a hardware error (Bit-Flip) on a qubit 0...");
    ecc_reg.apply_cnot(1, 0); // Namerno ubacivanje greške

    // Pokrećemo detekciju i korekciju
    let status = BitFlipCode::detect_and_correct(&mut ecc_reg, 0, 1, 2);
    println!("ECC Status: {}", status);

    println!("\n=== Testing Quantum IR Optimizer ===");

    // Unosimo neoptimizovani niz kapija koji sadrži redundantne operacije
    let unoptimized_program = vec![
        QuantumOp::Hadamard(0),
        QuantumOp::Hadamard(0), // Redundantno (H * H = I)
        QuantumOp::Hadamard(1),
        QuantumOp::CNOT { control: 0, target: 1 },
        QuantumOp::CNOT { control: 0, target: 1 }, // Redundantno (CNOT * CNOT = I)
    ];

    println!("Number of instructions before QIR optimization: {}", unoptimized_program.len());
    let optimized_program = QuantumIROptimizer::optimize(unoptimized_program);
    println!("Number of instructions AFTER QIR optimization: {}", optimized_program.len());
    println!("Remaining optimized instructions: {:?}", optimized_program);

    println!("\n--- Testing Quantum IPC (Teleportation) ---");
    let mut ipc_reg = quantum::QubitRegister::new(4); 
    // 0: Source (Process A), 1: EPR_A (Process A), 2: EPR_B (Process B), 3: Dest (Process B)

    // 1. Priprema EPR para između procesa A i B
    ipc_reg.apply_hadamard(1);
    ipc_reg.apply_cnot(1, 2);

    // 2. Postavljanje procesa A izvornog kubita u stanje superpozicije
    ipc_reg.apply_hadamard(0);
    println!("Process A prepared the data on the qubit 0.");

    // 3. Izvršavanje Teleportacije ka Procesu B
    QuantumIPCChannel::teleport(&mut ipc_reg, 0, 1, 2, 3)
        .expect("Teleportation failed");
    println!("Quantum state successfully transferred through IPC to Process B (Qubit 3)!");

     println!("=== QUANTUM OS KERNEL INITIALIZATION ===");

    // 1. Pokretanje benchmarkinga i stres-testa za matrike stanja (do 14 kubita)
    KernelBenchmark::run_stress_test(14);

    // 2. Inicijalizacija i pokretanje CLI interfejsa sa 8 fizičkih kubita
    let mut kernel_cli = KernelCLI::new(8);
    kernel_cli.start_shell();

    println!("\n==================================================");
    println!("  ENTERPRISE QUANTUM OS ENGINE INTEGRATION MODE  ");
    println!("==================================================");

    // 1. Matrix Product State (MPS) Tensor Network Test
    println!("\n1. Testing Matrix Product State (MPS) Tensor Compression...");
    let mut mps = MatrixProductState::new(50, 16); // Simulacija 50 KUBITA uz bond dimenziju 16!
    let hadamard_gate = [
        num_complex::Complex64::new(std::f64::consts::FRAC_1_SQRT_2, 0.0),
        num_complex::Complex64::new(std::f64::consts::FRAC_1_SQRT_2, 0.0),
        num_complex::Complex64::new(std::f64::consts::FRAC_1_SQRT_2, 0.0),
        num_complex::Complex64::new(-std::f64::consts::FRAC_1_SQRT_2, 0.0),
    ];

    mps.apply_local_gate(0, &hadamard_gate);
    mps.compress_bonds();
    println!("Successfully implemented a gate over a 50-qubit MPS chain without RAM explosion!");

    // 2. Hardware Topology Transpiler Test
    println!("\n2. Testing Grid Topology Transpiler...");
    let grid_chip = HardwareCouplingMap::new_grid(3, 3); // 3x3 Grid čip (9 kubita)
    let logical_program = vec![
        QuantumOp::CNOT { control: 0, target: 8 } // Kubit 0 i 8 NISU direktni susedi!
    ];

    let transpiled_program = QuantumTranspiler::transpile(logical_program, &grid_chip);
    println!("Original instruction: CNOT (0 -> 8)");
    println!("Number of physical instructions after transpiling (SWAP line): {}", transpiled_program.len());

    // 3. Pulse-Level HAL Driver Test
    println!("\n3. Testing Microwave Pulse Generation (HAL Layer)...");
    let hal = TransmonHAL::new(9);
    let pulse_stream = hal.compile_op_to_pulses(&QuantumOp::Hadamard(0));

    println!("Hadamard gate translated into {} microwave pulses:", pulse_stream.len());
    for p in pulse_stream {
        println!(" -> Channel: Qubit {} | Freq: {:.2} GHz | Duration: {} ns | Samples: {}", 
                 p.channel, p.frequency_ghz, p.duration_ns, p.waveform.len());
    }
    println!("==================================================\n");

    println!("\n==================================================");
    println!("  ABSOLUTE FULL-STACK QUANTUM OS COMPLETE ENGINE ");
    println!("==================================================");

    // 1. Error Mitigation (ZNE)
    println!("\n1. Testing Zero-Noise Extrapolation (ZNE)...");
    let exp_noisy = 0.65; // Očekivana vrednost pod normalnim šumom
    let exp_double_noise = 0.45; // Očekivana vrednost pod 3x većim šumom
    let mitigated = ErrorMitigation::extrapolate_zero_noise(exp_noisy, exp_double_noise);
    println!(" Noisy Expectation: {:.2}", exp_noisy);
    println!(" Scaled Expectation (3x Noise): {:.2}", exp_double_noise);
    println!(" -> Mitigated Zero-Noise Expectation: {:.2}", mitigated);

    // 2. Pulse Calibration
    println!("\n2. Testing Rabi Pulse Calibration Loop...");
    let _best_amp = PulseCalibrator::calibrate_rabi_pulse(&hal, 0);

    // 3. VQE Hybrid Loop
    println!("\n3. Running Hybrid Classical-Quantum VQE Loop...");
    let (optimal_theta, ground_energy) = VQEOptimizer::run_vqe_ground_state_search();
    println!(" -> VQE Search Complete! Optimal Theta: {:.4} rad | Min Energy: {:.4}", optimal_theta, ground_energy);
    println!("==================================================\n");

    println!("==================================================");
    println!("   ADVANCED INDUSTRIAL QUANTUM OS EXTENSIONS     ");
    println!("==================================================");

    // 1. Qutrit & DRAG Pulses
    println!("\n--- 1. Testing Qutrit Leakage & DRAG Pulses ---");
    let (i_wave, q_wave) = QutritState::generate_drag_pulse(100, 20.0, -0.3);
    let mut qutrit = QutritState::new();
    
    // Primena izračunatog DRAG pulsa
    for (&i, &q) in i_wave.iter().zip(q_wave.iter()) {
        qutrit.apply_qutrit_hamiltonian(i * 0.1, q * 0.1);
    }
    println!(" DRAG Pulse Generated (I/Q Channels: {} samples)", i_wave.len());
    println!(" Qutrit Leakage Probability |2>: {:.6}", qutrit.get_leakage_probability());

    // 2. Surface Code 17 & MWPM Decoder
    println!("\n--- 2. Surface Code 17 & MWPM Decoder ---");
    let sc17 = SurfaceCode17::new();
    let mut noisy_data = HashMap::new();
    
    // Simulacija greške na Data kubitu (2, 2)
    let error_coord = QubitCoord { x: 2, y: 2 };
    noisy_data.insert(error_coord, true);

    let triggered_syndromes = sc17.measure_syndromes(&noisy_data);
    println!(" Triggered Ancilla Syndromes: {:?}", triggered_syndromes);

    if let Some(corrected_qubit) = sc17.mwpm_decode(&triggered_syndromes) {
        println!(" MWPM Decoder successfully identified error qubit: {:?}", corrected_qubit);
        assert_eq!(corrected_qubit, error_coord);
    }

    // 3. 2D PEPS Tensor Network
    println!("\n--- 3. 2D PEPS Tensor Network Grid ---");
    let peps = PEPSTensorGrid::new(4, 4, 2); // 4x4 rešetka, bond dimension D=2
    let expectation = peps.contract_boundary_expectation();
    println!(" PEPS Grid initialized: 4x4 Nodes (Physical dim 2, Bond dim 2)");
    println!(" Contracted Boundary Expectation Value: {:.4}", expectation);

    // 4. GPU Compute Pipeline
    println!("\n--- 4. GPU WGSL Compute Pipeline ---");
    let gpu = GPUComputeEngine::init_virtual_gpu();
    gpu.dispatch_gate_execution(1024);
    println!(" Shader source preview:\n{}", GPUComputeEngine::get_hadamard_wgsl_shader());

    println!("==================================================");
    println!("   ALL ADVANCED INDUSTRIAL EXTENSIONS COMPLETE!   ");
    println!("==================================================");

    println!("\n==================================================");
    println!("   THE ABSOLUTE ULTIMATE QUANTUM OS ENGINE IS LIVE");
    println!("==================================================");

    // 1. Stabilizer Engine (Gottesman-Knill)
    println!("\n1. Running Gottesman-Knill Stabilizer Engine (1000 Qubits Scale)...");
    let mut tableau = StabilizerTableau::new(1000);
    tableau.apply_h(0);
    tableau.apply_cnot(0, 1);
    println!(" -> Successfully simulated Bell State across 1000 Qubits via Stabilizers!");

    // 2. Dynamic Mid-Circuit Control Flow
    println!("\n2. Testing Mid-Circuit Measurement & Feed-Forward Control...");
    let mut dynamic_reg = QubitRegister::new(2);
    // Stavljamo kubit 0 u superpoziciju
    let h_gate = [
        num_complex::Complex64::new(1.0 / 2.0f64.sqrt(), 0.0), num_complex::Complex64::new(1.0 / 2.0f64.sqrt(), 0.0),
        num_complex::Complex64::new(1.0 / 2.0f64.sqrt(), 0.0), num_complex::Complex64::new(-1.0 / 2.0f64.sqrt(), 0.0),
    ];
    dynamic_reg.apply_gate_1q(0, &h_gate);
    DynamicExecutor::execute_dynamic_branch(&mut dynamic_reg, 0, 1);

    // 3. Resource Estimator
    println!("\n3. Profiling Quantum Circuit Resource Requirements...");
    let sample_circuit = vec![
        QuantumOp::Hadamard (0),
        QuantumOp::CNOT { control: 0, target: 1 },
        QuantumOp::CNOT { control: 1, target: 2 },
    ];
    let report = ResourceEstimator::analyze(&sample_circuit, 20.0, 100.0, 0.001);
    println!(" -> Total Gates: {}", report.total_gates);
    println!(" -> CNOT Count: {}", report.cnot_count);
    println!(" -> Estimated Duration: {:.3} us", report.estimated_duration_us);
    println!(" -> Estimated Circuit Fidelity: {:.2}%", report.estimated_fidelity * 100.0);

    println!("\n==================================================");
    println!("   FINISHED! THERE ARE NO MORE HOLE IN THE SYSTEM! 🔥  ");
    println!("==================================================\n");

    println!("\n==================================================");
    println!("   ENTERPRISE CLOUD & DISTRIBUTED INFRASTRUCTURE  ");
    println!("==================================================");

    // 1. Multi-Tenant Security
    println!("\n1. Testing Enterprise Multi-Tenancy & Security...");
    let mut tenant_mgr = QuantumTenantManager::new();
    if tenant_mgr.authorize_and_isolate("Pharmaceutical_Corp", 50).is_ok() {
        println!(" -> Isolation Verified. Memory sandbox clear.");
    }

    // 2. Cryo-Thermal Drift Feedback
    println!("\n2. Testing Real-Time Cryo-Thermal Drift Compensation Loop...");
    let mut cryo = CryoThermalEngine::new();
    cryo.get_drift_corrected_frequency(0.35); // Simuliramo skok od +0.35 mK

    // 3. Distributed Cluster Orchestration
    println!("\n3. Planning Distributed State-Vector Allocation Across GPU Cluster...");
    let cluster = ClusterOrchestrator::new(4, 40); // 4 GPU čvora za 40 kubita
    cluster.plan_distributed_execution();

    println!("\n==================================================");
    println!("   ABSOLUTE 100% COMPLETE INDUSTRIAL OS MAP! 🔥  ");
    println!("==================================================\n");

}