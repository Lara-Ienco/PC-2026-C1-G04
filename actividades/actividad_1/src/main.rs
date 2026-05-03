
use actividad1::{Tarea, Procesable, gestionar_contador_tareas, GestorDeTareas};
use std::thread;

fn main() {
    gestionar_contador_tareas();

    // creo la nueva tarea
    let mut tarea1 = Tarea::nueva(1, String::from("Realizar Actividad 1 de Programación Concurrente"));

    tarea1.imprimir_estado();

    tarea1.ejecutar();

    tarea1.imprimir_estado();

    // Uso de Gestor de Tareas
    let mut gestor = GestorDeTareas::nuevo();
    let t1 = Tarea::nueva(1, String::from("Realizar Ejercicio 1 de la Actividad 1"));
    let t2 = Tarea::nueva(2, String::from("Realizar Ejercicio 2 de la Actividad 1"));
    gestor.agregar(t1);
    gestor.agregar(t2);

    // Obtener con Option
    match gestor.obtener_por_id(1) {
        Some(t) => println!("Tarea encontrada: {:?}", t),
        None => println!("Tarea no encontrada"),
    }

    // Procesamiento con Result
    match gestor.procesar_por_id(1) {
        Ok(()) => println!("Tarea 1 procesada con éxito"),
        Err(e) => println!("Error al procesar tarea 1: {}", e),
    }

    let tarea_demo = Tarea::nueva(10, String::from("Demo ownership"));
    tarea_demo.imprimir_estado();

    let tarea_hilo = Tarea::nueva(11, String::from("Tarea procesada en segundo plano"));
    let handle = thread::spawn(move || {
        let mut t = tarea_hilo;
        t.ejecutar();
    });
    println!("Hilo principal: esperando que el hilo secundario termine...");
    handle.join().unwrap();
}
