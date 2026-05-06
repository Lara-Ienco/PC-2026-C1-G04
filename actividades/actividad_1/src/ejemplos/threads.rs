use std::thread;

/// Representa una tarea con un identificador y una descripción.
pub struct Tarea {
    /// Identificador único de la tarea.
    id: u32,
    /// Descripción detallada de la tarea.
    descripcion: String,
}

impl Tarea {
    /// Método para ejecutar la tarea.
    fn ejecutar(&self) {
        println!("Procesando la tarea '{}'", self.descripcion);
    }
}

pub fn main() {
    let tarea_hilo = Tarea::nueva(11, String::from("Tarea procesada en segundo plano"));
    let handle = thread::spawn(move || {
        tarea_hilo.ejecutar();
    });
    println!("Hilo principal: esperando que el hilo secundario termine...");
    println!("{:?}", handle.join());
}