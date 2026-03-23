#[derive(Clone)]
pub enum Gate {
    H(usize),
    X(usize),
    CNOT(usize, usize),
    CZ(usize, usize),
    SWAP(usize, usize),
    Toffoli(usize, usize, usize),
    RX(usize, f64),
    RY(usize, f64),
    RZ(usize, f64),
    CRX(usize, usize, f64),
    CRY(usize, usize, f64),
    CRZ(usize, usize, f64),
    Phase(usize, f64),
    S(usize),
    T(usize),
}

impl Gate {
    pub fn qubit_span(&self) -> (usize, usize) {
        match self {
            Gate::H(q) | Gate::X(q) | Gate::RX(q, _) | Gate::RY(q, _) | Gate::RZ(q, _)
            | Gate::Phase(q, _) | Gate::S(q) | Gate::T(q) => (*q, *q),
            Gate::CNOT(c, t) | Gate::CZ(c, t) | Gate::SWAP(c, t)
            | Gate::CRX(c, t, _) | Gate::CRY(c, t, _) | Gate::CRZ(c, t, _) => {
                (*c.min(t), *c.max(t))
            }
            Gate::Toffoli(c0, c1, t) => {
                let lo = *c0.min(c1).min(t);
                let hi = *c0.max(c1).max(t);
                (lo, hi)
            }
        }
    }
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

    pub fn gate_count(&self) -> usize {
        self.gates.len()
    }

    pub fn depth(&self) -> usize {
        let mut qubit_depth = vec![0usize; self.qubits];

        for gate in &self.gates {
            let (lo, hi) = gate.qubit_span();
            let max_so_far = *qubit_depth[lo..=hi].iter().max().unwrap_or(&0);
            for q in lo..=hi {
                qubit_depth[q] = max_so_far + 1;
            }
        }

        *qubit_depth.iter().max().unwrap_or(&0)
    }

    pub fn append(&mut self, other: &Circuit) {
        assert_eq!(
            self.qubits, other.qubits,
            "circuit qubit count mismatch on append"
        );
        self.gates.extend(other.gates.clone());
    }

    pub fn draw(&self) -> String {
        let mut lines: Vec<Vec<String>> = vec![Vec::new(); self.qubits];

        for gate in &self.gates {
            match gate {
                Gate::H(q)        => self.push_single(&mut lines, *q, "──H──"),
                Gate::X(q)        => self.push_single(&mut lines, *q, "──X──"),
                Gate::S(q)        => self.push_single(&mut lines, *q, "──S──"),
                Gate::T(q)        => self.push_single(&mut lines, *q, "──T──"),
                Gate::RX(q, _)    => self.push_single(&mut lines, *q, "─RX──"),
                Gate::RY(q, _)    => self.push_single(&mut lines, *q, "─RY──"),
                Gate::RZ(q, _)    => self.push_single(&mut lines, *q, "─RZ──"),
                Gate::Phase(q, _) => self.push_single(&mut lines, *q, "──P──"),
                Gate::CNOT(c, t) => {
                    for i in 0..self.qubits {
                        if i == *c      { lines[i].push("──■──".to_string()); }
                        else if i == *t { lines[i].push("──X──".to_string()); }
                        else            { lines[i].push("──│──".to_string()); }
                    }
                }
                Gate::CZ(c, t) => {
                    for i in 0..self.qubits {
                        if i == *c || i == *t { lines[i].push("──■──".to_string()); }
                        else                  { lines[i].push("──│──".to_string()); }
                    }
                }
                Gate::SWAP(a, b) => {
                    for i in 0..self.qubits {
                        if i == *a || i == *b { lines[i].push("──×──".to_string()); }
                        else                  { lines[i].push("──│──".to_string()); }
                    }
                }
                Gate::Toffoli(c0, c1, t) => {
                    for i in 0..self.qubits {
                        if i == *c0 || i == *c1 { lines[i].push("──■──".to_string()); }
                        else if i == *t         { lines[i].push("──X──".to_string()); }
                        else                    { lines[i].push("──│──".to_string()); }
                    }
                }
                Gate::CRX(c, t, _) => {
                    for i in 0..self.qubits {
                        if i == *c      { lines[i].push("──■──".to_string()); }
                        else if i == *t { lines[i].push("─RX──".to_string()); }
                        else            { lines[i].push("──│──".to_string()); }
                    }
                }
                Gate::CRY(c, t, _) => {
                    for i in 0..self.qubits {
                        if i == *c      { lines[i].push("──■──".to_string()); }
                        else if i == *t { lines[i].push("─RY──".to_string()); }
                        else            { lines[i].push("──│──".to_string()); }
                    }
                }
                Gate::CRZ(c, t, _) => {
                    for i in 0..self.qubits {
                        if i == *c      { lines[i].push("──■──".to_string()); }
                        else if i == *t { lines[i].push("─RZ──".to_string()); }
                        else            { lines[i].push("──│──".to_string()); }
                    }
                }
            }
        }

        let mut result = String::new();
        for (i, line) in lines.iter().enumerate() {
            result.push_str(&format!("q{} ", i));
            for seg in line {
                result.push_str(seg);
            }
            result.push('\n');
        }
        result
    }

    fn push_single(&self, lines: &mut Vec<Vec<String>>, target: usize, label: &str) {
        for i in 0..self.qubits {
            if i == target { lines[i].push(label.to_string()); }
            else           { lines[i].push("─────".to_string()); }
        }
    }
}
