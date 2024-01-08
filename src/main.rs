#![allow(unused_imports)]

use {
    derive_more::{Add, AddAssign, Div, Mul},
    std::{
        f64::consts::PI,
        fs::File,
        io::{BufWriter, Write},
    },
};

mod double_pendulum;
mod integration;

use double_pendulum::{DoublePendulumState, DynamicalSystem};
use integration::{
    rk, Integration, State, System, Tableau, TABLEAU_EULER, TABLEAU_HEUN_2, TABLEAU_HEUN_3,
    TABLEAU_RALSTON, TABLEAU_RK_3, TABLEAU_RK_4,
};

fn main() {
    let stepsize = 0.001;
    let integration_time = 100.0;
    let steps = (integration_time / stepsize) as usize;

    let write_interval = 10;

    let integrate_no_write = 10_000;
    let integrate_no_write = integrate_no_write * ((1.0 / stepsize) as usize);

    // initial state
    let pendulum0 = DoublePendulumState {
        phi1: 0.0,
        phi2: 1.02 * PI,
        p1: -0.001,
        p2: 0.138,
    };

    let mut euler = Integration::<double_pendulum::DynamicalSystem, 1>::new(
        pendulum0,
        stepsize,
        TABLEAU_EULER,
        "euler",
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

    // integrate without writing into file
    for _ in 0..integrate_no_write {
        euler.step();
        ralston.step();
        heun_2.step();
        heun_3.step();
        rk_4.step();
        rk_3.step();
    }

    for _ in 0..steps / write_interval {
        euler.write();
        ralston.write();
        heun_2.write();
        heun_3.write();
        rk_4.write();
        rk_3.write();
        for _ in 0..write_interval {
            euler.step();
            ralston.step();
            heun_2.step();
            heun_3.step();
            rk_4.step();
            rk_3.step();
        }
    }
}
