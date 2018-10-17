use esrilib::polygon::Polygon;
use esrilib::point::Point;
use esrilib::Esri;

impl Esri {
    pub fn to_polygons(&self) -> Vec<Polygon> {

        let mut points = Vec::new();
        for _r in 0..self.n_rows {
            for _c in 0..self.n_cols {
                let mut p = Point::new();
                p.get_upper_left_point(_r, _c);
                points.push(p);
            }
        }
        /*
        points.push(Point{ x: 3.0, y: 4.0 } );
        points.push(Point{ x: 2.0, y: 1.0 } );
        points.push(Point{ x: 5.0, y: 9.0 } );
*/
        let mut polygons : Vec<Polygon> = Vec::new();
        polygons.push(
            Polygon::new(points)
        );
        polygons
    }



}
