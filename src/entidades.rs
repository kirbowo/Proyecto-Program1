use crate::input::{Tecla, leer_tecla};
use std::time::Duration;
pub struct Juego {

}
impl Juego {
    pub fn new() -> Juego {
        Juego {
            
        }
    }
    pub fn actualizar(&self) {
     
    }
    pub fn dibujar(&self){
    
    } 
    pub fn procesar_tecla(&mut self, tecla: Tecla) {
        match tecla {
            Tecla::Q => {println!("Saliendo...");
            std::thread::sleep(Duration::from_secs(3));
            std::process::exit(0);},
            Tecla::Espacio => {println!("Disparo!");
            std::thread::sleep(Duration::from_secs(1));
            },
            Tecla::FlechaIzquierda => {println!("Izquierda")},
            Tecla::FlechaDerecha => {println!("Derecha")},
            _ => (),
        }
    }
}
 