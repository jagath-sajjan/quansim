use std::{fs, usize};
use crate::circuit::builder::CircuitBuilder;
use crate::sim::measure::measure_many;
use crate::sim::probabilities::probabilities;
use crate::stats::histogram::histogram;
use crate::stats::ascii_histogram::print_histogram;

pub fn run(args: &[String]) {
    let path = match args.get(0) {
        Some(p) => p,
        None => {
            println!("usage: cargo run -- run <circuit.qasm>");
            return;
        }
    };

    let src = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            println!("error reading {}: {}", path, e);
            return;
        }
    };

    let mut qubits = 2;
    let mut gates: Vec<(String, Vec<String>)> = Vec::new();

    for line in src.lines() {
        let line = line.trim();
        if line.starts_with("//") || line.is_empty() { continue; }
        if line.starts_with("OPENQASM") || line.starts_with("include") { continue; }

        if line.starts_with("qreg") {
            if let Some(n) = parse_reg_size(line) {
                qubits = n;
            }
            continue;
        }

        if line.starts_with("creg") { continue; }

        let line = line.trim_end_matches(';');
        let parts: Vec<&str> = line.splitn(2, ' ').collect();
        if parts.is_empty() { continue; }

        let gate = parts[0].to_string();
        let operands: Vec<String> = parts
            .get(1)
            .unwrap_or(&"")
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();

        gates.push((gate, operands));
    }

    let mut builder = CircuitBuilder::new(qubits);

    for (gate, ops) in &gates {
        let q: Vec<usize> = ops.iter().filter_map(|s| parse_qubit_index(s)).collect();

        match gate.as_str() {
            "h"    => { if let Some(&a) = q.get(0) { builder.h(a); } }
            "x"    => { if let Some(&a) = q.get(0) { builder.x(a); } }
            "s"    => { if let Some(&a) = q.get(0) { builder.s(a); } }
            "t"    => { if let Some(&a) = q.get(0) { builder.t(a); } }
            "cx"   => { if let (Some(&c), Some(&t)) = (q.get(0), q.get(1)) { builder.cx(c, t); } }
            "cz"   => { if let (Some(&c), Some(&t)) = (q.get(0), q.get(1)) { builder.cz(c, t); } }
            "swap" => { if let (Some(&a), Some(&b)) = (q.get(0), q.get(1)) { builder.swap(a, b); } }
            "ccx"  => { if let (Some(&c0), Some(&c1), Some(&t)) = (q.get(0), q.get(1), q.get(2)) { builder.toffoli(c0, c1, t); } }
            g if g.starts_with("rx(") => {
                if let (Some(theta), Some(&tq)) = (parse_angle(g), q.get(0)) {
                    builder.rx(tq, theta);
                }
            }
            g if g.starts_with("ry(") => {
                if let (Some(theta), Some(&tq)) = (parse_angle(g), q.get(0)) {
                    builder.ry(tq, theta);
                }
            }
            g if g.starts_with("rz(") => {
                if let (Some(theta), Some(&tq)) = (parse_angle(g), q.get(0)) {
                    builder.rz(tq, theta);
                }
            }
            _ => println!("skipping unknown gate: {}", gate),
        }
    }

    println!("circuit from: {}", path);
    println!("{}", builder.circuit().draw());
    println!("depth: {}  gates: {}\n", builder.depth(), builder.gate_count());

    let state = builder.run();
    let probs = probabilities(&state);

    println!("probabilities:");
    for (i, p) in probs.iter().enumerate() {
        if *p > 1e-6 {
            println!("|{:0width$b}> {:.4}", i, p, width = qubits);
        }
    }

    println!();
    let results = measure_many(&state, 1000);
    let counts = histogram(&results);
    print_histogram(&counts, 1000);
}

fn parse_reg_size(line: &str) -> Option<usize> {
    let start = line.find('[')? + 1;
    let end = line.find(']')?;
    line[start..end].parse().ok()
}

fn parse_qubit_index(s: &str) -> Option<usize> {
    let start = s.find('[')? + 1;
    let end = s.find(']')?;
    s[start..end].parse().ok()
}

fn parse_angle(gate: &str) -> Option<f64> {
    let start = gate.find('(')? + 1;
    let end = gate.find(')')?;
    gate[start..end].parse().ok()
}
