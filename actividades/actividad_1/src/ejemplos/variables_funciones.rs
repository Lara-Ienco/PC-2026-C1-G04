/// Inicializa y actualiza un contador de tareas procesadas, demostrando el uso de variables mutables e inmutables.
pub fn gestionar_contador_tareas() {
    // Variable inmutable --> no se puede modificar después de su inicialización
    let limite_tareas: u32 = CANTIDAD_MAXIMA_TAREAS;
    // Variable mutable --> se puede modificar después de su inicialización
    let mut tareas_procesadas: u32 = 0;
    println!("Capacidad del sistema: {} tareas", limite_tareas);
    tareas_procesadas += 1; // simulo el procesamiento de 1 tarea
    println!("Tareas procesadas actualmente: {}", tareas_procesadas);
}

pub fn main() {
    gestionar_contador_tareas();
}