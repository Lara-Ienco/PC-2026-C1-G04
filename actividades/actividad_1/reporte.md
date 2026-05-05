### 1. Fundamentos y Memoria

#### <u>Variables y Funciones</u>

En Rust tenemos 2 formas de definir las variables:

- `let` para variables inmutables (**por defecto**), por lo tanto no se pueden modificar después de su inicialización.
- `let mut` para variables mutables que sí pueden ser modificadas después de su inicialización.

Una de las caracteristicas principales de Rust es que es el hecho de priorizar la **inmutabilidad** para mejorar principalmente todo lo que tenga que ver con la seguridad y rendimiento del programa ya que justamente al no permitir que las variables sean modificadas, se evitan muchos errores comunes relacionados con el estado mutable, por ejemplo:

- Evitar modificaciones accidentales de datos que pueden causar bugs
- Permitir una mejor performance del compilador al darle la certeza de que ciertos valores no cambiaran (inmutables)
- Codigo mas facil de entender al saber que las variables a lo largo de la ejecucion del programa no van a cambiar su valor

En el ejemplo *variables_funciones.rs* tomamos el siguiente extracto de codigo:

```rust
/// Inicializa y actualiza un contador de tareas procesadas, demostrando el uso de variables mutables e inmutables.
pub fn gestionar_contador_tareas() {
    let limite_tareas: u32 = CANTIDAD_MAXIMA_TAREAS; // Variable inmutable
    let mut tareas_procesadas: u32 = 0; // Variable mutable
    println!("Capacidad del sistema: {} tareas", limite_tareas);
    tareas_procesadas += 1; // Simulamos procesamiento de una tarea
    println!("Tareas procesadas actualmente: {}", tareas_procesadas);
}
```

Vemos que la variable `limite_tareas` es **inmutable** ya que representa un valor constante que no cambiara a lo largo de la ejecucion del programa, por lo tanto le da mayor seguridad al compilador permitiendo optimizaciones sobre la variable, mientras que `tareas_procesadas` es **mutable** porque su valor se va a ir actualizando a medida que se procesen las tareas permitiendo asi actualizaciones controladas y tambien seguras ya que el compilador se asegura de que solo se modifique a traves de operaciones explicitas (como el `+=`) y no de forma accidental.

Otro de los aspectos fundamentales de la inmutabilidad de Rust es que es fundamental para la **CONCURRENCIA** al no permitir justamente que varias partes del programa modifiquen el mismo estado al mismo tiempo evitando las **Race Conditions** por tanto Rust requiere que los datos compartidos por varios hilos sean inmutables o esten protegidos mediante `Mutex` o `Locks` por ejemplo.

Para ejemplificar tomamos el siguiente extracto de codigo en el apartado de ejemplos `threads.rs`:

```rust
pub fn main() {
    // Tarea inmutable desde el hilo principal
    let tarea_hilo = Tarea::nueva(11, String::from("Tarea procesada en segundo plano"));
    // Creamos un hilo hijo que ejecuta la tarea usando el metodo `ejecutar()` que modifica el estado interno de la tarea, por lo tanto es necesario usar `move` para transferir ownership al hilo secundario
    let handle = thread::spawn(move || {
        tarea_hilo.ejecutar();
    });
    println!("Hilo principal: esperando que el hilo secundario termine...");
    println!("{:?}", handle.join());
}
```

En este ejemplo vemos el basico metodo de concurrencia **Fork-Join**, el hilo principal crea una tarea inmutable y luego lanza un hilo secundario (fork) que ejecuta esa tarea. El hilo secundario puede modificar el estado interno de la tarea ya que le transferimos el ownership de la tarea usando `move` al momento de crear el hilo. Ademas modificamos el estado interno de la tarea ya que usamos el metodo `ejecutar()` que tiene permisos para modificar el estado interno de la misma. Por otro lado el hilo secundario toma el ownership de la tarea asegurando el acceso seguro a la misma sin necesidad de usar locks o mutex.

Por lo tanto mediante la inmutabilidad/mutabilidad de Rust podemos garantizar la seguridad y el rendimiento de nuestro programa, evitando errores comunes relacionados con el estado mutable y facilitando la concurrencia y el paralelismo de manera eficiente.

#### <u>Heap vs Stack</u>

Las principales diferencias entre los 2 tipos de memoria son:

