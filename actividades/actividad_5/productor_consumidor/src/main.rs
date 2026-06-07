use rand::random_range;
use std::{
    collections::VecDeque,       // Estructura del buffer
    sync::{Arc, Condvar, Mutex}, // Estructuras de sincronización
    thread::spawn,
};

const CAPACIDAD: usize = 5; // Capacidad del buffer
const NUMEROS: u8 = 20; // Cantidad de elementos a procesar
const CONSUMIDORES: u8 = 2; // Cantidad de consumidores
const MIN: usize = 1; // Número random mínimo
const MAX: usize = 100; // Número random máximo

fn producir(par: &Arc<(Mutex<VecDeque<usize>>, Condvar)>) -> Result<(), String> {
    let buffer = &par.0;
    let condvar = &par.1;

    // Productor genera 20 numeros aleatorios [1,100]
    for _ in 0..NUMEROS {
        // Se genera un numero aleatorio del 1 al 100
        let numero_random = random_range(MIN..=MAX);

        // Se espera a que haya espacio en el buffer
        let mut buffer = condvar
            .wait_while(
                buffer.lock().map_err(|error| error.to_string())?,
                |buffer| buffer.len() >= CAPACIDAD,
            )
            .map_err(|error| error.to_string())?;

        // Se inserta el número
        buffer.push_back(numero_random);

        // Se libera el lock
        drop(buffer);

        println!("[PRODUCTOR] Genere {numero_random}");

        // No es necesario notificar a todos al tener un solo productor
        condvar.notify_one();
    }
    println!("[PRODUCTOR] Termine de generar números aleatorios");
    Ok(())
}

fn consumir(id: u8, par: &Arc<(Mutex<VecDeque<usize>>, Condvar)>) -> Result<(), String> {
    let buffer = &par.0;
    let condvar = &par.1;

    let mut numeros_procesados: u8 = 0;

    // Si el consumidor proceso la mitad de items generados entonces termina su parte
    while numeros_procesados < NUMEROS / CONSUMIDORES {
        // Se espera a que haya elementos en el buffer
        let mut buffer = condvar
            .wait_while(
                buffer.lock().map_err(|error| error.to_string())?,
                |buffer| buffer.is_empty(),
            )
            .map_err(|error| error.to_string())?;

        // Se obtiene un número del buffer
        let numero = buffer.pop_front().ok_or("No hay nada en el buffer")?;

        // Se libera el lock
        drop(buffer);

        println!("[CONSUMIDOR {id}] Procese {numero}");
        numeros_procesados += 1;

        // Se notifica a todos para asegurar que se notifique al productor
        condvar.notify_all();
    }
    println!("[CONSUMIDOR {id}] Termine mi trabajo");
    Ok(())
}

fn main() {
    // Estado inicial
    let buffer = Mutex::new(VecDeque::new());
    let condvar = Condvar::new();
    let par = Arc::new((buffer, condvar));

    // Para agregar hilos
    let mut handles = vec![];

    let par_productor = par.clone();

    // Hilo productor
    let productor = spawn(move || {
        if let Err(error) = producir(&par_productor) {
            eprintln!("[PRODUCTOR] {error}");
        }
    });
    handles.push(productor);

    for id in 1..=CONSUMIDORES {
        let par = par.clone();

        // Hilo consumidor
        let consumidor = spawn(move || {
            if let Err(error) = consumir(id, &par) {
                eprintln!("[CONSUMIDOR {id}] {error}");
            }
        });
        handles.push(consumidor);
    }
    // Hilo principal espera a los 3 hilos (1 productor y 2 consumidores)
    for hilo in handles {
        if hilo.join().is_err() {
            eprintln!("[PRINCIPAL] Error al hacer join");
        }
    }
    println!("[PRINCIPAL] Fin de ejecución");
}
