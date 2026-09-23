use crate::scheduler::QuantumContextFrame;
use std::fs::File;
use std::io::{Read, Write, BufReader, BufWriter};
use std::path::PathBuf;

pub struct QuantumVMM {
    swap_directory: PathBuf,
}

impl QuantumVMM {
    pub fn new<P: Into<PathBuf>>(swap_dir: P) -> Self {
        let path = swap_dir.into();
        if !path.exists() {
            std::fs::create_dir_all(&path).expect("Failed to create Swap directory");
        }
        Self { swap_directory: path }
    }

    fn get_swap_path(&self, pid: usize) -> PathBuf {
        self.swap_directory.join(format!("process_{}.qswap", pid))
    }

    /// Swap OUT: Upisuje zamrznuti kontekst na disk
    pub fn swap_out(&self, frame: &QuantumContextFrame) -> Result<(), String> {
        let path = self.get_swap_path(frame.pid);
        let file = File::create(&path)
            .map_err(|e| format!("Error creating swap file: {}", e))?;
        let mut writer = BufWriter::new(file);

        // Upisujemo PID i broj kubita
        writer.write_all(&frame.pid.to_le_bytes()).unwrap();
        writer.write_all(&frame.num_qubits.to_le_bytes()).unwrap();
        
        // Upisujemo broj nenultih stanja
        let len = frame.sparse_state.len();
        writer.write_all(&len.to_le_bytes()).unwrap();

        // Serijalizacija amplituda (index, re, im)
        for (idx, amp) in &frame.sparse_state {
            writer.write_all(&idx.to_le_bytes()).unwrap();
            writer.write_all(&amp.re.to_le_bytes()).unwrap();
            writer.write_all(&amp.im.to_le_bytes()).unwrap();
        }

        writer.flush().map_err(|e| format!("Swap write error: {}", e))?;
        Ok(())
    }

    /// Swap IN: Učitava kontekst sa diska i briše swap fajl
    pub fn swap_in(&self, pid: usize) -> Result<QuantumContextFrame, String> {
    let path = self.get_swap_path(pid);
    if !path.exists() {
        return Err(format!("Swap file for PID {} does not exist", pid));
    }

    let file = File::open(&path)
        .map_err(|e| format!("Error opening swap file: {}", e))?;
    let mut reader = BufReader::new(file);

    // Dodeljujemo 'mut' baferima
    let mut buf_usize = [0u8; std::mem::size_of::<usize>()];
    let mut buf_f64 = [0u8; 8];

    // Koristimo '&mut' umesto '&' pri pozivu read_exact
    reader.read_exact(&mut buf_usize).unwrap();
    let loaded_pid = usize::from_le_bytes(buf_usize);

    reader.read_exact(&mut buf_usize).unwrap();
    let num_qubits = usize::from_le_bytes(buf_usize);

    reader.read_exact(&mut buf_usize).unwrap();
    let len = usize::from_le_bytes(buf_usize);

    let mut sparse_state = Vec::with_capacity(len);

    for _ in 0..len {
        reader.read_exact(&mut buf_usize).unwrap();
        let idx = usize::from_le_bytes(buf_usize);

        reader.read_exact(&mut buf_f64).unwrap();
        let re = f64::from_le_bytes(buf_f64);

        reader.read_exact(&mut buf_f64).unwrap();
        let im = f64::from_le_bytes(buf_f64);

        sparse_state.push((idx, num_complex::Complex64::new(re, im)));
    }

    std::fs::remove_file(path).ok();

    Ok(QuantumContextFrame {
        pid: loaded_pid,
        num_qubits,
        sparse_state,
    })
  }
}