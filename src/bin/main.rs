#[macro_use]
extern crate probabilistic_robotics;
use probabilistic_robotics::prob_rob_lib::math_utils::*;
use probabilistic_robotics::prob_rob_lib::robot2d::*;

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

fn main() {
    let mut robot = Robot2d::new(INITIAL_STATE, CNTL);
    let (mut t, dt) = (0.0, 0.1);
    let mut cl = h_analyzer_client_lib::HAnalyzerClient::new();
    let name = "guess".to_string();
    let truth = "truth".to_string();

    cl.connect_to_series(&name).unwrap();
    cl.connect_to_series(&truth);

    cl.clear_series(&name);
    cl.clear_series(&truth);

    while t <= END_TIME {
        robot.process(&mut t, dt);
        println!("{:?}", robot.guess);

        cl.send_point(&name, robot.guess.x, robot.guess.y).unwrap();
        cl.send_point(&truth, robot.truth.x, robot.truth.y).unwrap();

        // robot.draw(&mut ax2d);
        std::thread::sleep(std::time::Duration::from_millis((1000.0 * dt) as u64));
    }

    cl.runtime.shutdown_background();
}
