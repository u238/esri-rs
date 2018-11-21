use esrilib::point::Point;
use esrilib::polygon::Polygon;
use esrilib::esrifile::EsriFile;

pub struct EsriPolygons {
    esri: EsriFile,
    polygons: Vec<Polygon>,
    threshold: i32,
}

pub fn new_esripolygons(esri: EsriFile, polygons: Vec<Polygon>) -> EsriPolygons {
    EsriPolygons{
        esri,
        polygons,
        threshold: 1
    }
}

impl EsriPolygons {
    pub fn to_polygons(&mut self) -> &EsriPolygons {


        for _r in 0..self.esri.n_rows {
            for _c in 0..self.esri.n_cols {

                let p = self.esri.get_upper_left_point(_r, _c);

                match (
                    self.esri.get_upper_left_data_value(_r, _c) >= self.threshold,
                    self.esri.get_upper_data_value(_r, _c) >= self.threshold,
                    self.esri.get_left_data_value(_r, _c) >= self.threshold,
                    self.esri.get_esri_data_value(_r, _c) >= self.threshold
                ) {
                    (false, false, false, true) |
                    (true, true, true, false) => {
                        // new polygon
                        self.add_polygon(p);
                    }
                    (true, false, false, false) |
                    (true, false, false, true) |
                    (false, true, true, false) |
                    (false, true, true, true) => {
                        // merge polygons
                        let left_cell = self.esri.get_upper_left_point_of_upper_cell(_r, _c);

                        let mut points2 = {
                            let mut poly2 = self.extract_polygon_with_ending_point (left_cell);
                            poly2.points.reverse();
                            poly2.points
                        };

                        let mut poly = self.add_to_polygon_with_point_of_left_cell(p, _r, _c).unwrap();

                        poly.points.append(&mut points2);

                    }
                    (false, true, false, false) |
                    (false, true, false, true) |
                    (true, false, true, false) |
                    (true, false, true, true) => {
                        // add to polygon upper_cell
                        let upper_cell = self.esri.get_upper_left_point_of_upper_cell(_r, _c);

                        match self.search_polygon_with_ending_point(upper_cell) {
                            Some(mut poly) => { poly.add_point(p); }
                            None => {}
                        }
                    }
                    (true, true, false, false) |
                    (true, true, false, true) |
                    (false, false, true, false) |
                    (false, false, true, true) => {
                        // add to polygon left cell
                        self.add_to_polygon_with_point_of_left_cell(p, _r, _c);

                    }
                    (true, true, true, true) |
                    (false, false, false, false) => {
                        // continue
                    }
                }
            }
        }

        self
    }

    fn add_to_polygon_with_point_of_left_cell(&mut self, p: Point, r: i32, c: i32) -> Option<&mut Polygon> {
        let left_cell = self.esri.get_upper_left_point_of_left_cell(r, c);

        match self.search_polygon_with_ending_point(left_cell) {
            Some(poly) => { Some(poly.add_point(p)) }
            None => { None }
        }
    }

    fn extract_polygon_with_ending_point(&mut self, p: Point) -> &mut Polygon {
        let v = {
            self.polygons.iter_mut()
                .filter(|poly| poly.ends_with_point(&p))
                .next().unwrap()
        };
        self.polygons.retain(|&v| !v.ends_with_point(&p));
        v
    }

    fn search_polygon_with_ending_point(&mut self, p: Point) -> Option<&mut Polygon> {
        self.polygons.iter_mut()
            .filter(|poly|poly.ends_with_point(&p))
            .next()
    }

    fn add_polygon(&mut self, p: Point) -> &EsriPolygons {
        let mut points = Vec::new();
        points.push(p);
        self.polygons.push(Polygon::new(points));
        self
    }


}

impl std::fmt::Debug for EsriPolygons {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[ {:?} ]", self.polygons)
    }
}