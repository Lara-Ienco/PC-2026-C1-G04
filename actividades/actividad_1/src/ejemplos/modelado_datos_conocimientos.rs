/// Representa una tarea con un identificador, una descripción y un estado.
pub struct Tarea {
    /// Identificador único de la tarea.
    id: u32,
    /// Descripción detallada de la tarea.
    descripcion: String,
    /// Estado actual de la tarea, representado por el enum EstadoTarea.
    estado: EstadoTarea,
}

impl Tarea {
    /// Método para crear una nueva tarea con un identificador y una descripción, y establecer el estado inicial como Pendiente
    pub fn nueva(id: u32, descripcion: String) -> Self {
        Tarea {
            id,
            descripcion,
            estado: EstadoTarea::Pendiente, // la tarea comienza por defecto en estado Pendiente
        }
    }

    /// Marca la tarea como en progreso.
    pub fn iniciar(&mut self) {
        self.estado = EstadoTarea::EnProgreso;
    }
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

/// Imprime el estado de la tarea dado un estado específico.
pub fn mostrar_estado(estado: &EstadoTarea) {
    match estado {
        EstadoTarea::Pendiente => println!("Pendiente"),
        EstadoTarea::EnProgreso => println!("En progreso"),
        EstadoTarea::Completada => println!("Completada"),
        EstadoTarea::Fallida => println!("Fallida"),
    }
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
    mostrar_estado(&EstadoTarea::Pendiente);

    let id = 30;
    let descripcion = String::from("Actividad 1");
    let tarea = Tarea::nueva(id, descripcion);
    tarea.ejecutar();
}