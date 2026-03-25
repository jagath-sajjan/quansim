use std::io::{self, Write};
use crate::circuit::builder::CircuitBuilder;
use crate::circuit::qasm::to_qasm;
use crate::sim::measure::{measure, measure_many};
use crate::sim::probabilities::probabilities;
use crate::sim::expectation::{expect_x, expect_y, expect_z};
use crate::stats::histogram::histogram;
use crate::stats::ascii_histogram::print_histogram;

pub fn run() {
    println!("quansim — type 'help' for commands");

    let mut qubits = 2;
    let mut builder = CircuitBuilder::new(qubits);

    loop {
        print!("q[{}]> ", qubits);
        io::stdout().flush().unwrap();

        let mut line = String::new();
        if io::stdin().read_line(&mut line).is_err() {
            break;
        }

        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "help" => print_help(),

            "new" => {
                qubits = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(2);
                builder = CircuitBuilder::new(qubits);
                println!("new {}-qubit circuit", qubits);
            }

            "reset" => {
                builder = CircuitBuilder::new(qubits);
                println!("circuit cleared");
            }

            "h" => {
                if let Some(q) = parse_usize(parts.get(1)) {
                    builder.h(q);
                    println!("H q{}", q);
                }
            }

            "x" => {
                if let Some(q) = parse_usize(parts.get(1)) {
                    builder.x(q);
                    println!("X q{}", q);
                }
            }

            "cx" => {
                if let (Some(c), Some(t)) = (parse_usize(parts.get(1)), parse_usize(parts.get(2))) {
                    builder.cx(c, t);
                    println!("CNOT q{} q{}", c, t);
                }
            }

            "cz" => {
                if let (Some(c), Some(t)) = (parse_usize(parts.get(1)), parse_usize(parts.get(2))) {
                    builder.cz(c, t);
                    println!("CZ q{} q{}", c, t);
                }
            }

            "swap" => {
                if let (Some(a), Some(b)) = (parse_usize(parts.get(1)), parse_usize(parts.get(2))) {
                    builder.swap(a, b);
                    println!("SWAP q{} q{}", a, b);
                }
            }

            "rx" => {
                if let (Some(q), Some(theta)) = (parse_usize(parts.get(1)), parse_f64(parts.get(2))) {
                    builder.rx(q, theta);
                    println!("RX({:.4}) q{}", theta, q);
                }
            }

            "ry" => {
                if let (Some(q), Some(theta)) = (parse_usize(parts.get(1)), parse_f64(parts.get(2))) {
                    builder.ry(q, theta);
                    println!("RY({:.4}) q{}", theta, q);
                }
            }

            "rz" => {
                if let (Some(q), Some(theta)) = (parse_usize(parts.get(1)), parse_f64(parts.get(2))) {
                    builder.rz(q, theta);
                    println!("RZ({:.4}) q{}", theta, q);
                }
            }

            "s" => {
                if let Some(q) = parse_usize(parts.get(1)) {
                    builder.s(q);
                    println!("S q{}", q);
                }
            }

            "t" => {
                if let Some(q) = parse_usize(parts.get(1)) {
                    builder.t(q);
                    println!("T q{}", q);
                }
            }

            "toffoli" => {
                if let (Some(c0), Some(c1), Some(t)) = (
                    parse_usize(parts.get(1)),
                    parse_usize(parts.get(2)),
                    parse_usize(parts.get(3)),
                ) {
                    builder.toffoli(c0, c1, t);
                    println!("Toffoli q{} q{} q{}", c0, c1, t);
                }
            }

            "draw" => {
                println!("{}", builder.circuit().draw());
                println!("depth: {}  gates: {}", builder.depth(), builder.gate_count());
            }

            "state" => {
                let state = builder.run();
                for (i, amp) in state.amplitudes.iter().enumerate() {
                    if amp.norm_sqr() > 1e-10 {
                        println!("|{:0width$b}> {:.6} + {:.6}i", i, amp.re, amp.im, width = qubits);
                    }
                }
            }

            "probs" => {
                let state = builder.run();
                let probs = probabilities(&state);
                for (i, p) in probs.iter().enumerate() {
                    if *p > 1e-10 {
                        println!("|{:0width$b}> {:.4}", i, p, width = qubits);
                    }
                }
            }

            "measure" => {
                let state = builder.run();
                let result = measure(&state);
                println!("result: {:0width$b}", result, width = qubits);
            }

            "shots" => {
                let n = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(1000);
                let state = builder.run();
                let results = measure_many(&state, n);
                let counts = histogram(&results);
                print_histogram(&counts, n);
            }

            "expect" => {
                let state = builder.run();
                for q in 0..qubits {
                    println!(
                        "q{} <X>={:.4}  <Y>={:.4}  <Z>={:.4}",
                        q,
                        expect_x(&state, q),
                        expect_y(&state, q),
                        expect_z(&state, q)
                    );
                }
            }

            "qasm" => {
                println!("{}", to_qasm(builder.circuit()));
            }

            "quit" | "exit" | "q" => {
                println!("bye");
                break;
            }

            _ => println!("unknown cmd BRUH — type 'help'"),
        }
    }
}

fn print_help() {
    println!("
new <n>              new n-qubit circuit (default 2)
reset                clear all gates
draw                 show circuit diagram + depth/gates
state                show full quantum state amplitudes
probs                show probabilities of each basis state
measure              single measurement result
shots <n>            run n shots and show histogram (default 1000)
expect               show <X> <Y> <Z> expectation values per qubit
qasm                 export circuit as OpenQASM 2.0

gates:
  h <q>              Hadamard
  x <q>              Pauli-X
  s <q>              S gate
  t <q>              T gate
  rx <q> <theta>     RX rotation
  ry <q> <theta>     RY rotation
  rz <q> <theta>     RZ rotation
  cx <c> <t>         CNOT
  cz <c> <t>         CZ
  swap <a> <b>       SWAP
  toffoli <c0> <c1> <t>

demos (run outside the repl):
  cargo run -- demo bell
  cargo run -- demo ghz <n_qubits>
  cargo run -- demo grover <n_qubits> <target>
  cargo run -- demo qft <n_qubits>
  cargo run -- demo teleport

run a qasm file:
  cargo run -- run <circuit.qasm>

quit / exit / q      exit
");
}
fn parse_usize(s: Option<&&str>) -> Option<usize> {
    s?.parse().ok()
}

fn parse_f64(s: Option<&&str>) -> Option<f64> {
    s?.parse().ok()
}
