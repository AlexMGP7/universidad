use objects::{products::Producto, cart::Carrito, cashiers::Cajera};
use std::{fs};//Libreria local para leer el archivo .txt entrante
mod objects;
fn main(){

    //Alexander Gonzalez / 30.230.460
    println!("Leyendo el archivo de entrada {}","QSM.txt");//Se lee el archivo
    let contenido = fs::read_to_string("QSM.txt").expect("Something went wrong reading the file");

    let mut arr_text: Vec<&str> =contenido.lines().collect();//Se divide en lineas
    let mut arr_text_iterable=arr_text.iter_mut();//Se convierte en iterable

    let mut cajeras=Vec::new();//Se crea el vector de cajeras

    println!("");
    println!("-----Procesando carritos en espera predefinidos en una cajero-----");
    println!("");

    for _ in 0..3 {
        let linea_texto = arr_text_iterable.next().unwrap();//Se leen las cajeras
        let vec_contenedor = linea_texto.trim().split(" ").collect::<Vec<&str>>();//Se divide por espacios

        let nom_cajera = vec_contenedor[0].to_string();
        let dinero_recaudado = vec_contenedor[1].parse().unwrap();
        let carritos_en_espera = vec_contenedor[2].parse().unwrap();

        let cajera = Cajera::cajera_nueva(nom_cajera, dinero_recaudado, carritos_en_espera);//Se crea la cajera

        cajeras.push(cajera);
    }

    for cajera in cajeras.iter_mut() {
        for _ in 0..cajera.carritos_en_espera{
            let linea_texto = arr_text_iterable.next().unwrap();

            let mut carro = Carrito::carrito_nuevo();

            for producto in linea_texto.trim().split(",") {
                let vec_contenedor = producto.trim().split(" ").collect::<Vec<&str>>();

                let producto = Producto::producto_nuevo(vec_contenedor[1].parse().unwrap(),vec_contenedor[0].to_string());
                carro.push(producto);
            }
        cajera.encolar(carro);
        }
    }
    for cajera in cajeras.iter_mut() {
        //println!("[{:?}]",cajera);
        while cajera.carritos_en_espera_actuales() > 0 {
            let carro_actual=cajera.frente().unwrap().clone();
            cajera.sumar_dinero(carro_actual);
            //println!("{}",cajera.dinero_recaudado);
        }
    }

    println!("Mostrando todas las cajeras por el momento: ");
    println!("{:?}",cajeras);

    println!("");
    println!("-----Procesando carritos sobrantes en el archivo de texto-----");
    println!("");

    for cajera in cajeras.iter_mut() {
        for _ in 0..cajera.carritos_en_espera{
            let linea_texto = arr_text_iterable.next().unwrap();

            let mut carro = Carrito::carrito_nuevo();

            for producto in linea_texto.trim().split(",") {
                let vec_contenedor = producto.trim().split(" ").collect::<Vec<&str>>();

                let producto_a_insertar = Producto::producto_nuevo(vec_contenedor[1].parse().unwrap(),vec_contenedor[0].to_string());
                carro.push(producto_a_insertar);
            }
        cajera.encolar(carro);
        }
    }

    for cajera in cajeras.iter_mut() {
        //println!("[{:?}]",cajera);
        while cajera.carritos_en_espera_actuales() > 0 {
            let carro_actual=cajera.frente().unwrap().clone();
            cajera.sumar_dinero(carro_actual);
            //println!("{}",cajera.dinero_recaudado);
        }
    }

    println!("Mostrando todas las cajeras: ");
    println!("{:?}",cajeras);

    let mut dinero_recaudado:f32=0.00;

    for x in 0..cajeras.len(){
        dinero_recaudado += cajeras[x].dinero_recaudado;
    }

    println!("");
    println!("El dinero total recaudado por el supermercado: {}$",(dinero_recaudado*100.00).round()/100.00);
    println!("Se despedira al cajero: {} por no haber recaudado dinero",cajeras[2].nom_cajera)

    //141.28

}