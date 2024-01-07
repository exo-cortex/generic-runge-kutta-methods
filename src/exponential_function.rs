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
    fn out(&self) -> Vec<f64> {
        vec![self.x]
    }
}

fn f(input: &ThisState) -> ThisState {
    // ThisState { x: (input.x).cos() } // solution: f(x) = x.sin()
    ThisState { x: -input.x } // solution: f(x) = (-x).exp()
}
