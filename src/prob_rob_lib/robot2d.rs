
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

pub fn transit(cur: &mut State2d, cnt: &Control) {
    cur.x += cnt.v * cur.theta.cos();
    cur.y += cnt.v * cur.theta.sin();
    cur.theta += cnt.w;
}