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
