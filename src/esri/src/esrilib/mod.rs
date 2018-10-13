pub mod parser;

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
    nCols: int,
    nRows: int,
    XllCenter: float,
    YllCenter: float,
    CellSize: int,
    NoDataValue: int,
}
