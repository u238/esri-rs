use std::fmt;

//#[derive(Serialize, Deserialize)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}


impl Point {
    pub fn new() -> Point {
        Point{
            x: 0.0,
            y: 0.0,
        }
    }

    pub fn get_upper_left_point(&mut self, r: i32, c: i32) -> &mut Point {
        self.x = r as f32;
        self.y = c as f32;
        self
    }

    pub fn get_upper_right_point(&mut self, r: i32, c: i32) -> &mut Point {
        self.get_upper_left_point(r, c).add_x(1.0)
    }

    pub fn get_lower_left_point(&mut self, r: i32, c: i32) -> &mut Point {
        self.get_upper_left_point(r, c).add_y(-1.0)
    }

    pub fn get_lower_right_point(&mut self, r: i32, c: i32) -> &mut Point {
        self.get_lower_left_point(r, c).add_x(1.0)
    }

    pub fn add_x(&mut self, delta: f32) -> &mut Point {
        self.x += delta;
        self
    }

    pub fn add_y(&mut self, delta: f32) -> &mut Point {
        self.y += delta;
        self
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl std::string::ToString for Point {
    fn to_string(&self) -> String {
        format!("({}, {})", self.x, self.y)
    }
}