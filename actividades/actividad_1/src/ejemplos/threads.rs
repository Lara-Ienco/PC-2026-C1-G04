use std::thread;

/// Representa una tarea con un identificador, una descripción y un estado.
pub struct Tarea {
    /// Identificador único de la tarea.
    id: u32,
    /// Descripción detallada de la tarea.
    descripcion: String,
    /// Estado actual de la tarea, representado por el enum EstadoTarea.
    estado: EstadoTarea,
}

/// Define un comportamiento común para las tareas que pueden ser procesadas.
pub trait Procesable {
    fn ejecutar(&mut self); // &mut self porque el método ejecutar modificará el estado de la tarea
}

impl Procesable for Tarea {
    /// Método para ejecutar la tarea, cambiando su estado a EnProgreso y luego a Completada
    fn ejecutar(&mut self) {
        self.iniciar(); // cambio el estado a EnProgreso
        println!("Procesando la tarea '{}'", self.descripcion);
        self.completar(); // cambio el estado a Completada
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