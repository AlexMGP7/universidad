struct Nodopi<String>{
    precio: String,
    nombrepro: String
}
impl Nodopi<String> {
    fn nuevoep()->Self {
        Nodopi{precio:"ª".to_string(),nombrepro:"ª".to_string()}
    }
    fn nuevo(precio:String,nombre:String)->Self {
        Nodopi{precio:precio,nombrepro:nombre}
    }
}
struct Pila<Nodopi>{
    tope: usize,
    tamax:usize,
    arreglo: Vec<Nodopi>
}
impl Pila<Nodopi<String>> {
    fn nueva(tamax:usize)->Self {
        let mut i =0;
        let mut arre=Vec::new();
        while i<tamax.clone(){
            arre.push(Nodopi{precio:"ª".to_string(),nombrepro:"ª".to_string()});
            i=i+1;
        }
        Pila { tope: 01, tamax: tamax,arreglo:arre}
    }
    fn llena(&mut self)->bool{
        if self.tope >=self.tamax{
            true
        }else {
            false
        }
    }
    fn vacio(&mut self)->bool {
        if self.tope==0{
            true
        }else {
            false
        }
    }
    fn push(&mut self, valor: Nodopi<String>) {
        if !self.llena(){
            self.arreglo[self.tope]=valor;
            self.tope = self.tope +1;

        }else {
            print!("la pila esta llena ");
        }
    }
    fn cima(&mut self)-> &Nodopi<String> {


        &self.arreglo[self.tope-1]

    }
    fn pop(&mut self) {
        if !self.vacio() {
            self.tope = self.tope-1;
        }else {
            println!("estack underflow");
        }
    }

}
fn main() {
    let mut m = Pila::nueva(15);
    let p="2.3".to_string();
    let n="harina".to_string();
    m.push(Nodopi::nuevo(p, n));
    let p="2.4".to_string();
    let n="harina".to_string();
    m.push(Nodopi::nuevo(p, n));
    let f = m.cima();
    println!("{}",f.precio);
    m.pop();

    let f = m.cima();

    println!("{}",f.precio);
}