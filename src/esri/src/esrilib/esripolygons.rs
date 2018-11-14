use esrilib::point::Point;
use esrilib::polygon::Polygon;
use esrilib::esrifile::EsriFile;

pub struct EsriPolygons {
    esri: EsriFile,
    polygons: Vec<Polygon>,
    threshold: i32,
}

pub fn NewEsriPolygons(esri: EsriFile, polygons: Vec<Polygon>) -> EsriPolygons {
    EsriPolygons{
        esri,
        polygons,
        threshold: 1
    }
}

impl EsriPolygons {
    pub fn to_polygons(&mut self) -> &EsriPolygons {


        let mut points = Vec::new();
        for _r in 0..self.esri.n_rows {
            for _c in 0..self.esri.n_cols {

                let p = self.esri.get_upper_left_point(_r, _c);

                match self.esri.get_esri_data_value(_r, _c) >= self.threshold {
                    false => match self.esri.get_left_data_value(_r, _c) >= self.threshold {
                        false => match self.esri.get_upper_data_value(_r, _c) >= self.threshold {
                            false => match self.esri.get_upper_left_data_value(_r, _c) >= self.threshold {
                                false => {}
                                true  => {}
                            }
                            true  => match self.esri.get_upper_left_data_value(_r, _c) >= self.threshold {
                                false => {}
                                true  => {}
                            }
                        }
                        true => match self.esri.get_upper_data_value(_r, _c) >= self.threshold {
                            false => match self.esri.get_upper_left_data_value(_r, _c) >= self.threshold {
                                false => {}
                                true  => {}
                            }
                            true  => match self.esri.get_upper_left_data_value(_r, _c) >= self.threshold {
                                false => {}
                                true  => {}
                            }
                        }
                    }
                    true => match self.esri.get_left_data_value(_r, _c) >= self.threshold {
                        false => match self.esri.get_upper_data_value(_r, _c) >= self.threshold {
                            false => match self.esri.get_upper_left_data_value(_r, _c) >= self.threshold {
                                false => {

                                }
                                true  => {}
                            }
                            true  => match self.esri.get_upper_left_data_value(_r, _c) >= self.threshold {
                                false => {}
                                true  => {}
                            }
                        }
                        true => match self.esri.get_upper_data_value(_r, _c) >= self.threshold {
                            false => match self.esri.get_upper_left_data_value(_r, _c) >= self.threshold {
                                false => {}
                                true  => {}
                            }
                            true  => match self.esri.get_upper_left_data_value(_r, _c) >= self.threshold {
                                false => {}
                                true  => {}
                            }
                        }
                    }
                }




                points.push(p);
            }
        }
        /*
        points.push(Point{ x: 3.0, y: 4.0 } );
        points.push(Point{ x: 2.0, y: 1.0 } );
        points.push(Point{ x: 5.0, y: 9.0 } );
*/

        self.polygons.push(
            Polygon::new(points)
        );

        self
    }

    fn search_matching_polygon(&self, p: Point) -> Option<&Polygon> {
        self.polygons.iter()
            .filter(|poly|poly.contains_point(&p))
            .next()
    }


}

impl std::fmt::Debug for EsriPolygons {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[ {:?} ]", self.polygons)
    }
}