/// Representa una tarea con un identificador, una descripción y un estado.
pub struct Tarea {
    /// Identificador único de la tarea.
    id: u32,
    /// Descripción detallada de la tarea.
    descripcion: String,
    /// Estado actual de la tarea.
    estado: String,
}

impl Tarea {
    /// Marca la tarea como en progreso.
    pub fn iniciar(&mut self) {
        self.estado = "En progreso";
    }
}

pub fn main() {
    let id = 30;
    let descripcion = String::from("Actividad 1");
    let estado = String::from("Pendiente");
    let tarea = Tarea {
        id,
        descripcion,
        estado,
    };
    
    println!("{:?}", tarea);
    tarea.iniciar();
    println!("{:?}", tarea);
}