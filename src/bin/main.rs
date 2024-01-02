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

#[tokio::main]
async fn main() {
    let mut robot = Robot2d::new(INITIAL_STATE, CNTL);
    let (mut t, dt) = (0.0, 0.1);
    let mut cl = h_analyzer_client_lib::HAnalyzerClient::new().await;
    cl.register_new_world(&"prob_rob".to_string())
        .await
        .unwrap();

    let mut index = 0;
    while t <= END_TIME {
        robot.process(&mut t, dt);
        //println!("{:?}", robot.guess);
        // robot.draw(&mut ax2d);
        std::thread::sleep(std::time::Duration::from_millis((1000.0 * dt) as u64));

        let mut wf = h_analyzer_data::WorldFrame::new(index, t);
        let mut ego = h_analyzer_data::Entity::new();
        ego.add_estimate(
            "odom".to_string(),
            h_analyzer_data::Estimate::Pose2D(h_analyzer_data::Pose2D::new(
                robot.guess.x,
                robot.guess.y,
                robot.guess.theta,
            )),
        );
        ego.add_estimate(
            "truth".to_string(),
            h_analyzer_data::Estimate::Pose2D(h_analyzer_data::Pose2D::new(
                robot.truth.x,
                robot.truth.y,
                robot.truth.theta,
            )),
        );
        wf.add_entity("ego".to_string(), ego);
        cl.send_world_frame(wf).await.unwrap();

        index += 1;
    }
}
