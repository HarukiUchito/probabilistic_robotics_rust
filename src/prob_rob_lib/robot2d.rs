use crate::prob_rob_lib::math_utils::*;
use gnuplot::*;
use rand::distributions::{Distribution, Normal};
use std::thread::sleep;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct State2d {
    pub x: f64,
    pub y: f64,
    pub theta: f64,
}

#[derive(Debug)]
pub struct Control {
    pub v: f64, // The velocity of straight transition
    pub w: f64, // The angular velocity
}

impl Control {
    /// The transition of one sec
    pub fn transit(&self, cur: &State2d, t: &mut f64, dt: f64, noisy: bool) -> State2d {
        *t += dt;
        let (mut v, mut w) = (self.v, self.w);
        let mut dir_err = 0.0; // The error of direction for moving forward
        if noisy {
            // add noise of each control
            let nd = Normal::new(v, v / 10.0);
            v = nd.sample(&mut rand::thread_rng());
            let nd = Normal::new(0.0, deg2rad!(3.0));
            dir_err = nd.sample(&mut rand::thread_rng());
            let nd = Normal::new(w, w / 10.0);
            w = nd.sample(&mut rand::thread_rng());
            //println!("{:?}, {:?}, {:?}", v, dir_err, w);
        }
        State2d {
            x: cur.x + v * (cur.theta + dir_err).cos() * dt,
            y: cur.y + v * (cur.theta + dir_err).sin() * dt,
            theta: cur.theta + w * dt,
        }
    }
}

pub struct Robot2d {
    
}

pub fn draw_arrow(
    ax2d: &mut Axes2D,
    state2d: &State2d,
    length: f64,
    color: PlotOption<&str>,
) {
    let (x, y, th) = (state2d.x, state2d.y, state2d.theta);
    ax2d.arrow(
            Axis(x),
            Axis(y),
            Axis(x + length * th.cos()),
            Axis(y + length * th.sin()),
            &[ArrowType(Open), ArrowSize(0.1), Caption("arrow"), LineWidth(2.0), color],
        )
        .points(
            &[x],
            &[y],
            &[
                Caption("robot"),
                color,
                PointSymbol('O'),
                PointSize(0.5),
            ],
        );
}

pub fn draw_animation(
    initial_state: State2d,
    cntl: Control,
    end_time: f64,
    noisy: bool,
    create_gif: bool,
    name: &str,
) {
    let mut current = initial_state;

    let mut fg = Figure::new();
    if create_gif {
        fg.set_terminal("gif animate optimize delay 2 size 480,360", &("movie/".to_owned() + name + ".gif"));
    }
    let (mut t, dt) = (0.0, 0.1);
    while t <= end_time {
        if create_gif {
            fg.new_page();
        } else {
            fg.clear_axes();
        }
        current = cntl.transit(&current, &mut t, dt, noisy);
        
        let mut ax2d = fg.axes2d();
        ax2d.set_title(
                &format!(
                    "Control: v = {:?}[m/s], w = {:?}[deg], time {:.*}[s]",
                    cntl.v,
                    rad2deg!(cntl.w),
                    2,
                    t
                ),
                &[],)
            .set_aspect_ratio(Fix(1.0))
            .set_x_range(Fix(-1.0), Fix(1.0))
            .set_y_range(Fix(-0.5), Fix(1.5))
            .points(&[-1.0, -1.0, 1.0, 1.0], &[-0.5, 1.5, -0.5, 1.5], &[Color("black")])
            .set_x_grid(true)
            .set_y_grid(true)
            .set_grid_options(false, &[Color("black"), LineStyle(DashType::SmallDot)])
            ;
        draw_arrow(ax2d, &current, 0.125, Color("black"));
        if !create_gif {
            fg.show();
        }
        sleep(Duration::from_millis((1000.0 * dt) as u64));
    }
	if create_gif {
        fg.show();
    }
}
