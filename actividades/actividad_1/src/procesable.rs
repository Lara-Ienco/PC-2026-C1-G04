/// Define un comportamiento común para las tareas que pueden ser procesadas.
pub trait Procesable {
    fn ejecutar(&mut self); // &mut self porque el método ejecutar modificará el estado de la tarea
}
