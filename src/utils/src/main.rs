extern crate esrilib;

use esrilib::esrilib::esrifile::EsriFile;
use esrilib::esrilib::polygon::Polygon;
use esrilib::esrilib::esripolygons;


fn main() {

    let esri = EsriFile::new_from_file();

    let polygons : Vec<Polygon> = Vec::new();
    let mut esri_polygons = esripolygons::NewEsriPolygons(esri, polygons);

    esri_polygons.to_polygons();

    println!("Hello {:?}!", esri_polygons);
}
