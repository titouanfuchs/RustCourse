pub trait Shape {
    fn surface(&self) -> f32;
    fn perimeter(&self) -> f32;
}

pub struct Rect {
    l: f32,
    L: f32,
}

impl Rect {
    pub fn new(le: f32, Le: f32) -> Rect {
        Rect { L: Le, l: le }
    }
}

impl Shape for Rect {
    fn surface(&self) -> f32 {
        self.l * self.L
    }

    fn perimeter(&self) -> f32 {
        (self.l + self.L) * 2.0
    }
}

pub struct Circle {
    r: f32,
}

impl Circle {
    pub fn new(re: f32) -> Circle {
        Circle { r: re }
    }
}

impl Shape for Circle {
    fn surface(&self) -> f32 {
        self.r.powf(2.0) * 3.14f32
    }

    fn perimeter(&self) -> f32 {
        2.0 * 3.14 * self.r
    }
}
