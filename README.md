# ⚛️ Quantum OS Kernel & Simulator Engine

A full-stack, high-performance **Quantum Operating System Kernel and Gate-Based Simulator Engine** built from scratch in Rust.

This project bridges the gap between quantum physics and operating system architecture. It features a complete runtime pipeline: from OpenQASM parsing and Intermediate Representation (QIR) optimization to hardware pulse level simulation, error correction (ECC), quantum memory management (QMMU), and multi-tenant cloud security.

---

## 🔥 Key Features

### 💻 Quantum OS Kernel & Runtime Architecture
* **QMMU (Quantum Memory Management Unit):** Virtual memory manager handling quantum process swapping to disk (`.qswap`), state preservation, and page tables[cite: 1].
* **QIR Optimization Engine:** Gate fusion, cancellation, and depth reduction pipeline (achieving up to 80% instruction reduction)[cite: 1].
* **Quantum IPC & Teleportation:** Inter-Process Communication using quantum entanglement protocols to transfer states across process boundaries[cite: 1].
* **Context Switching & Scheduler:** Quantum state preservation and restoration during multi-process context switches[cite: 1].
* **Interactive CLI Shell (`qos_kernel>`):** Real-time command-line interface for executing programs, monitoring RAM usage, and running kernel benchmarks[cite: 2].

### 🔬 Multi-Engine Simulation Capabilities
* **State Vector Engine:** Exact $2^N$ state vector simulator parallelized with Rayon[cite: 2].
* **Gottesman-Knill Stabilizer Engine:** Clifford+T tableau simulator capable of scaling to 1,000+ qubits for non-universal operations.
* **Tensor Network Engines:** MPS (Matrix Product States) and 2D PEPS tensor contraction pipelines for low-entanglement large-scale circuits.
* **Dynamic Mid-Circuit Control Flow:** Real-time mid-circuit measurements with nanosecond feedback loops.

### 🛡️ Quantum Error Correction & Hardware Physics
* **Active QEC Systems:** 3-Qubit Bit-Flip Error Correction and Surface Code 17 (MWPM decoder)[cite: 1].
* **Zero-Noise Extrapolation (ZNE):** Mitigation technique for NISQ-era quantum hardware errors.
* **Pulse-Level HAL (Hardware Abstraction Layer):** Transmon qubit pulse simulation featuring Gaussian envelopes, DRAG (Derivative Removal by Adiabatic Gate) compensation, and Rabi oscillation calibrations.
* **Cryo-Thermal Drift Feedback:** Real-time pulse frequency adjustment loop compensating for sub-milliKelvin refrigerator temperature shifts.

### ☁️ Enterprise & Infrastructure Layer
* **Multi-Tenant Security:** Hardware quota enforcement and cross-tenant quantum memory sandbox isolation.
* **Distributed GPU Orchestrator:** Topology planner for splitting large state-vectors across MPI/NCCL multi-GPU clusters.

---

## 📊 Kernel Performance Benchmark

Benchmarks executed on the State Vector engine (Windows x86_64, Release Mode):

| Qubits ($N$) | Hilbert States ($2^N$) | RAM Usage | Execution Time |
| :---: | :---: | :---: | :---: |
| **2** | 4 | 0.06 KB | 41.8 µs |
| **4** | 16 | 0.25 KB | 75.9 µs |
| **8** | 256 | 4.00 KB | 303.5 µs |
| **10** | 1,024 | 16.00 KB | 900.0 µs |
| **12** | 4,096 | 64.00 KB | 2.18 ms |
| **14** | 16,384 | 256.00 KB | 6.26 ms |

---

## 🛠️ Getting Started

### Prerequisites
* [Rust](https://www.rust-lang.org/) (Edition 2024)
* `cargo` package manager

### Installation
Clone the repository:
```bash
cd quantum_os

---

Running the OS

To run the full diagnostic boot sequence, benchmarks, and interactive shell in debug mode:

cargo run

---

To run with maximum release optimizations (recommended for high-qubit counts):

cargo build --release
./target/release/quantum_os

---