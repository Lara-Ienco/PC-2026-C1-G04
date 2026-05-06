/// Inicializa y actualiza un contador de tareas procesadas, demostrando el uso de variables mutables e inmutables.
pub fn gestionar_contador_tareas() {
    let limite_tareas: u32 = 10; // Variable inmutable
    let mut tareas_procesadas: u32 = 0; // Variable mutable
    println!("Capacidad del sistema: {} tareas", limite_tareas);
    tareas_procesadas += 1; // Simulamos procesamiento de una tarea
    println!("Tareas procesadas actualmente: {}", tareas_procesadas);
}

fn main(){
    gestionar_contador_tareas();
}
