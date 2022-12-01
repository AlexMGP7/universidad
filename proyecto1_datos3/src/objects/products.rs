#[derive(Debug)]
#[derive(Clone)]
pub struct Producto{        //Se crea la estructura producto (Equivalente a un nodo hablando en un caso general)
    pub producto:String,  //Nombre del producto
    pub precio:f32        //Precio del producto
}

impl Producto {         //Las implementaciones del producto
    #[allow(dead_code)]
    pub fn producto_nuevo(precio:f32, producto:String)->Self{
        Producto { //Funcion para crear un producto nuevo
            producto:producto, precio:precio,   //Las variables de las estructuras valdran lo que se pase aca por parametros
        }
    }
}