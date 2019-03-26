use crate::prob_rob_lib::math_utils::*;
use rand::distributions::{Normal, Distribution};

#[derive(Debug, Clone)]
pub struct State2d {
    pub x: f64,
    pub y: f64,
    pub theta: f64,
}

pub struct Control {
    pub v: f64, // The velocity of straight transition
    pub w: f64, // The angular velocity
}

impl Control {
    /// The transition of one sec
    pub fn transit(&self, cur: &State2d, noisy: bool) -> State2d {
        let (mut v, mut w) = (self.v, self.w);
        let mut dir_err = 0.0; // The error of direction for moving forward
        if noisy { // add noise of each control
            let nd = Normal::new(v, v / 10.0);
            v = nd.sample(&mut rand::thread_rng());
            let nd = Normal::new(0.0, deg_to_rad(3.0));
            dir_err = nd.sample(&mut rand::thread_rng());
            let nd = Normal::new(w, w / 10.0);
            w = nd.sample(&mut rand::thread_rng());
            println!("{:?}, {:?}, {:?}", v, dir_err, w);
        }
        State2d {
            x: cur.x + v * (cur.theta + dir_err).cos(),
            y: cur.y + v * (cur.theta + dir_err).sin(),
            theta: cur.theta + w,
        }
    }
}

pub fn calc_in_time(
    initial_state: &State2d,
    cntl: &Control,
    num: usize,
    noisy: bool
) -> (Vec<f64>, Vec<f64>) {
    let mut x = vec![0.0; num];
    let mut y = vec![0.0; num];
    let mut current = initial_state.clone();
    for i in 0..num {
        x[i] = current.x;
        y[i] = current.y;
        current = cntl.transit(&current, noisy);
    }
    (x, y)
}