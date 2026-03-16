#![allow(warnings)]

mod sim;
mod ui;
mod stats;
mod circuit;

use sim::state::QuantumState;
use sim::gates::*;
use sim::measure::*;
use stats::system::SystemStats;

use crate::stats::system;

fn main() {
    let mut state = QuantumState::new(2);

    hadmard(&mut state, 0);
    cnot(&mut state, 0, 1);

    println!("Quantum State:");
    state.print_state();

    let result = measure(&state);
    println!("\nMeasurement result: {:02b}", result);

    let mut sys_stats = SystemStats::new();
    sys_stats.refresh();

    let mut sys_stats = SystemStats::new();
sys_stats.refresh();

println!("\nSystem Stats");
println!("CPU Usage: {:.2}%", sys_stats.cpu_usage());
println!("RAM Used: {} MB", sys_stats.ram_used_mb());
println!("Threads: {}", sys_stats.thread_count());
println!("Simulator Memory: {} bytes", state.memory_bytes());
}
