use esrilib::*;

impl Esri {
    pub fn to_polygons(&self) -> Vec<Polygon> {

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
}