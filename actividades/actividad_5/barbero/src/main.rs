use rand::Rng;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

const CLIENTES: usize = 10;
const SILLAS: usize = 3;

struct Barberia {
    sillas_libres: usize,
    barbero_esta_durmiendo: bool,
    barberia_abierta: bool,
}

struct BarberiaCompartida {
    estado: Mutex<Barberia>,
    cv_cliente: Condvar,
    cv_barbero: Condvar,
}

fn main() {
    let barberia = Arc::new(BarberiaCompartida {
        estado: Mutex::new(Barberia {
            sillas_libres: SILLAS,
            barbero_esta_durmiendo: true,
            barberia_abierta: true,
        }),
        cv_cliente: Condvar::new(),
        cv_barbero: Condvar::new(),
    });

    //HILO BARBERO
    let barberia_comp = Arc::clone(&barberia);
    let barbero_handle = thread::spawn(move || {
        let mut guard = match barberia_comp.estado.lock() {
            Ok(g) => g,
            Err(_) => {
                println!("Barbero: El mutex está envenenado. Saliendo.");
                return;
            }
        };
        loop {
            if !guard.barberia_abierta && guard.sillas_libres == SILLAS {
                println!("Barbero: No hay más clientes y ya cerramos, me voy a casa.");
                break;
            }
            if guard.sillas_libres == SILLAS {
                println!("Barbero: No hay clientes, me voy a dormir");
                guard.barbero_esta_durmiendo = true;
                match barberia_comp.cv_barbero.wait(guard) {
                    Ok(g) => guard = g,
                    Err(_) => {
                        println!("Barbero: Error en la condvar (envenenamiento). Saliendo.");
                        break;
                    }
                }
            } else {
                guard.barbero_esta_durmiendo = false;
                guard.sillas_libres += 1;
                println!("Barbero: ¡Siguiente!");
                println!("Sillas libres ahora: {})", guard.sillas_libres);
                barberia_comp.cv_cliente.notify_one();
                drop(guard);
                println!("Barbero: Despierto y atiendo a un cliente...");
                thread::sleep(Duration::from_millis(
                    rand::thread_rng().gen_range(200..600),
                ));
                println!("Barbero: Terminé de atender a un cliente.");
                match barberia_comp.estado.lock() {
                    Ok(g) => guard = g,
                    Err(_) => {
                        println!("Barbero: No se pudo readquirir el lock. Saliendo.");
                        break;
                    }
                }
            }
        }
    });

    //WHILE QUE GENERA LOS CLIENTES
    let mut handles = vec![];
    let mut rng = rand::thread_rng();

    for id in 1..=CLIENTES {
        let espera = rng.gen_range(50..200);
        thread::sleep(Duration::from_millis(espera));
        let barberia_comp = Arc::clone(&barberia);

        let handle = thread::spawn(move || {
            let mut guard = match barberia_comp.estado.lock() {
                Ok(g) => g,
                Err(_) => {
                    println!("Cliente {}: No se pudo readquirir el lock. Saliendo.", id);
                    return;
                }
            };
            println!("Cliente {}: Llegó a la barberia.", id);

            // CASO 1: Hay silla libre, me quedo
            if guard.sillas_libres > 0 {
                guard.sillas_libres -= 1;
                println!("Cliente {}: Hay lugar, me siento a esperar.", id);
                println!("Sillas libres: {})", guard.sillas_libres);

                // CASO 1: El barbero está dormido, lo despierto
                if guard.barbero_esta_durmiendo {
                    println!("Cliente {}: Barbero está durmiendo, lo despierto", id);
                    barberia_comp.cv_barbero.notify_one();
                }

                // CASO 1 y 2: El barbero está despierto, espero mi turno
                match barberia_comp.cv_cliente.wait(guard) {
                    Ok(g) => guard = g,
                    Err(_) => {
                        println!("Cliente {}: Error esperando el corte.", id);
                        return;
                    }
                }
                println!("Cliente {}: Me están atendiendo, me voy contento :)", id);
            }
            //CASO 3: Barbero está ocupado y no hay silla libre. Me voy
            else {
                println!(
                    "Cliente {}: NO hay lugar, asi que me voy (no muy contento)",
                    id
                )
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        if let Err(e) = handle.join() {
            println!("Error esperando a un hilo cliente: {:?}", e)
        }
    }

    // cerrar barberia cuando todos los clientes hayan llegado y se hayan ido
    {
        let mut guard = match barberia.estado.lock() {
            Ok(g) => g,
            Err(_) => {
                println!("Error al intentar cerrar la barbería.");
                return;
            }
        };
        guard.barberia_abierta = false;
        barberia.cv_barbero.notify_all();
    }

    if let Err(e) = barbero_handle.join() {
        println!("Error esperando al barbero: {:?}", e);
    }

    println!("La barbería ha cerrado por hoy :).");
}