- **Stack**: Almacena los datos de **tamaño fijo y conocidos** en tiempo de compilacion, por ende son datos de un acceso muy rapido. Ademas son datos cuyos recursos se liberan automaticamente al salirnos del scope por ende no liberamos la memoria manualmente. Por ejemplo tipos de datos cuyo tamaño en bits es fijo y conocido como los tipos de datos:

    - Enteros (`i32`, `u64`, etc.)
    - Flotantes (`f32`, `f64`)
    - Booleanos (`bool`)
    - Caracteres (`char`)
    - etc...

- **Heap**: Almacena datos de **tamaño dinamico o desconocido** en tiempo de compilacion, por lo tanto el acceso a esta memoria es mas lento que el stack. En Rust, los datos almacenados en el heap se gestionan a través del **sistema de ownership**, lo que garantiza la seguridad de la memoria sin necesidad de un recolector de basura, es decir, para la liberacion de memoria se utiliza el mismo ownership de la variable. Por ejemplo tipos de datos cuyo tamaño no es fijo o conocido se almacenan en Heap como:

    - Strings (`String`)
    - Vectores (`Vec<T>`)
    - HashMaps (`HashMap<K, V>`)
    - etc...

Ejemplificando un poco a partir de nuestro codigo se da la utilizacion del Heap y Stack en el siguiente segmento de codigo al definir el struct `Tarea`:

```rust
pub struct Tarea {
    id: u32, // Se almacena en Stack ya que es un tamaño fijo de 4 bytes
    descripcion: String, // Se almacena en Heap ya que tiene un tamaño dinamico
    estado: EstadoTarea, // Se almacena en Stack ya que EstadoTarea es un enum con variantes conocidas
}

impl Tarea {
    // Al crear una nueva tarea el ID y su estado se almacenan en Stack mientras que la descripcion se almacena en Heap ya que es un String con tamaño dinamico
    pub fn nueva(id: u32, descripcion: String) -> Self {
        Tarea {
            id,
            descripcion,
            estado: EstadoTarea::Pendiente,
        }
    }
}
```

Un detalle importante es que cuando por ejemplo en el Heap se almacena la descripcion de la tarea (String) lo que realmente se almacena en el Stack es un **puntero** a esa descripcion en el Heap, junto con su longitud y capacidad. Esto es parte del sistema de ownership de Rust que nos permite gestionar la memoria de manera segura sin necesidad de un recolector de basura.

Por ejemplo si la descricion de la tarea es "Comprar alimentos", lo que se almacenaria seria lo siguiente:

- En **Heap**: Se almacena los datos puros, osea los bytes de "Comprar alimentos"

- En **Stack**: Se almacena un puntero a la ubicacion en el Heap donde se encuentra "Comprar alimentos", junto con su longitud (17 bytes) y capacidad (17 bytes)

### 2. Modelado de Datos y Conocimiento

#### <u>Structs y Metodos</u>

Implementamos el siguiente struct `Tarea` en *tarea.rs* con los siguientes campos detallados:

```rust
pub struct Tarea {
    /// Identificador único de la tarea.
    id: u32,
    /// Descripción detallada de la tarea.
    descripcion: String,
    /// Estado actual de la tarea, representado por el enum EstadoTarea.
    estado: EstadoTarea,
}
```

Y a continuacion implementamos como ejemplo el siguiente metodo asociado a la tarea con la palabra reservada `impl` donde le definimos entre otros el siguiente metodo:

```rust
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
    ...
    ...
}
```

En este ultimo segmento de codigo definimos con la palabra reservada `impl` algunos metodos de la struct `Tarea` donde entre ellos esta `nueva()` (que seria el constructor) y `iniciar()` (que seria justamente para iniciar la tarea y cambiar su estado a *EnProgreso*)

#### <u>Enums y Pattern Matching</u>

El enum que implementamos para las distintas tareas se encuentra en *estado_tarea.rs*, donde se detalla todos los estados por el que puede pasar una tarea:

```rust
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
```

Por otro lado la funcion que implementamos para tomar una tarea y segun su estado llevar a cabo distintas acciones (mediante **pattern matching**) es la siguiente que se encuentra en *gestor_tareas.rs*:

