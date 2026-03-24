use num_complex::Complex64;
use super::state::QuantumState;

pub fn fidelity(a: &QuantumState, b: &QuantumState) -> f64 {
    assert_eq!(a.amplitudes.len(), b.amplitudes.len(), "state dimension mismatch");

    let inner: Complex64 = a.amplitudes
        .iter()
        .zip(b.amplitudes.iter())
        .map(|(x, y)| x.conj() * y)
        .sum();

    inner.norm_sqr()
}

pub fn trace_distance(a: &QuantumState, b: &QuantumState) -> f64 {
    assert_eq!(a.amplitudes.len(), b.amplitudes.len(), "state dimension mismatch");

    let sum: f64 = a.amplitudes
        .iter()
        .zip(b.amplitudes.iter())
        .map(|(x, y)| (x - y).norm_sqr())
        .sum();

    0.5 * sum.sqrt()
}

pub struct DensityMatrix {
    pub dim: usize,
    pub data: Vec<Vec<Complex64>>,
}

impl DensityMatrix {
    pub fn from_state(state: &QuantumState) -> Self {
        let dim = state.amplitudes.len();
        let mut data = vec![vec![Complex64::new(0.0, 0.0); dim]; dim];

        for i in 0..dim {
            for j in 0..dim {
                data[i][j] = state.amplitudes[i] * state.amplitudes[j].conj();
            }
        }

        Self { dim, data }
    }

    pub fn trace(&self) -> f64 {
        (0..self.dim).map(|i| self.data[i][i].re).sum()
    }

    pub fn purity(&self) -> f64 {
        let mut sum = 0.0;
        for i in 0..self.dim {
            for j in 0..self.dim {
                sum += (self.data[i][j] * self.data[j][i]).re;
            }
        }
        sum
    }

    pub fn print(&self) {
        for row in &self.data {
            let formatted: Vec<String> = row
                .iter()
                .map(|c| format!("{:+.3}{:+.3}i", c.re, c.im))
                .collect();
            println!("[{}]", formatted.join("  "));
        }
    }
}
