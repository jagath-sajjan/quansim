use eframe::egui;
use crate::sim::state::QuantumState;
use crate::sim::gates::*;
use crate::sim::measure::*;
use crate::stats::system::SystemStats;

pub struct QuanSimApp {
    state: QuantumState,
    stats: SystemStats,
    last_measurement: usize,
}

impl Default for QuanSimApp {
    fn default() -> Self {
        let mut state = QuantumState::new(2);

        hadamard(&mut state, 0);
        cnot(&mut state, 0, 1);

        let result = measure(&state);

        Self {
            state,
            stats: SystemStats::new(),
            last_measurement: result,
        }
    }
}

impl eframe::App for QuanSimApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        self.stats.refresh();

        egui::CentralPanel::default().show(ctx, |ui| {

            ui.heading("Quantum Simulator");

            ui.separator();

            ui.horizontal(|ui| {

                ui.vertical(|ui| {
                    ui.heading("Circuit");

                    ui.label("H ──■──");
                    ui.label("    │");
                    ui.label("    X");
                });

                ui.separator();

                ui.vertical(|ui| {
                    ui.heading("System Stats");

                    ui.label(format!("CPU: {:.2}%", self.stats.cpu_usage()));
                    ui.label(format!("RAM: {} MB", self.stats.ram_used_mb()));
                    ui.label(format!("Threads: {}", self.stats.thread_count()));
                });
            });

            ui.separator();

            ui.horizontal(|ui| {

                ui.vertical(|ui| {
                    ui.heading("State Vector");

                    for (i, amp) in self.state.amplitudes.iter().enumerate() {
                        if amp.norm() > 0.0001 {
                            ui.label(format!("|{:02b}> {:.6}", i, amp.re));
                        }
                    }
                });

                ui.separator();

                ui.vertical(|ui| {
                    ui.heading("Measurement");

                    ui.label(format!("Last result: {:02b}", self.last_measurement));

                    if ui.button("Measure Again").clicked() {
                        self.last_measurement = measure(&self.state);
                    }
                });

            });

        });
    }
}
