use crate::{estado_tarea::EstadoTarea, procesable::Procesable};

/// Representa una tarea con un identificador, una descripción y un estado.
#[derive(Debug, Clone)]
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

    /// Marca la tarea como completada.
    pub fn completar(&mut self) {
        self.estado = EstadoTarea::Completada;
    }

    /// Marca la tarea como fallida.
    pub fn fallar(&mut self) {
        self.estado = EstadoTarea::Fallida;
    }

    /// Imprime el estado actual de la tarea.
    pub fn imprimir_estado(&self) {
        match self.estado {
            EstadoTarea::Pendiente => println!("Pendiente"),
            EstadoTarea::EnProgreso => println!("En progreso"),
            EstadoTarea::Completada => println!("Completada"),
            EstadoTarea::Fallida => println!("Fallida"),
        }
    }

    /// Obtiene el estado actual de la tarea.
    pub fn obtener_estado(&self) -> EstadoTarea {
        self.estado.clone()
    }

    /// Verifica si el id recibido es el de la tarea.
    pub fn coincide_con_id(&self, id: u32) -> bool {
        self.id == id
    }
}

impl Procesable for Tarea {
    /// Método para ejecutar la tarea, cambiando su estado a EnProgreso y luego a Completada
    fn ejecutar(&mut self) {
        self.iniciar(); // cambio el estado a EnProgreso
        println!("Procesando la tarea '{}'", self.descripcion);
        self.completar(); // cambio el estado a Completada
    }
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
