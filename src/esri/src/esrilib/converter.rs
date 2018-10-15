use esrilib::*;

impl Esri {
    pub fn to_polygons(&self) -> Vec<Polygon> {

        for r in 0..self.n_rows {
            for c in 0..self.n_cols {

            }
        }
        let mut points = Vec::new();
        points.push(Point{ x: 3, y: 4 } );
        points.push(Point{ x: 2, y: 1 } );
        points.push(Point{ x: 5, y: 9 } );

        let mut polygons : Vec<Polygon> = Vec::new();
        polygons.push(
            Polygon::new(points)
        );
        polygons
    }

    fn get_upper_left_point(&self, r: i32, c: i32) -> Point {
        Point {
            x: self.x_origin + r * self.cell_size,
            y: self.y_origin - c * self.cell_size,
        }
    }

    fn get_upper_right_point(&self, r: i32, c: i32) -> Point {
        self.get_upper_left_point(r, c).add_x(1.0)
    }

    fn get_lower_left_point(&self, r: i32, c: i32) -> Point {
        self.get_upper_left_point(r, c).add_y(-1.0)
    }

    fn get_lower_right_point(&self, r: i32, c: i32) -> Point {
        self.get_lower_left_point(r, c).add_x(1.0)
    }
}
