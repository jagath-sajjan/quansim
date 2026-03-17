use crate::sim::state::QuantumState;
use crate::sim::gates::{hadamard, x, cnot};

use super::circuit::{Circuit, Gate};

pub struct CircuitBuilder {
    circuit: Circuit,
}

impl CircuitBuilder {
    pub fn new(qubits: usize) -> Self {
        Self {
            circuit: Circuit::new(qubits),
        }
    }

    pub fn h(&mut self, target: usize) {
        self.circuit.gates.push(Gate::H(target));
    }

    pub fn x(&mut self, target: usize) {
        self.circuit.gates.push(Gate::X(target));
    }

    pub fn cx(&mut self, control: usize, target: usize) {
        self.circuit.gates.push(Gate::CNOT(control, target));
    }

    pub fn run(&self) -> QuantumState {
        let mut state = QuantumState::new(self.circuit.qubits);

        for gate in &self.circuit.gates {
            match gate {
                Gate::H(q) => hadamard(&mut state, *q),
                Gate::X(q) => x(&mut state, *q),
                Gate::CNOT(c, t) => cnot(&mut state, *c, *t),
            }
        }

        state
    }

    pub fn circuit(&self) -> &Circuit {
        &self.circuit
    }
}
