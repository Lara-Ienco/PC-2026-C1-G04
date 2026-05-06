/// Representa una tarea con un identificador, una descripción y un estado.
pub struct Tarea {
    /// Identificador único de la tarea.
    id: u32,
    /// Descripción detallada de la tarea.
    descripcion: String,
    /// Estado actual de la tarea.
    estado: EstadoTarea,
}

/// Representa los posibles estados de una tarea dentro del sistema.
pub enum EstadoTarea {
    /// La tarea está pendiente de ser realizada.
    Pendiente,
    /// La tarea está actualmente en progreso.
    EnProgreso,
    /// La tarea ha sido completada.
    Completada,
    /// La tarea ha fallado o no se pudo completar.
    Fallida,
}

/// Define un comportamiento común para las tareas que pueden ser procesadas.
pub trait Procesable {
    fn ejecutar(&mut self); // &mut self porque el método ejecutar modificará el estado de la tarea
}

impl Procesable for Tarea {
    /// Método para ejecutar la tarea, cambiando su estado a EnProgreso y luego a Completada
    fn ejecutar(&mut self) {
        self.estado = EstadoTarea::EnProgreso;
        println!("Procesando la tarea '{}'", self.descripcion);
        self.estado = EstadoTarea::Completada;
    }
}

pub fn main() {
    let tarea = Tarea {
        id: 1,
        descripcion: String::from("Tarea 1"),
        estado: EstadoTarea::Pendiente,
    };
    tarea.ejecutar();
}