use super::state::QuantumState;
use num_complex::Complex64;

pub fn hadamard(state: &mut QuantumState, target: usize) {
    let n = state.amplitudes.len();
    let step = 1 << target;

    let inv_sqrt2 = 1.0 / (2.0_f64).sqrt();

    for i in (0..n).step_by(step * 2) {
        for j in 0..step {
            let a = state.amplitudes[i + j];
            let b = state.amplitudes[i + j + step];

            state.amplitudes[i + j] = (a + b) * inv_sqrt2;
            state.amplitudes[i + j + step] = (a - b) * inv_sqrt2;
        }
    }
}

pub fn x(state: &mut QuantumState, target: usize) {
    let n = state.amplitudes.len();
    let step = 1 << target;

    for i in (0..n).step_by(step * 2) {
        for j in 0..step {
            state.amplitudes.swap(i + j, i + j + step);
        }
    }
}

pub fn cnot(state: &mut QuantumState, control: usize, target: usize) {
    let n = state.amplitudes.len();

    for i in 0..n {
        if ((i >> control) & 1) == 1 {
            let flipped = i ^ (1 << target);

            if i < flipped {
                state.amplitudes.swap(i, flipped);
            }
        }
    }
}

pub fn rx(state: &mut QuantumState, target: usize, theta: f64) {
    let n = state.amplitudes.len();
    let step = 1 << target;
    let cos = (theta / 2.0).cos();
    let sin = (theta / 2.0).sin();

    for i in (0..n).step_by(step * 2) {
        for j in 0..step {
            let a = state.amplitudes[i + j];
            let b = state.amplitudes[i + j + step];
            state.amplitudes[i + j] = a * cos - b * Complex64::new(0.0, sin);
            state.amplitudes[i + j +  step] = b * cos - a * Complex64::new(0.0, sin);
        }
    }
}

pub fn ry(state: &mut QuantumState, target: usize, theta: f64) {
    let n = state.amplitudes.len();
    let step = 1 << target;
    let neg = Complex64::new(0.0, -theta / 2.0).exp();
    let pos = Complex64::new(0.0, theta / 2.0).exp();

    for i in (0..n).step_by(step * 2) {
        for j in 0..step {
            state.amplitudes[i + j] *= neg;
            state.amplitudes[i + j + step] *= pos;
        }
    }
}

pub fn rz(state: &mut QuantumState, target: usize, theta: f64) {
    let n = state.amplitudes.len();
    let step = 1 << target;
    let neg = Complex64::new(0.0, -theta / 2.0).exp();
    let pos = Complex64::new(0.0, theta / 2.0).exp();

    for i in (0..n).step_by(step * 2) {
        for j in 0..step {
            state.amplitudes[i + j] *= neg;
            state.amplitudes[i + j + step] *= pos;
        }
    }
}

pub fn cz(state: &mut QuantumState, control: usize, target: usize) {
    let n = state.amplitudes.len();

    for i in 0..n {
        if ((i >> control) & 1) == 1 && ((i >> target) & 1) == 1 {
            state.amplitudes[i] *= -1.0;
        }
    }
}

pub fn swap(state: &mut QuantumState, a: usize, b: usize) {
    let n = state.amplitudes.len();

    for i in 0..n {
        let bit_a = (i >> a) & 1;
        let bit_b = (i >> b) & 1;
        if bit_a != bit_b {
            let j = i ^ (1 << a) ^ (1 << b);
            if i < j {
                state.amplitudes.swap(i, j);
            }
        }
    }
}

pub fn toffoli(state: &mut QuantumState, c0: usize, c1: usize, target: usize) {
    let n = state.amplitudes.len();

    for i in 0..n {
        if ((i >> c0) & 1) == 1 && ((i >> c1) & 1) == 1 {
            let flipped = i ^ (1 << target);
            if i < flipped {
                state.amplitudes.swap(i, flipped);
            }
        }
    }
}

pub fn phase(state: &mut QuantumState, target: usize, phi: f64) {
    let n = state.amplitudes.len();
    let step = 1 << target;
    let factor = Complex64::new(0.0, phi).exp();

    for i in (0..n).step_by(step * 2) {
        for j in 0..step {
            state.amplitudes[i + j + step] *= factor;
        }
    }
}

pub fn s(state: &mut QuantumState, target: usize) {
    phase(state, target, std::f64::consts::FRAC_PI_2);
}

pub fn t(state: &mut QuantumState, target: usize) {
    phase(state, target, std::f64::consts::FRAC_PI_4);
}
