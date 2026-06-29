# quansim

A quantum circuit simulator written in Rust, runs entirely on your machine, no quantum hardware needed.

📖 [Documentation](https://brewaway.gitbook.io/quansim-docs)

---

## What it does

Simulates quantum circuits using a full complex amplitude state vector. Supports gates, measurement, algorithms, and noise models. Exports to OpenQASM 2.0, which is readable by real quantum computers like IBM Quantum.

---

## Install

```bash
git clone https://github.com/jagath-sajjan/quansim
cd quansim
cargo build --release
```

---

## Usage

### Interactive REPL

```bash
cargo run
```

Type gates and commands live, see state update in real time. Type `help` for all commands.

```
q[2]> h 0
q[2]> cx 0 1
q[2]> probs
q[2]> shots 1000
q[2]> qasm
```

### Demos

```bash
cargo run -- demo bell
cargo run -- demo ghz 4
cargo run -- demo grover 3 5
cargo run -- demo qft 3
cargo run -- demo teleport
```

### Run a QASM file

```bash
cargo run -- run circuit.qasm
```

---

## Gates

| Type        | Gates                              |
|-------------|------------------------------------|
| Single qubit | H, X, S, T, RX, RY, RZ, Phase    |
| Two qubit   | CNOT, CZ, SWAP, CRX, CRY, CRZ     |
| Three qubit | Toffoli                            |

---

## Algorithms

- Quantum Fourier Transform (QFT) and inverse QFT
- Grover's search algorithm
- Quantum teleportation

---

## Analysis

- Full state vector amplitudes
- Probability distributions
- Single shot and multi shot measurement
- Expectation values (X, Y, Z) per qubit
- State fidelity between two states
- Density matrix purity and trace
- Circuit depth and gate count
- Depolarizing, bit flip, and phase flip noise models

---

## Qubit limit

Around **28 qubits** before memory becomes a constraint on a standard machine. A 15 qubit circuit uses approximately 512 KB of RAM.

---

## Dependencies

`num-complex` · `rand` · `rayon` · `crossterm` · `indicatif` · `sysinfo`
