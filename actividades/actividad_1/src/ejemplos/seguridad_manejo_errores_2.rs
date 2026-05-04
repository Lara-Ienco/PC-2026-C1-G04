use std::mem::type_info::Str;

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

    /// Obtiene el estado actual de la tarea.
    pub fn obtener_estado(&self) -> EstadoTarea {
        self.estado.clone()
    }

    /// Marca la tarea como en progreso.
    pub fn iniciar(&mut self) {
        self.estado = EstadoTarea::EnProgreso;
    }

    /// Marca la tarea como completada.
    pub fn completar(&mut self) {
        self.estado = EstadoTarea::Completada;
    }

    /// Marca la tarea como fallida.
    pub fn fallar(&mut self) {
        self.estado = EstadoTarea::Fallida;
    }

}

/// Procesa una tarea por ID y devuelve `Result<(), String>`.
/// - Si la tarea existe y se puede procesar (estado Pendiente), ejecuta y devuelve Ok(()).
/// - Si la tarea existe pero ya está en otro estado, devuelve Err con un mensaje.
/// - Si no existe, devuelve Err.
pub fn procesar_por_id(tareas: Vec<Tarea>, id: u32) -> Result<(), String> {
    // Buscamos una referencia mutable a la tarea
    let tarea = tareas.iter_mut().find(|tarea| tarea.coincide_con_id(id));

    match tarea {
        Some(tarea) => {
            // Verificamos el estado actual.
            // En este caso convendría que el objeto Tarea tenga un método que
            // permita obtener su estado y no acceder directamente como se hace a continuación
            match tarea.obtener_estado() {
                EstadoTarea::Pendiente => {
                    tarea.ejecutar();
                    Ok(())
                },
                EstadoTarea::EnProgreso => Err("La tarea ya está en progreso".to_string()),
                EstadoTarea::Completada => Err("La tarea ya fue completada".to_string()),
                EstadoTarea::Fallida => Err("La tarea falló anteriormente".to_string()),
            }
        }
        None => Err(format!("No se encontró la tarea con ID {}", id)),
    }
}

pub fn main() {
    let tareas: Vec<Tareas> = vec![];

    for i in ..4 {
        let tarea = Tarea::nueva(i, format!("Tarea {i}"));
    }
    tareas[2].iniciar();
    tareas[3].completar();
    tareas[4].fallar();

    println!("{:?}", procesar_por_id(tareas, 0));
    println!("{:?}", procesar_por_id(tareas, 1));
    println!("{:?}", procesar_por_id(tareas, 2));
    println!("{:?}", procesar_por_id(tareas, 3));
    println!("{:?}", procesar_por_id(tareas, 4));
}