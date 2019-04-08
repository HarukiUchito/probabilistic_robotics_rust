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
const NAME: &str = "dead_reckoning";

fn main() {
    let create_gif = false;
    let mut rb = Robot2d::new(INITIAL_STATE, CNTL);
    draw_robot_animation(&mut rb, END_TIME, create_gif, NAME);
}

#[test]
fn no_noise_test() {
    let create_gif = true;
    let mut rb = Robot2d::new(INITIAL_STATE, CNTL);
    draw_robot_animation(&mut rb, END_TIME, create_gif, NAME);
}