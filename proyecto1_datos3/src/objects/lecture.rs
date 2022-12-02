use std::{fs};

use crate::objects::cashiers;

#[allow(dead_code)]
pub fn leerCajera1() -> cashiers::Cajera{
    println!("Leyendo el archivo de entrada {}","QSM.txt");
    let contenido = fs::read_to_string("QSM.txt").expect("Something went wrong reading the file");

    let arr_text: Vec<&str> =contenido.lines().collect();

    let mut cajeros:Vec<&str>= Vec::new();
    for x in 0..3{
        cajeros.push(arr_text[x]);
        println!("[{}]",cajeros[x]);
    }

    let data=cajeros[0].trim().split(" ").collect::<Vec<&str>>();
    let mut arr_datos_cajero:Vec<&str>=Vec::new();
    for x in data{
        arr_datos_cajero.push(x);
    }

    return cashiers::Cajera::cajera_nueva(arr_datos_cajero[0].to_string(), arr_datos_cajero[1].parse::<f32>().unwrap());

    //println!("{:?}",arr_datos_cajero);

    /* let datos_cajeros2=cajeros[1].split_whitespace();
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
    } */

}