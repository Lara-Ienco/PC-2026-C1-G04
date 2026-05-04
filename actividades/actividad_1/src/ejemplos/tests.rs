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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn nueva_tarea_comienza_en_estado_pendiente() {
        let tarea = Tarea::nueva(1, String::from("Test"));
        assert!(matches!(tarea.estado, EstadoTarea::Pendiente));
    }
}