use num_complex::Complex64;

pub struct QuantumState {
    pub qubits: usize,
    pub amplitudes: Vec<Complex64>
}

impl QuantumState {
    pub fn new(qubits: usize) -> Self {
        let size = 1usize << qubits;

        let mut amplitudes = vec![Complex64::new(0.0, 0.0); size];

        // |000...0> initial state
        amplitudes[0] = Complex64::new(1.0, 0.0);

        Self { qubits, amplitudes }
    }
    pub fn dimension(&self) -> usize {
        self.amplitudes.len()
    }

    pub fn memory_bytes(&self) -> usize {
        self.amplitudes.len() * std::mem::size_of::<Complex64>()
    }

    pub fn print_state(&self) {
        for (i, amp) in self.amplitudes.iter().enumerate() {
            if amp.norm_sqr() > 1e-10 {
                println!("|{:0width$b}> {:.6} + {:.6}",
                    i,
                    amp.re,
                    amp.im,
                    width = self.qubits
                );
            }
        }
    }
}
