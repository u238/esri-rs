use esrilib::*;

impl Esri {
    pub fn new_from_file() -> Esri {
        Esri {
            n_cols: 5,
            n_rows: 5,
            xll_center: 2.5,
            yll_center: 2.5,
            cell_size: 1,
            no_data_value: -1,
            data: EsriData{
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

    pub fn to_string(&self) -> String {
        // TODO: how can I retorn format! directly?
        let mut buf = String::new();
        buf += &format!("[ {}, {} ]", self.n_cols, self.n_rows);
        buf
    }
}