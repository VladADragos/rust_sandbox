pub struct Vec2d {
    x: f64,
    y: f64,
}

// pub fn new(x: f64, y: f64) -> Vec2d {
//     Vec2d { x: x, y: y }
// }

// pub fn distance_between_points(point1: Vec2d, point2: Vec2d) -> f64 {
//     ((point2.x - point1.x).powi(2) + (point2.y - point1.y).powi(2)).sqrt()
// }

impl Vec2d {
    pub const fn new(x: f64, y: f64) -> Vec2d {
        Vec2d { x: x, y: y }
    }

    pub fn vecAdd(&mut self, v2: Vec2d) {
        self.x += v2.x;
        self.y += v2.y;
    }

    pub fn scalarAdd(&mut self, n: f64) {
        self.x += n;
        self.y += n;
    }

    pub fn scale(&mut self, n: f64) {
        self.x *= n;
        self.y *= n;
    }

    pub fn distance(&mut self, v2: Vec2d) -> f64 {
        ((v2.x - self.x).powi(2) + (v2.y - self.y).powi(2)).sqrt()
    }
}
