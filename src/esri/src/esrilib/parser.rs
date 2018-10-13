use esri::Esri;

impl Esri {
    pub fn newFromFile() -> Esri {
        Esri {
            nCols: 5,
            nRows: 5,
            XllCenter: 2.5,
            YllCenter: 2.5,
            CellSize: 1,
            NoDataValue: -1,
        }
    }
}