use crate::estado_tarea::EstadoTarea;
use crate::procesable::Procesable;
use crate::tarea::Tarea;

/// Constante que define la cantidad máxima de tareas permitidas
pub const CANTIDAD_MAXIMA_TAREAS: u32 = 10;

/// Representa una gestor de tareas con un vector de tareas
#[derive(Debug)]
pub struct GestorDeTareas {
    /// Vector de Tareas
    tareas: Vec<Tarea>,
}

impl GestorDeTareas {
    /// Crea un gestor de tareas vacío
    pub fn nuevo() -> Self {
        GestorDeTareas { tareas: Vec::new() }
    }

    /// Agrega una tarea al gestor
    pub fn agregar(&mut self, tarea: Tarea) {
        self.tareas.push(tarea)
    }

    /// Busca una tarea por ID.
    /// Devuelve 'Option<&Tarea>' (prestado).
    /// Si no existe, devuelve 'None'
    pub fn buscar(&self, id: u32) -> Option<&Tarea> {
        self.tareas.iter().find(|tarea| tarea.coincide_con_id(id))
    }

    /// Busca una tarea por ID.
    /// Devuelve una copia 'Option<Tarea>'.
    pub fn obtener_por_id(&self, id: u32) -> Option<Tarea> {
        self.buscar(id).cloned() //tuve que modificar Tarea para que se pueda clonar.
    }

    /// Busca una tarea por ID y la elimina del gestor.
    /// Devuelve 'Option<Tarea>' (toma ownership de la tarea removida).
    pub fn quitar(&mut self, id: u32) -> Option<Tarea> {
        let pos = self
            .tareas
            .iter()
            .position(|tarea| tarea.coincide_con_id(id));
        pos.map(|i| self.tareas.remove(i))
    }

    /// Procesa una tarea por ID y devuelve `Result<(), String>`.
    /// - Si la tarea existe y se puede procesar (estado Pendiente), ejecuta y devuelve Ok(()).
    /// - Si la tarea existe pero ya está en otro estado, devuelve Err con un mensaje.
    /// - Si no existe, devuelve Err.
    pub fn procesar_por_id(&mut self, id: u32) -> Result<(), String> {
        // Buscamos una referencia mutable a la tarea
        let tarea = self
            .tareas
            .iter_mut()
            .find(|tarea| tarea.coincide_con_id(id));
        match tarea {
            Some(tarea) => {
                // Verificamos el estado actual.
                // En este caso convendría que el objeto Tarea tenga un método que
                // permita obtener su estado y no acceder directamente como se hace a continuación
                match tarea.obtener_estado() {
                    EstadoTarea::Pendiente => {
                        tarea.ejecutar();
                        Ok(())
                    }
                    EstadoTarea::EnProgreso => Err("La tarea ya está en progreso".to_string()),
                    EstadoTarea::Completada => Err("La tarea ya fue completada".to_string()),
                    EstadoTarea::Fallida => Err("La tarea falló anteriormente".to_string()),
                }
            }
            None => Err(format!("No se encontró la tarea con ID {}", id)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn procesar_tarea_pendiente_devuelve_ok() {
        let mut gestor = GestorDeTareas::nuevo();
        gestor.agregar(Tarea::nueva(1, String::from("Test")));
        assert!(gestor.procesar_por_id(1).is_ok());
    }

    #[test]
    fn procesar_tarea_ya_completada_devuelve_err() {
        let mut gestor = GestorDeTareas::nuevo();
        gestor.agregar(Tarea::nueva(1, String::from("Test")));
        gestor.procesar_por_id(1).unwrap();
        assert!(gestor.procesar_por_id(1).is_err());
    }

    #[test]
    fn procesar_tarea_inexistente_devuelve_err() {
        let mut gestor = GestorDeTareas::nuevo();
        assert!(gestor.procesar_por_id(99).is_err());
    }
}
