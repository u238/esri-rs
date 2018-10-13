pub mod parser;
pub mod converter;
pub mod polygon;
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

pub struct Esri {
    n_cols: i32,
    n_rows: i32,
    xll_center: f32,
    yll_center: f32,
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

pub struct Polygon {
    points: Vec<Point>,
}

pub struct Point {
    x: i32,
    y: i32,
}