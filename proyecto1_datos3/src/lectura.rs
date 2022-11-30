use std::{fs};

//use crate::cola;
//use std::env;

#[allow(dead_code)]
pub fn leer(){
    println!("Leyendo el archivo de entrada {}","QSM.txt");
    let contenido = fs::read_to_string("QSM.txt").unwrap();
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
    //cola::Queue::cola_nueva(arr_datos_cajero1[0].to_string(), arr_datos_cajero1[1].to_string(), arr_datos_cajero1[2].to_string());

    let datos_cajeros2=cajeros[1].split_whitespace();
    let mut arr_datos_cajero2:Vec<&str>=Vec::new();
    for x in datos_cajeros2{
        arr_datos_cajero2.push(x);
    }
    //cola::Queue::cola_nueva(arr_datos_cajero2[0].to_string(), arr_datos_cajero2[1].to_string(), arr_datos_cajero2[2].to_string());

    let datos_cajeros3=cajeros[2].split_whitespace();
    let mut arr_datos_cajero3:Vec<&str>=Vec::new();
    for x in datos_cajeros3{
        arr_datos_cajero3.push(x);
    }
    //cola::Queue::cola_nueva(arr_datos_cajero3[0].to_string(), arr_datos_cajero3[1].to_string(), arr_datos_cajero3[2].to_string());



    let mut carritos=Vec::new();
    for x in 3..arr_text.len(){
        carritos.push(arr_text[x]);
        println!("[{}]",carritos[x-3]);
    }



}