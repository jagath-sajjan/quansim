use sysinfo::System;

pub struct SystemStats {
    sys: System,
}

impl SystemStats {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();

        Self { sys }
    }

    pub fn refresh(&mut self) {
        self.sys.refresh_all();
    }

    pub fn cpu_usage(&self) -> f32 {
        self.sys.global_cpu_usage()
    }

    pub fn ram_used_mb(&self) -> u64 {
        self.sys.used_memory() / 1024
    }

    pub fn thread_count(&self) -> usize {
        self.sys.cpus().len()
    }
}
