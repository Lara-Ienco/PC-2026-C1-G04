use rand::random_range;
use std::{
    sync::{Arc, Condvar, Mutex},
    thread::{sleep, spawn},
    time::Duration,
};

const CLIENTES: usize = 10;
const SILLAS: usize = 3;
const MIN_ATENDER: u64 = 50;
const MAX_ATENDER: u64 = 100;
const MIN_ARRIBO: u64 = 200;
const MAX_ARRIBO: u64 = 600;

struct EstadoBarberia {
    sillas_libres: usize,
    barbero_durmiendo: bool,
    cortes_terminados: u8,
    abierta: bool,
}

struct Barberia {
    estado: Mutex<EstadoBarberia>,
    cv_cliente: Condvar,
    cv_barbero: Condvar,
}

fn barbero(barberia: &Arc<Barberia>) -> Result<(), String> {
    loop {
        let mut estado = barberia.estado.lock().map_err(|error| error.to_string())?;

        if estado.sillas_libres == SILLAS {
            println!("[BARBERO] No hay clientes, me voy a dormir");
            estado.barbero_durmiendo = true;
        }
        let mut estado = barberia
            .cv_barbero
            .wait_while(estado, |estado| {
                estado.sillas_libres == SILLAS && estado.abierta
            })
            .map_err(|error| error.to_string())?;

        if !estado.abierta {
            println!("[BARBERO] Ya cerramos, me voy a casa.");
            break;
        }
        if estado.barbero_durmiendo {
            println!("[BARBERO] Despierto y atiendo a un cliente...");
            estado.barbero_durmiendo = false;
        }
        drop(estado);
        println!("[BARBERO] ¡Siguiente!");
        sleep(Duration::from_millis(random_range(
            MIN_ATENDER..=MAX_ATENDER,
        )));
        println!("[BARBERO] Terminé de atender a un cliente.");
        barberia
            .estado
            .lock()
            .map_err(|error| error.to_string())?
            .cortes_terminados += 1;
        barberia.cv_cliente.notify_one();
    }
    Ok(())
}

fn cliente(id: usize, barberia: &Arc<Barberia>) -> Result<(), String> {
    let espera = random_range(MIN_ARRIBO..=MAX_ARRIBO);
    sleep(Duration::from_millis(espera));
    println!("[CLIENTE {id}] Llegó a la barberia.");

    let mut estado = barberia.estado.lock().map_err(|error| error.to_string())?;

    // CASO 3: Barbero está ocupado y no hay silla libre. Me voy
    if estado.sillas_libres == 0 {
        println!("[CLIENTE {id}] No hay lugar, asi que me voy (no muy contento)");
        return Ok(());
    }
    estado.sillas_libres -= 1;

    // CASO 2: El barbero está dormido, lo despierto
    if estado.barbero_durmiendo {
        println!("[CLIENTE {id}] Barbero está durmiendo, lo despierto");
        barberia.cv_barbero.notify_one();
    }
    println!("[CLIENTE {id}] Hay lugar, me siento a esperar.");
    let mut estado = barberia
        .cv_cliente
        .wait_while(estado, |estado| estado.cortes_terminados == 0)
        .map_err(|error| error.to_string())?;
    println!("[CLIENTE {id}] Me atendieron, me voy contento :)");
    estado.sillas_libres += 1;
    estado.cortes_terminados -= 1;
    Ok(())
}

fn main() {
    let estado_barberia = EstadoBarberia {
        sillas_libres: SILLAS,
        barbero_durmiendo: false,
        cortes_terminados: 0,
        abierta: true,
    };
    let barberia = Arc::new(Barberia {
        estado: Mutex::new(estado_barberia),
        cv_cliente: Condvar::new(),
        cv_barbero: Condvar::new(),
    });
    // HILO BARBERO
    let barberia_barbero = Arc::clone(&barberia);
    let barbero_handle = spawn(move || {
        if let Err(error) = barbero(&barberia_barbero) {
            eprintln!("[BARBERO] {error}");
        }
    });

    // WHILE QUE GENERA LOS CLIENTES
    let mut handles = vec![];

    for id in 0..CLIENTES {
        let barberia = Arc::clone(&barberia);
        let handle = spawn(move || {
            if let Err(error) = cliente(id, &barberia) {
                eprintln!("[CLIENTE {id}] {error}");
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        if let Err(error) = handle.join() {
            eprintln!("[PRINCIPAL] Error esperando a un hilo cliente: {error:?}");
        }
    }
    // Cerrar barberia cuando todos los clientes  se hayan ido
    {
        let Ok(mut estado) = barberia.estado.lock() else {
            eprintln!("[PRINCIPAL] Error al intentar cerrar la barbería.");
            return;
        };
        estado.abierta = false;
        barberia.cv_barbero.notify_one();
    }
    if let Err(e) = barbero_handle.join() {
        eprintln!("[PRINCIPAL] Error esperando al barbero: {e:?}");
    }
    println!("[PRINCIPAL] La barbería ha cerrado por hoy :).");
}
