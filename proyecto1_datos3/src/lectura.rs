use std::fs;
//use std::env;

#[allow(dead_code)]
pub fn leer(){
    println!("Leyendo el archivo de entrada {}","QSM.txt");
    let contenido = fs::read_to_string("QSM.txt").unwrap();
    println!("{}",contenido);
}