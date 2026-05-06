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

pub fn main() {
    let estado = EstadoTarea::Pendiente;
    mostrar_estado(&estado);
}