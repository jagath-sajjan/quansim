use std::usize;

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
    Phase(usize, f64),
    S(usize),
    T(usize),
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

    pub fn draw(&self) -> String {
        let mut lines: Vec<Vec<String>> = vec![Vec::new(); self.qubits];

        for gate in &self.gates {
            match gate {
                Gate::H(q) => self.push_single(&mut lines, *q, "──H──"),
                Gate::X(q) => self.push_single(&mut lines, *q, "──X──"),
                Gate::S(q) => self.push_single(&mut lines, *q, "──S──"),
                Gate::T(q) => self.push_single(&mut lines, *q, "──T──"),
                Gate::RX(q, _) => self.push_single(&mut lines, *q, "─RX──"),
                Gate::RY(q, _) => self.push_single(&mut lines, *q, "─RY──"),
                Gate::RZ(q, _) => self.push_single(&mut lines, *q, "─RZ──"),
                Gate::Phase(q, _) => self.push_single(&mut lines, *q, "──P──"),
                Gate::CNOT(c, t) => {
                    for i in 0..self.qubits {
                        if i == *c { lines[i].push("──■──".to_string()); }
                        else if i == *t { lines[i].push("──X──".to_string()); }
                        else { lines[i].push("──│──".to_string()); }
                    }
                }
                Gate::CZ(c, t) => {
                    for i in 0..self.qubits {
                        if i == *c || i == *t { lines[i].push("──■──".to_string()); }
                        else { lines[i].push("──│──".to_string()); }
                    }
                }
                Gate::SWAP(a, b) => {
                    for i in 0..self.qubits {
                        if i == *a || i == *b { lines[i].push("──X──".to_string()); }
                        else { lines[i].push("──│──".to_string()); }
                    }
                }
                Gate::Toffoli(c0, c1, t) => {
                    for i in 0..self.qubits {
                        if i == *c0 || i == *c1 { lines[i].push("──■──".to_string()); }
                        else if i == *t { lines[i].push("──X──".to_string()); }
                        else { lines[i].push("──│──".to_string()); }
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
            if i == target {
                lines[i].push(label.to_string());
            } else {
                lines[i].push("─────".to_string());
            }
        }
    }
}