```rust
pub fn procesar_por_id(&mut self, id: u32) -> Result<(), String> {
    // Buscamos la tarea que coincida con la ID
    let tarea = self
        .tareas
        .iter_mut()
        .find(|tarea| tarea.coincide_con_id(id));
    // Primer pattern matching por si efectivamente obtenemos una tarea dada la ID o None
    match tarea {
        Some(tarea) => {
            // Segundo pattern matching por si se encontro la tarea entonces evaluamos su estado
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
```

Es decir, viendo el ultimo segmento de codigo logramos mediante pattern matching identificar si una tarea existe o no, y en caso de que exista entonces que logica se llevaria a cabo dependiendo de su estado interno. En caso de que el estado de la tarea sea *EnProgreso* entonces ejecutarla

#### <u>Traits</u>

Definimos el siguiente trait `Procesable` donde internamente se implementa el metodo `ejectar()` en *procesable.rs*:

```rust
/// Define un comportamiento común para las tareas que pueden ser procesadas.
pub trait Procesable {
    fn ejecutar(&mut self); // &mut self porque el método ejecutar modificará el estado de la tarea
}
```

Lo que hace `ejecutar()` es recibir una tarea como estado mutable para justamente modificar su estado interno

Luego implementamos dicho Trait `Procesable` al struct `Tarea` en *tarea.rs* donde detallamos la logica del metodo `ejecutar()` para las tareas que recibe.

```rust
// Definimos el Trait 'Procesable' para Tarea que vendria a funcionar como una interfaz en otros lenguajes como en Go
impl Procesable for Tarea {
    /// Método para ejecutar la tarea, cambiando su estado a EnProgreso y luego a Completada
    fn ejecutar(&mut self) {
        self.iniciar(); // Metodo que cambia su estado a EnProgreso
        println!("Procesando la tarea '{}'", self.descripcion);
        self.completar(); // Metodo que cambia su estado a Completada
    }
}
```

Una de las particularidades de Rust es que implementa Traits en lugar de herencia clasica de objetos. Esto es porque la herencia clasica se lleva a cabo una jerarquia muy rigida de clases por tanto un cambio accidental de una clase podria romper todas las demas subclases. Es por ello que existen los Traits (conocido como interfaces en otros lenguajes) que sirven basicamente para **implementar comportamientos** especificos a cada una de las clases. Por ejemplo nosotros en este caso implementamos el trait `Procesable` e internamente el metodo `ejecutar()` donde luego le definimos ese mismo trait al struct Tarea, sin embargo a futuro podriamos implementar el mismo trait con el mismo metodo a otro struct, por ejemplo imaginando que tenemos un struct `Accion` o `Calculo`

### 3. Seguridad y manejo de errores

#### <u>Option</u>

En otros lenguajes, la implementación del valor nulo (`null`) puede permitir muchos errores y problemas de gran riesgo. Por lo tanto, Rust **no tiene valores nulos**, sino que implementa este concepto utilizando un tipo `enum` que sirve para representar la presencia o ausencia de un valor: `Option<T>`, definido en su librería estándar.

Veamos un ejemplo usando un `GestorDeTareas` que contiene el siguiente método:

```rust
    /// Busca una tarea por ID.
    /// Devuelve una copia `Option<Tarea>`.
    /// - Si el ID existe, devuelve `Some(tarea)`
    /// - Si el ID no existe, devuelve `None`
    pub fn obtener_por_id(&self, id: u32) -> Option<Tarea> {
        self.buscar(id).cloned()
    }
```

Ejemplo de uso: (con pattern matching que permite manejar ambos casos)

```rust
    match gestor.obtener_por_id(1) {
        Some(t) => println!("Tarea encontrada: {:?}", t),
        None => println!("Tarea no encontrada"),
    }
```

De esta manera, si encontramos el valor que buscamos se devuelve dentro de `Some()`. Si no se encuentra el valor, simplemente se obtiene `None`. El compilador nos obliga a manejar ambos casos con el match que cubre las variantes `Some` y `None`, y así nos aseguramos de que siempre habrá un valor válido porque hemos cubierto todas las posibilidades.

El manejo del enum `Option` y del `Result` (que veremos luego) juegan un rol clave en la seguridad y el manejo correcto de la memoria, características fundamentales de Rust.

#### <u>Result</u>

