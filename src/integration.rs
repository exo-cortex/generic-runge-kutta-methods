#![allow(dead_code)]

pub trait Zero {
    fn zero() -> Self;
}

pub trait State:
    Sized
    + Clone
    + Copy
    + Default
    + Zero
    + std::ops::Mul<f64, Output = Self>
    + std::ops::Add<Self, Output = Self>
    + std::ops::AddAssign // + std::ops::Div<f64, Output = Self>
{
    fn out(&self) -> f64;
}

pub fn rk<S, const N: usize>(state: &mut S, f: fn(&S) -> S, tableau: &Tableau<N>, h: f64)
where
    S: State,
{
    let mut ks = vec![S::zero(); Tableau::<N>::num_ks()];

    ks[0] = f(&state);

    for i in 1..ks.len() {
        let mut input = S::zero();
        for k in 0..i {
            input += ks[k] * tableau.a[i][k];
        }
        input = *state + input * h;
        ks[i] = f(&input);
    }

    *state = *state
        + ks.into_iter()
            .enumerate()
            .fold(S::zero(), |acc, (i, k)| acc + k * tableau.b[i])
            * h;
}

pub struct Tableau<const N: usize> {
    a: [[f64; N]; N],
    b: [f64; N],
    c: [f64; N],
}

pub trait ButcherTableau<const N: usize> {
    fn num_ks() -> usize;
    fn a(&self, i: usize, j: usize) -> f64;
}

impl<const N: usize> ButcherTableau<N> for Tableau<N> {
    fn num_ks() -> usize {
        N
    }
    fn a(&self, i: usize, j: usize) -> f64 {
        self.a[i][j]
    }
}

pub const TABLEAU_EULER: Tableau<1> = Tableau::<1> {
    a: [[0.0]],
    b: [1.0],
    c: [0.0],
};

pub const TABLEAU_EULER_IMPLICIT: Tableau<1> = Tableau::<1> {
    a: [[1.0]],
    b: [1.0],
    c: [1.0],
};

pub const TABLEAU_HEUN_2: Tableau<2> = Tableau::<2> {
    a: [[0.0, 0.0], [1.0, 0.0]],
    b: [0.5, 0.5],
    c: [0.0, 1.0],
};

pub const TABLEAU_HEUN_3: Tableau<3> = Tableau::<3> {
    a: [
        [0.0, 0.0, 0.0],
        [1.0 / 3.0, 0.0, 0.0],
        [0.0, 2.0 / 3.0, 0.0],
    ],
    b: [1.0 / 4.0, 0.0, 3.0 / 4.0],
    c: [0.0, 1.0 / 3.0, 2.0 / 3.0],
};

pub const TABLEAU_RK_3: Tableau<3> = Tableau::<3> {
    a: [[0.0, 0.0, 0.0], [0.5, 0.0, 0.0], [-1.0, 2.0, 0.0]],
    b: [1.0 / 6.0, 2.0 / 3.0, 1.0 / 6.0],
    c: [0.0, 0.5, 1.0],
};

pub const TABLEAU_RK_4: Tableau<4> = Tableau::<4> {
    a: [
        [0.0, 0.0, 0.0, 0.0],
        [0.5, 0.0, 0.0, 0.0],
        [0.0, 0.5, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
    ],
    b: [1.0 / 6.0, 1.0 / 3.0, 1.0 / 3.0, 1.0 / 6.0],
    c: [0.0, 0.5, 0.5, 1.0],
};

pub const TABLEAU_RALSTON: Tableau<2> = Tableau::<2> {
    a: [[0.0, 0.0], [2.0 / 3.0, 0.0]],
    b: [0.25, 0.75],
    c: [0.0, 2.0 / 3.0],
};
