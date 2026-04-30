/// Constante que define la cantidad máxima de tareas permitidas
pub const CANTIDAD_MAXIMA_TAREAS: u32 = 10;

/// Representa los posibles estados de una tarea dentro del sistema.
#[derive(Debug)]
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


/// Representa una tarea con un identificador, una descripción y un estado.
#[derive(Debug)]
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

    // VER --> la consigna dice que tiene que recibir ese estado y mostrarlo, pero eso, si yo lo llamo desde main no deberia permitirse pq el estado es privado !! Osea romperia encapsulamiento.
    // entonces esta de aca abajo cumple con eso, pero no se si deberiamos usarla en el main justamente por eso.
    /// Imprime el estado de la tarea dado un estado específico.
    pub fn mostrar_estado(estado: &EstadoTarea) {
        match estado {
            EstadoTarea::Pendiente => println!("Pendiente"),
            EstadoTarea::EnProgreso => println!("En progreso"),
            EstadoTarea::Completada => println!("Completada"),
            EstadoTarea::Fallida => println!("Fallida"),
        }
    }
}

/// Define un comportamiento común para las tareas que pueden ser procesadas.
pub trait Procesable {
    fn ejecutar(&mut self); // &mut self porque el método ejecutar modificará el estado de la tarea
}

impl Procesable for Tarea {
    /// Método para ejecutar la tarea, cambiando su estado a EnProgreso y luego a Completada
    fn ejecutar(&mut self) {
        self.iniciar(); // cambio el estado a EnProgreso
        println!("Procesando la tarea '{}'", self.descripcion);
        // aca capaz hay q agregar logica para determinar si la tarea se completa o falla
        self.completar(); // cambio el estado a Completada
    }
}

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