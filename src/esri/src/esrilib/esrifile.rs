use esrilib::point::Point;

/*
ESRI format:


Parameter               Description	                                                                Requirements
NCOLS                   Number of cell columns.	                                                    Integer greater than 0.
NROWS	                Number of cell rows.	                                                    Integer greater than 0.
XLLCENTER or XLLCORNER	X coordinate of the origin (by center or lower left corner of the cell).	Match with Y coordinate type.
YLLCENTER or YLLCORNER	Y coordinate of the origin (by center or lower left corner of the cell).	Match with X coordinate type.
CELLSIZE	            Cell size.	                                                                Greater than 0.
NODATA_VALUE	        The input values to be NoData in the output raster.	                        Optional. Default is -9999.
*/

pub struct EsriFile {
    pub n_cols: i32,
    pub n_rows: i32,
    xll_center: f32,
    yll_center: f32,
    pub x_origin: f32,
    pub y_origin: f32,
    cell_size: i32,
    no_data_value: i32,
    data: EsriData,
}

/*
The data component of the ESRI ASCII raster follows the header information.
Cell values should be delimited by spaces.
No carriage returns are necessary at the end of each row in the raster. The number of columns in the header determines when a new row begins.
Row 1 of the data is at the top of the raster, row 2 is just under row 1, and so on.
*/

pub struct EsriData {
    values: Vec<i32>,
}

impl EsriFile {
    pub fn new() -> EsriFile {
        EsriFile {
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
    pub fn new_from_file() -> EsriFile {
        let mut esri = EsriFile::new();
        esri.calculate_origin();
        esri
    }

    fn calculate_origin(&mut self) -> &mut EsriFile {
        self.x_origin = self.xll_center - ((self.n_cols as f32) / 2.0);
        self.y_origin = self.yll_center + ((self.n_rows as f32) / 2.0);
        self
    }

    pub fn get_upper_left_point(&self, r: i32, c: i32) -> Point {
        Point {
            x: self.x_origin + (r * self.cell_size),
            y: self.y_origin + (c * self.cell_size),
        }
    }

    fn esri_data_coords_valid(&self, r: i32, c: i32) -> bool {
        r >= 0 && r < self.n_rows && c >= 0 && c < self.n_cols
    }

    pub fn get_esri_data_value(&self, r: i32, c: i32) -> i32 {
        match self.esri_data_coords_valid(r, c) {
            true => self.data.values[(r * self.n_cols + c) as usize],
            false => 0
        }
    }

    pub fn get_left_data_value(&self, r: i32, c: i32) -> i32 {
        self.get_esri_data_value(r, c-1)
    }

    pub fn get_upper_left_data_value(&self, r: i32, c: i32) -> i32 {
        self.get_esri_data_value(r-1, c-1)
    }

    pub fn get_upper_data_value(&self, r: i32, c: i32) -> i32 {
        self.get_esri_data_value(r-1, c)
    }



    pub fn to_string(&self) -> String {
        format!("[ {}, {} ]", self.n_cols, self.n_rows)
    }
}