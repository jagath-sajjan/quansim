# quansim

a quantum circuit simulator written in rust runs entirely on your machine no quantum hardware needed

## what it does

simulates quantum circuits using a full complex amplitude state vector supports gates measurement algorithms and noise models exports to openqasm 2.0 which is readable by real quantum computers like ibm quantum

## install

```
git clone https://github.com/jagath-sajjan/quansim
cd quansim
cargo build --release
```

## usage

### interactive repl

```
cargo run
```

type gates and commands live see state update in real time type `help` for all commands

```
q[2]> h 0
q[2]> cx 0 1
q[2]> probs
q[2]> shots 1000
q[2]> qasm
```

### demos

```
cargo run -- demo bell
cargo run -- demo ghz 4
cargo run -- demo grover 3 5
cargo run -- demo qft 3
cargo run -- demo teleport
```

### run a qasm file

```
cargo run -- run circuit.qasm
```

## gates

single qubit: H X S T RX RY RZ Phase

two qubit: CNOT CZ SWAP CRX CRY CRZ

three qubit: Toffoli

## algorithms

- quantum fourier transform and inverse qft
- grovers search algorithm
- quantum teleportation

## analysis

- full state vector amplitudes
- probability distributions
- single shot and multi shot measurement
- expectation values X Y Z per qubit
- state fidelity between two states
- density matrix purity and trace
- circuit depth and gate count
- depolarizing bit flip and phase flip noise

## qubit limit

around 28 qubits before memory becomes a constraint on a standard machine a 15 qubit circuit uses about 512kb of ram

## dependencies

num-complex rand rayon crossterm indicatif sysinfo
