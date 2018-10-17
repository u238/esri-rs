use esrilib::*;

impl Esri {
    pub fn new() -> Esri {
        Esri {
            n_cols: 5,
            n_rows: 5,
            xll_center: 2.5,
            yll_center: 2.5,
            x_origin: 0.0,
            y_origin: 0.0,
            cell_size: 1,
            no_data_value: -1,
            data: EsriData {
                values: vec![
                    0, 1, 0, 0, 1,
                    1, 1, 0, 1, 0,
                    1, 1, 1, 1, 1,
                    0, 0, 1, 0, 1,
                    1, 1, 1, 1, 0,
                ]
            }
        }
    }
    pub fn new_from_file() -> Esri {
        let mut esri = Esri::new();
        esri.calculate_origin();
        esri
    }

    fn calculate_origin(&mut self) -> &mut Esri {
        self.x_origin = self.xll_center - ((self.n_cols as f32) / 2.0);
        self.y_origin = self.yll_center + ((self.n_rows as f32) / 2.0);
        self
    }

    pub fn to_string(&self) -> String {
        format!("[ {}, {} ]", self.n_cols, self.n_rows)
    }
}
