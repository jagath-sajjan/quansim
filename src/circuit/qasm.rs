use crate::circuit::circuit::{Circuit, Gate};

pub fn to_qasm(circuit: &Circuit) -> String {
    let mut out = String::new();

    out.push_str("OPENQASM 2.0;\n");
    out.push_str("include \"qelib1.inc\";\n");
    out.push_str(&format!("qreg q[{}];\n", circuit.qubits));
    out.push_str(&format!("creg c[{}];\n", circuit.qubits));

    for gate in &circuit.gates {
        let line = match gate {
            Gate::H(q)               => format!("h q[{}];", q),
            Gate::X(q)               => format!("x q[{}];", q),
            Gate::S(q)               => format!("s q[{}];", q),
            Gate::T(q)               => format!("t q[{}];", q),
            Gate::CNOT(c, t)         => format!("cx q[{}], q[{}];", c, t),
            Gate::CZ(c, t)           => format!("cz q[{}], q[{}];", c, t),
            Gate::SWAP(a, b)         => format!("swap q[{}], q[{}];", a, b),
            Gate::Toffoli(c0, c1, t) => format!("ccx q[{}], q[{}], q[{}];", c0, c1, t),
            Gate::RX(q, theta)       => format!("rx({:.6}) q[{}];", theta, q),
            Gate::RY(q, theta)       => format!("ry({:.6}) q[{}];", theta, q),
            Gate::RZ(q, theta)       => format!("rz({:.6}) q[{}];", theta, q),
            Gate::CRX(c, t, theta)   => format!("crx({:.6}) q[{}], q[{}];", theta, c, t),
            Gate::CRY(c, t, theta)   => format!("cry({:.6}) q[{}], q[{}];", theta, c, t),
            Gate::CRZ(c, t, theta)   => format!("crz({:.6}) q[{}], q[{}];", theta, c, t),
            Gate::Phase(q, phi)      => format!("p({:.6}) q[{}];", phi, q),
        };
        out.push_str(&line);
        out.push('\n');
    }

    out
}
