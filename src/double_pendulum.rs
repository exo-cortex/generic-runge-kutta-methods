use crate::integration::{rk, State, System, Tableau, Zero};
use derive_more::{Add, AddAssign, Div, Mul};

// double pendulum code adapted from here:
// https://diego.assencio.com/?index=e5ac36fcb129ce95a61f8e8ce0572dbf

pub struct DynamicalSystem {} // empty struct

impl System for DynamicalSystem {
    type StateT = DoublePendulumState;
    type ParamT = Parameters;
    fn f(i: &Self::StateT, p: &Self::ParamT) -> Self::StateT {
        let h1 = i.p1 * i.p2 * (i.phi1 - i.phi2).sin() / p.l1
            * p.l2
            * (p.m1 + p.m2 * (i.phi1 - i.phi2).sin().powi(2));

        let h2 = (p.m2 * p.l2.powi(2) * i.p1.powi(2) + (p.m1 + p.m2) * p.l1.powi(2) * i.p2.powi(2)
            - 2.0 * p.m2 * p.l1 * p.l2 * i.p1 * i.p2 * (i.phi1 - i.phi2).cos())
            / (2.0
                * p.l1.powi(2)
                * p.l2.powi(2)
                * (p.m1 + p.m2 * (i.phi1 - i.phi2).sin().powi(2)).powi(2));

        let phi_diff = i.phi1 - i.phi2;
        DoublePendulumState {
            phi1: (p.l2 * i.p1 - p.l1 * i.p2 * phi_diff.cos())
                / (p.l1.powi(2) * p.l2 * (p.m1 + p.m2 * phi_diff.sin().powi(2))), // check
            p1: -(p.m1 - p.m2) * p.g * p.l1 * i.phi1.powi(2) - h1
                + h2 * (2.0 * (i.phi1 - i.phi2)).sin(),
            phi2: (-p.m2 * p.l2 * i.p1 * (i.phi1 - i.phi2).cos() + (p.m1 + p.m2) * p.l1 * i.p2)
                / p.m2
                * p.l1
                * p.l2.powi(2)
                * (p.m1 + p.m2 * (i.phi1 - i.phi2).sin().powi(2)),
            p2: -p.m2 * p.g * p.l2 * i.phi2.sin() + h1 - h2 * (2.0 * (i.phi1 - i.phi2)).sin(),
        }
    }
}

#[derive(Debug, Add, AddAssign, Mul, Div, Copy, Clone)]
pub struct Parameters {
    pub l1: f64,
    pub l2: f64,
    pub m1: f64,
    pub m2: f64,
    pub g: f64,
}

impl Zero for Parameters {
    fn zero() -> Self {
        Parameters {
            l1: 0.0,
            l2: 0.0,
            m1: 0.0,
            m2: 0.0,
            g: 0.0,
        }
    }
}

impl Default for Parameters {
    fn default() -> Self {
        Parameters {
            l1: 1.0,
            l2: 1.0,
            m1: 1.0,
            m2: 1.0,
            g: 8.0, // 9.81 m / s^2
        }
    }
}

impl State for Parameters {
    fn out(&self) -> Vec<f64> {
        vec![self.l1, self.l2, self.m1, self.m2, self.g]
    }
}

#[derive(Debug, Default, Add, AddAssign, Mul, Div, Copy, Clone)]
pub struct DoublePendulumState {
    pub phi1: f64,
    pub p1: f64,
    pub phi2: f64,
    pub p2: f64,
}

impl Zero for DoublePendulumState {
    fn zero() -> Self {
        DoublePendulumState {
            phi1: 0.0,
            p1: 0.0,
            phi2: 0.0,
            p2: 0.0,
        }
    }
}

impl State for DoublePendulumState {
    fn out(&self) -> Vec<f64> {
        vec![self.phi1, self.p1, self.phi2, self.p2]
    }
}
