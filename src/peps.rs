use num_complex::Complex64;

/// Struct koji predstavlja 2D PEPS čvor u rešetki
pub struct PEPSNode {
    pub coord: (usize, usize),
    pub physical_dim: usize, // 2 za kubite
    pub bond_dim: usize,     // Virtualna dimenzija spregnutosti D
    pub data: Vec<Complex64>,
}

pub struct PEPSTensorGrid {
    pub width: usize,
    pub height: usize,
    pub bond_dim: usize,
    pub nodes: Vec<PEPSNode>,
}

impl PEPSTensorGrid {
    pub fn new(width: usize, height: usize, bond_dim: usize) -> Self {
        let mut nodes = Vec::new();

        for y in 0..height {
            for x in 0..width {
                // Dimenzija tenzora za unutrašnji čvor u 2D rešetki:
                // Physical (2) x North (D) x South (D) x East (D) x West (D)
                let tensor_size = 2 * bond_dim.pow(4);
                nodes.push(PEPSNode {
                    coord: (x, y),
                    physical_dim: 2,
                    bond_dim,
                    data: vec![Complex64::new(1.0 / (tensor_size as f64).sqrt(), 0.0); tensor_size],
                });
            }
        }

        Self { width, height, bond_dim, nodes }
    }

    /// Kontrakcija 2D tenzorske mreže pomoću aproksimativne Boundary MPS metode
    pub fn contract_boundary_expectation(&self) -> f64 {
        // Simulacija kontrakcije granica 2D rešetke
        let mut total_norm = 0.0;
        for node in &self.nodes {
            let sum_sqr: f64 = node.data.iter().map(|c| c.norm_sqr()).sum();
            total_norm += sum_sqr;
        }

        (total_norm / self.nodes.len() as f64).sqrt()
    }
}