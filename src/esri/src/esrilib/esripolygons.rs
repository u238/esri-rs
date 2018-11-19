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

                match self.esri.get_esri_data_value(_r, _c) >= self.threshold {
                    // current cell is 0
                    false => match self.esri.get_left_data_value(_r, _c) >= self.threshold {
                        // left cell is 0
                        false => match self.esri.get_upper_data_value(_r, _c) >= self.threshold {
                            // upper cell is 0
                            false => match self.esri.get_upper_left_data_value(_r, _c) >= self.threshold {
                                // upper left cell is 0
                                // 0 0
                                // 0 0
                                false => {
                                    // continue
                                }
                                // upper left cell is 1
                                // 1 0
                                // 0 0
                                true  => {}
                            }
                            // upper cell is 1
                            true  => match self.esri.get_upper_left_data_value(_r, _c) >= self.threshold {
                                // upper left cell is 0
                                // 0 1
                                // 0 0
                                false => {}
                                // upper left cell is 1
                                // 1 1
                                // 0 0
                                true  => {}
                            }
                        }
                        // left cell is 1
                        true => match self.esri.get_upper_data_value(_r, _c) >= self.threshold {
                            // upper cell is 0
                            false => match self.esri.get_upper_left_data_value(_r, _c) >= self.threshold {
                                // upper left cell is 0
                                // 0 0
                                // 1 0
                                false => {}
                                // upper left cell is 1
                                // 1 0
                                // 1 0
                                true  => {}
                            }
                            // upper cell is 1
                            true  => match self.esri.get_upper_left_data_value(_r, _c) >= self.threshold {
                                // upper left cell is 0
                                // 0 1
                                // 1 0
                                false => {}
                                // upper left cell is 1
                                // 1 1
                                // 1 0
                                true  => {}
                            }
                        }
                    }
                    // current cell is 1
                    true => match self.esri.get_left_data_value(_r, _c) >= self.threshold {
                        // left cell is 0
                        false => match self.esri.get_upper_data_value(_r, _c) >= self.threshold {
                            // upper cell is 0
                            false => match self.esri.get_upper_left_data_value(_r, _c) >= self.threshold {
                                // upper left cell is 0
                                // 0 0
                                // 0 1
                                false => {

                                }
                                // upper left cell is 1
                                // 1 0
                                // 0 1
                                true  => {}
                            }
                            // upper cell is 1
                            true  => match self.esri.get_upper_left_data_value(_r, _c) >= self.threshold {
                                // upper left cell is 0
                                // 0 1
                                // 0 1
                                false => {}
                                // upper left cell is 1
                                // 1 1
                                // 0 1
                                true  => {}
                            }
                        }
                        // left cell is 1
                        true => match self.esri.get_upper_data_value(_r, _c) >= self.threshold {
                            // upper cell is 0
                            false => match self.esri.get_upper_left_data_value(_r, _c) >= self.threshold {
                                // upper left cell is 0
                                // 0 0
                                // 1 1
                                false => {}
                                // upper left cell is 1
                                // 1 0
                                // 1 1
                                true  => {}
                            }
                            // upper cell is 1
                            true  => match self.esri.get_upper_left_data_value(_r, _c) >= self.threshold {
                                // upper left cell is 0
                                // 0 1
                                // 1 1
                                false => {}
                                // upper left cell is 1
                                // 1 1
                                // 1 1
                                true  => {}
                            }
                        }
                    }
                }
            }
        }

        self
    }

    fn add_polygon(&mut self, p: Point) -> &EsriPolygons {
        let mut points = Vec::new();
        points.push(p);
        self.polygons.push(Polygon::new(points));
        self
    }

    fn search_polygon_with_ending_point(&self, p: Point) -> Option<&Polygon> {
        self.polygons.iter()
            .filter(|poly|poly.ends_with_point(&p))
            .next()
    }


}

impl std::fmt::Debug for EsriPolygons {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[ {:?} ]", self.polygons)
    }
}