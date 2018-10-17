use esrilib::point::Point;

pub struct Polygon {
    points: Vec<Point>,
}

impl Polygon {
    pub fn new(points: Vec<Point>) -> Polygon {
        Polygon { points }
    }
}

