use probabilistic_robotics::prob_rob_lib::math_utils::*;
use probabilistic_robotics::prob_rob_lib::robot2d::*;

fn main() {
    let initial_state = State2d {
        x: 0.0,
        y: 0.0,
        theta: 0.0,
    };
    let cntl = Control {
        v: 0.1,
        w: deg_to_rad(10.0),
    };
    let end_time = 30.0;
    let noisy = true;

    draw_animation(initial_state, cntl, end_time, noisy);
}
