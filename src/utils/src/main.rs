extern crate esrilib;

fn main() {

    let esri = esrilib::esrilib::Esri::new_from_file();
    println!("Hello {}!", esri.to_string());
}
