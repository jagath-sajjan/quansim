use crate::sim::state::QuantumState;
use crate::sim::gates::{hadamard, x, cnot, rx, ry, rz, cz, swap, toffoli, phase, s, t};

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

    pub fn cz(&mut self, control: usize, target: usize) {
        self.circuit.gates.push(Gate::CZ(control, target));
    }

    pub fn swap(&mut self, a: usize, b: usize) {
        self.circuit.gates.push(Gate::SWAP(a, b));
    }

    pub fn toffoli(&mut self, c0: usize, c1: usize, target: usize) {
        self.circuit.gates.push(Gate::Toffoli(c0, c1, target));
    }

    pub fn rx(&mut self, target: usize, theta: f64) {
        self.circuit.gates.push(Gate::RX(target, theta));
    }

    pub fn ry(&mut self, target: usize, theta: f64) {
        self.circuit.gates.push(Gate::RY(target, theta));
    }

    pub fn rz(&mut self, target: usize, theta: f64) {
        self.circuit.gates.push(Gate::RZ(target, theta));
    }

    pub fn phase(&mut self, target: usize, phi: f64) {
        self.circuit.gates.push(Gate::Phase(target, phi));
    }

    pub fn s(&mut self, target: usize) {
        self.circuit.gates.push(Gate::S(target));
    }

    pub fn t(&mut self, target: usize) {
        self.circuit.gates.push(Gate::T(target));
    }

    pub fn run(&self) -> QuantumState {
        let mut state = QuantumState::new(self.circuit.qubits);

        for gate in &self.circuit.gates {
            match gate {
                Gate::H(q) => hadamard(&mut state, *q),
                Gate::X(q) => x(&mut state, *q),
                Gate::CNOT(c, t) => cnot(&mut state, *c, *t),
                Gate::CZ(c, t) => cz(&mut state, *c, *t),
                Gate::SWAP(a, b) => cz(&mut state, *a, *b),
                Gate::Toffoli(c0, c1, t) => toffoli(&mut state, *c0, *c1, *t),
                Gate::RX(q, theta) => rx(&mut state, *q, *theta),
                Gate::RY(q, theta) => ry(&mut state, *q, *theta),
                Gate::RZ(q, theta) => rz(&mut state, *q, *theta),
                Gate::Phase(q, phi) => phase(&mut state, *q, *phi),
                Gate::S(q) => s(&mut state, *q),
                Gate::T(q) => t(&mut state, *q),
            }
        }

        state
    }

    pub fn circuit(&self) -> &Circuit {
        &self.circuit
    }
}
