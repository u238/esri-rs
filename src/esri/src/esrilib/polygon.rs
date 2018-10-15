use esrilib::Polygon;
use esrilib::Point;

impl Polygon {
    pub fn new(points: Vec<Point>) -> Polygon {
        Polygon { points }
    }
}

impl Point {
    pub fn add_x(&self, delta: f32) -> &mut Point {
        self.x += delta;
        self
    }

    pub fn add_y(&self, delta: f32) -> &mut Point {
        self.y += delta;
        self
    }
}