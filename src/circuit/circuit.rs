#[derive(Clone)]
pub enum Gate {
    H(usize),
    X(usize),
    CNOT(usize, usize),
}

pub struct Circuit {
    pub qubits: usize,
    pub gates: Vec<Gate>,
}

impl Circuit {
    pub fn new(qubits: usize) -> Self {
        Self {
            qubits,
            gates: Vec::new(),
        }
    }
}
