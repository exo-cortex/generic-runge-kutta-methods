use derive_more::{Add, AddAssign, Div, Mul};
mod integration;
use integration::{
    rk, State, Tableau, Zero, TABLEAU_EULER, TABLEAU_EULER_IMPLICIT, TABLEAU_HEUN_2,
    TABLEAU_HEUN_3, TABLEAU_RALSTON, TABLEAU_RK_3, TABLEAU_RK_4,
};

use std::{
    fs::File,
    io::{BufWriter, Write},
};

#[derive(Debug, Default, Add, AddAssign, Mul, Div, Copy, Clone)]
struct ThisState {
    pub x: f64,
}

impl Zero for ThisState {
    fn zero() -> Self {
        ThisState { x: 0.0 }
    }
}

impl State for ThisState {
    fn out(&self) -> f64 {
        self.x
    }
}

fn f(input: &ThisState) -> ThisState {
    // ThisState { x: (input.x).cos() } // solution: f(x) = x.sin()
    ThisState { x: -input.x } // solution: f(x) = (-x).exp()
}

pub struct Integration<const N: usize> {
    state: ThisState,
    stepsize: f64,
    time: f64,
    tableau: Tableau<N>,
    output: BufWriter<File>,
}

impl<const N: usize> Integration<N> {
    pub fn new(x0: f64, dt: f64, tableau: Tableau<N>, filename: &str) -> Self {
        let path = format!("./{}.dat", filename);
        println!("{}", &path);
        Integration {
            state: ThisState { x: x0 },
            stepsize: dt,
            time: 0.0,
            tableau,
            output: BufWriter::new(File::create(path).unwrap()),
        }
    }
    fn step(&mut self) {
        rk(&mut self.state, f, &self.tableau, self.stepsize);
        self.time += self.stepsize;
    }
    fn write(&mut self) {
        write!(&mut self.output, "{}\t{}\n", self.time, self.state.out()).unwrap();
    }
    fn combined_step(&mut self) {
        self.write();
        self.step();
    }
}

fn main() {
    let stepsize = 0.01;
    let steps = 1000;

    let y0 = 1.0;

    // let mut euler = Integration::new(y0, stepsize, TABLEAU_EULER, "euler");
    let mut euler_implicit =
        Integration::new(y0, stepsize, TABLEAU_EULER_IMPLICIT, "euler_implicit");
    let mut ralston = Integration::new(y0, stepsize, TABLEAU_RALSTON, "ralston");
    let mut heun_2 = Integration::new(y0, stepsize, TABLEAU_HEUN_2, "heun_2");
    let mut heun_3 = Integration::new(y0, stepsize, TABLEAU_HEUN_3, "heun_3");
    let mut rk_4 = Integration::new(y0, stepsize, TABLEAU_RK_4, "rk_4");
    let mut rk_3 = Integration::new(y0, stepsize, TABLEAU_RK_3, "rk_3");

    for _ in 0..steps {
        // euler.combined_step();
        euler_implicit.combined_step();
        ralston.combined_step();
        heun_2.combined_step();
        heun_3.combined_step();
        rk_4.combined_step();
        rk_3.combined_step();
    }

    let mut ground_truth = BufWriter::new(File::create("./ground_truth_exp.dat").unwrap());
    for x in 0..steps {
        let x = (x as f64) * stepsize;
        write!(&mut ground_truth, "{}\t{}\n", x, (-x).exp()).unwrap();
        // write!(&mut ground_truth, "{}\t{}\n", x, x.sin()).unwrap();
    }
}
