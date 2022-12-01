use std::{fs};

#[allow(dead_code)]
pub fn leer(){
    println!("Leyendo el archivo de entrada {}","QSM.txt");
    let contenido = fs::read_to_string("QSM.txt").expect("Something went wrong reading the file");

    let arr_text: Vec<&str> =contenido.lines().collect();

    let mut cajeros= Vec::new();
    for x in 0..3{
        cajeros.push(arr_text[x]);
        println!("[{}]",cajeros[x]);
    }
    let datos_cajeros1=cajeros[0].split_whitespace();
    let mut arr_datos_cajero1:Vec<&str>=Vec::new();
    for x in datos_cajeros1{
        arr_datos_cajero1.push(x);
    }

    let datos_cajeros2=cajeros[1].split_whitespace();
    let mut arr_datos_cajero2:Vec<&str>=Vec::new();
    for x in datos_cajeros2{
        arr_datos_cajero2.push(x);
    }

    let datos_cajeros3=cajeros[2].split_whitespace();
    let mut arr_datos_cajero3:Vec<&str>=Vec::new();
    for x in datos_cajeros3{
        arr_datos_cajero3.push(x);
    }

    let mut carritos=Vec::new();
    for x in 3..arr_text.len(){
        carritos.push(arr_text[x]);
        println!("[{}]",carritos[x-3]);
    }

}