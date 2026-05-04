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

    /// Verifica si el id recibido es el de la tarea.
    pub fn coincide_con_id(&self, id: u32) -> bool {
        self.id == id
    }
}

/// Busca una tarea por ID.
/// Devuelve 'Option<&Tarea>' (prestado).
/// Si no existe, devuelve 'None'
pub fn buscar_por_id(tareas: Vec<Tareas>, id: u32) -> Option<&Tarea> {
    tareas.iter().find(|tarea| tarea.coincide_con_id(id))
}

pub fn main() {
    let tareas: Vec<Tareas> = vec![];

    for i in ..3 {
        let tarea = Tarea::nueva(i, format!("Tarea {i}"));
    }
    println!("{:?}", buscar_por_id(tareas, 1));
    println!("{:?}", buscar_por_id(tareas, 4));
}