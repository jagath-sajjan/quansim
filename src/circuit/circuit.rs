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

    pub fn draw(&self) -> String {
        let mut lines: Vec<Vec<String>> = vec![Vec::new(); self.qubits];

        for gate in &self.gates {
            match gate {
                Gate::H(q) => {
                    for i in 0..self.qubits {
                        if i == *q {
                            lines[i].push("──H──".to_string());
                        } else {
                            lines[i].push("─────".to_string());
                        }
                    }
                }

                Gate::X(q) => {
                    for i in 0..self.qubits {
                        if i == *q {
                            lines[i].push("──X──".to_string());
                        } else {
                            lines[i].push("─────".to_string());
                        }
                    }
                }

                Gate::CNOT(c, t) => {
                    for i in 0..self.qubits {
                        if i == *c {
                            lines[i].push("──■──".to_string());
                        } else if i == *t {
                            lines[i].push("──X──".to_string());
                        } else {
                            lines[i].push("──│──".to_string());
                        }
                    }
                }
            }
        }

        let mut result = String::new();

        for (i, line) in lines.iter().enumerate() {
            result.push_str(&format!("q{} ", i));
            for gate in line {
                result.push_str(gate);
            }
            result.push('\n');
        }

        result
    }
}
