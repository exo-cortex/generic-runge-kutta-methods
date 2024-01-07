use derive_more::{Add, AddAssign, Div, Mul};
use integration::{
    rk, State, System, Tableau, Zero, TABLEAU_EULER, TABLEAU_HEUN_2, TABLEAU_HEUN_3,
    TABLEAU_RALSTON, TABLEAU_RK_3, TABLEAU_RK_4,
};

use std::f64::consts::TAU;

mod double_pendulum;
use double_pendulum::{DoublePendulumState, DynamicalSystem};

mod integration;

use std::{
    fs::File,
    io::{BufWriter, Write},
};

pub struct Integration<SysT, const N: usize>
where
    SysT: System,
{
    state: SysT::StateT,
    parameters: SysT::ParamT,
    stepsize: f64,
    time: f64,
    tableau: Tableau<N>,
    output: BufWriter<File>,
}

impl<SysT, const N: usize> Integration<SysT, N>
where
    SysT: System,
{
    pub fn new(x0: SysT::StateT, dt: f64, tableau: Tableau<N>, filename: &str) -> Self {
        let path = format!("./{}.dat", filename);
        println!("{}", &path);
        Integration {
            state: x0,
            parameters: SysT::ParamT::default(),
            stepsize: dt,
            time: 0.0,
            tableau,
            output: BufWriter::new(File::create(path).unwrap()),
        }
    }
    fn step(&mut self) {
        rk::<SysT, N>(
            &mut self.state,
            &self.parameters,
            SysT::f,
            &self.tableau,
            self.stepsize,
        );
        self.time += self.stepsize;
    }
    fn write(&mut self) {
        write!(
            &mut self.output,
            "{}\t{}",
            self.time,
            self.state
                .out()
                .iter()
                .map(|num| num.to_string())
                .collect::<Vec<_>>()
                .join("\t")
                + "\n"
        )
        .unwrap();
    }
    fn combined_step(&mut self) {
        self.write();
        self.step();
    }
}

fn main() {
    let stepsize = 0.01;
    let steps = 40000;
    let write_interval = 100;

    let pendulum0 = DoublePendulumState {
        phi1: 0.0,
        phi2: 0.0,
        p1: 0.0,
        p2: 0.5,
    };

    // let mut euler = Integration::<double_pendulum::DynamicalSystem, 1>::new(
    //     pendulum0,
    //     stepsize,
    //     TABLEAU_EULER,
    //     "euler",
    // );

    let mut rk_4 = Integration::<double_pendulum::DynamicalSystem, 4>::new(
        pendulum0,
        stepsize,
        TABLEAU_RK_4,
        "rk_4",
    );

    let mut ralston = Integration::<double_pendulum::DynamicalSystem, 2>::new(
        pendulum0,
        stepsize,
        TABLEAU_RALSTON,
        "ralston",
    );
    let mut heun_2 = Integration::<double_pendulum::DynamicalSystem, 2>::new(
        pendulum0,
        stepsize,
        TABLEAU_HEUN_2,
        "heun_2",
    );
    let mut heun_3 = Integration::<double_pendulum::DynamicalSystem, 3>::new(
        pendulum0,
        stepsize,
        TABLEAU_HEUN_3,
        "heun_3",
    );

    let mut rk_3 = Integration::<double_pendulum::DynamicalSystem, 3>::new(
        pendulum0,
        stepsize,
        TABLEAU_RK_3,
        "rk_3",
    );

    let mut rk_4 = Integration::<double_pendulum::DynamicalSystem, 4>::new(
        pendulum0,
        stepsize,
        TABLEAU_RK_4,
        "rk_4",
    );

    for _ in 0..steps / write_interval {
        for _ in 0..write_interval {
            // euler.combined_step();
            ralston.combined_step();
            heun_2.combined_step();
            heun_3.combined_step();
            rk_4.combined_step();
            rk_3.combined_step();
        }
    }

    // let mut ground_truth = BufWriter::new(File::create("./ground_truth_exp.dat").unwrap());
    // for x in 0..steps {
    //     let x = (x as f64) * stepsize;
    //     write!(&mut ground_truth, "{}\t{}\n", x, (-x).exp()).unwrap();
    //     // write!(&mut ground_truth, "{}\t{}\n", x, x.sin()).unwrap();
    // }
}
