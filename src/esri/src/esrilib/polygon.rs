use esrilib::point::Point;

//extern crate serde;
//extern crate serde_json;
//
////use serde_json::Error;

//#[derive(Serialize, Deserialize)]
pub struct Polygon {
    pub points: Vec<Point>,
}

impl Polygon {
    pub fn new(points: Vec<Point>) -> Polygon {
        Polygon { points }
    }

    pub fn begins_with_point(&self, p: &Point) -> bool {
        self.points.len() > 0 && self.points[0] == *p
    }

    pub fn ends_with_point(&self, p: &Point) -> bool {
        self.points.len() > 0 && self.points[self.points.len()-1] == *p
    }

    pub fn add_point(&mut self, p: Point) -> &mut Polygon {
        self.points.push(p);
        self
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