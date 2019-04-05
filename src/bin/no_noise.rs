#[macro_use]
extern crate probabilistic_robotics;
use probabilistic_robotics::prob_rob_lib::robot2d::*;
use probabilistic_robotics::prob_rob_lib::math_utils::*;

const INITIAL_STATE: State2d = State2d {
    x: 0.0,
    y: 0.0,
    theta: 0.0,
};

const CNTL: Control = Control {
    v: 0.1,
    w: deg2rad!(10.0),
};
const END_TIME: f64 = 30.0;
const NOISY: bool = false;
const NAME: &str = "no_noise";

fn main() {
    let create_gif = false;

    draw_animation(INITIAL_STATE, CNTL, END_TIME, NOISY, create_gif, NAME);
}

#[test]
fn no_noise_test() {
    let create_gif = true;

    draw_animation(INITIAL_STATE, CNTL, END_TIME, NOISY, create_gif, NAME);
}