Así como tenemos `Option` para cuando un valor puede faltar, Rust también usa otro enum llamado `Result` para representar operaciones que pueden fallar. `Result` tiene dos variantes: `Ok(T)` cuando todo sale bien, y `Err(E)` cuando ocurre un error.

En el `GestorDeTareas`, implementamos el metodo `procesar_por_id` que devuelve un `Result<(), String>`. El `Ok(())` significa que la tarea se procesó con exito, y el `Err(String)` trae un mensaje de error explicando que falló.

```rust
    pub fn procesar_por_id(&mut self, id: u32) -> Result<(), String> {
        let tarea = self.tareas.iter_mut().find(|t| t.id == id);
        match tarea {
            Some(t) => {
                match t.estado {
                    EstadoTarea::Pendiente => Ok(t.ejecutar()),
                    EstadoTarea::EnProgreso => Err("La tarea ya está en progreso".to_string()),
                    EstadoTarea::Completada => Err("La tarea ya fue completada".to_string()),
                    EstadoTarea::Fallida => Err("La tarea falló anteriormente".to_string()),
                }
            }
            None => Err(format!("No se encontró la tarea con ID {}", id)),
        }
    }
```

Si la tarea esta `Pendiente`, llamamos a `ejecutar()` y envolvemos el resultado en `Ok(())`. Si la tarea no esta `Pendiente`, o directamente no existe, devolvemos un `Err` con el motivo.

Ejemplo de uso: (con pattern matching que permite manejar ambos casos resultantes)

```rust
    match gestor.procesar_por_id(1) {
        Ok(()) => println!("Tarea 1 procesada con éxito"),
        Err(e) => println!("Error al procesar tarea 1: {}", e),
    }

```

El compilador nos obliga a manejar los dos casos: el exitoso `Ok` y el fallido `Err`. Asi nos aseguramos que nunca se nos pase por alto un error. Esto hace que nuestro programa sea mas robusto frente a fallas, porque permite que pensemos en todas las posibilidades y podamos decidir como manejarlas antes de que ocurran.

---

### 4. Ownership, Threads y Testing

#### <u>Ownership</u>

En Rust, cada valor tiene un único dueño (_owner_). Cuando ese dueño sale de scope, el valor se libera. Si pasamos un valor a una función **por valor**, el ownership se mueve (_move_) a esa función y la variable original queda inválida.

En el código definimos dos funciones para ilustrar esto:

```rust
// Toma ownership: tarea deja de ser válida en el llamador
fn describir_tarea_por_valor(t: Tarea) {
    println!("(por valor) {:?}", t);
}

// Recibe un préstamo: tarea sigue siendo válida en el llamador
fn describir_tarea_por_referencia(t: &Tarea) {
    println!("(por referencia) {:?}", t);
}
```

Si intentáramos usar `describir_tarea_por_valor` y luego acceder a la variable original, el compilador lo rechazaría:

```rust
// Esto NO compila:
let tarea_demo = Tarea::nueva(10, String::from("Demo ownership"));
describir_tarea_por_valor(tarea_demo); // ownership movido
tarea_demo.imprimir_estado();          // error[E0382]: use of moved value
```

La solución es pasar una referencia con `&`, de modo que la función sólo _presta_ el valor sin tomar su ownership:

```rust
// Esto SÍ compila:
let tarea_demo = Tarea::nueva(10, String::from("Demo ownership"));
describir_tarea_por_referencia(&tarea_demo); // préstamo inmutable
tarea_demo.imprimir_estado();                // OK: seguimos siendo dueños
```

#### <u>Threads</u>

Rust permite lanzar hilos con `std::thread::spawn`. Para que el hilo pueda usar una variable local, usamos la palabra clave `move` en el closure, que transfiere el ownership de la variable al hilo:

```rust
let tarea_hilo = Tarea::nueva(11, String::from("Tarea procesada en segundo plano"));
let handle = thread::spawn(move || {
    let mut t = tarea_hilo; // ownership transferido al hilo secundario
    t.ejecutar();
});
println!("Hilo principal: esperando que el hilo secundario termine...");
handle.join().unwrap(); // esperamos a que el hilo termine
```

El hilo principal sigue ejecutándose (imprime el mensaje de espera) mientras el hilo secundario procesa la tarea. `join()` bloquea el hilo principal hasta que el secundario termina, garantizando que no se pierda trabajo.
