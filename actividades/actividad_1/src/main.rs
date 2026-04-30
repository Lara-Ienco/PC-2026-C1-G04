
use actividad1::{Tarea,Procesable,gestionar_contador_tareas};

fn main() {
    gestionar_contador_tareas();

    // creo la nueva tarea
    let mut tarea1 = Tarea::nueva(1, String::from("Realizar Actividad 1 de Programación Concurrente"));
    
    tarea1.imprimir_estado();
    
    tarea1.ejecutar();
    
    tarea1.imprimir_estado();
}