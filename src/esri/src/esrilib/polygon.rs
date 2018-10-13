use esrilib::Polygon;
use esrilib::Point;

impl Polygon {
    pub fn new(points: Vec<Point>) -> Polygon {
        Polygon { points }
    }
}
