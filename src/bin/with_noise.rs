use gnuplot::*;
use probabilistic_robotics::prob_rob_lib::math_utils::*;
use probabilistic_robotics::prob_rob_lib::robot2d::*;

fn main() {
    let mut current = State2d { x: 0.0, y: 0.0, theta: 0.0 };
    let cntl = Control { v: 0.1, w: deg_to_rad(10.0) };

    let num = 30;
    let noisy = true;

    let mut fg = Figure::new();
    for _ in 0..10 {
        let (mut x,mut y) = calc_in_time(&mut current, &cntl, num, noisy);
        fg.axes2d()
            .set_aspect_ratio(Fix(1.0))
            .set_x_range(Fix(-1.0), Fix(1.0))
            .set_y_range(Fix(-0.5), Fix(1.5))
            .lines_points(
            &x, &y, 
            &[Caption("robot"), Color("blue"), PointSymbol('s'), PointSize(2.0)])
            ;
    }
    fg.show();
}