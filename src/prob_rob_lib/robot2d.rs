
#[derive(Debug)]
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
    pub fn transit(&self, cur: &State2d) -> State2d {
        State2d {
            x: cur.x + self.v * cur.theta.cos(),
            y: cur.y + self.v * cur.theta.sin(),
            theta: cur.theta + self.w,
        }
    }
}