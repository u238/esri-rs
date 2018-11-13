use esrilib::point::Point;

pub struct Polygon {
    points: Vec<Point>,
}

impl Polygon {
    pub fn new(points: Vec<Point>) -> Polygon {
        Polygon { points }
    }

    pub fn to_string(&self) -> String {
        let mut s: String = format!("[ {:?} ]", self.points);
        s.push_str("a test00");
        s
    }
}


impl std::fmt::Debug for Polygon {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[ {:?} ]", self.points)
    }
}

impl std::string::ToString for Polygon {
    fn to_string(&self) -> String {
        let mut s: String = format!("[ {:?} ]", self.points);
        s.push_str("a test00");
        s
    }
}