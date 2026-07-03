use rand::random_range;
use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

// --- CONSTANTES DEL SISTEMA ---
const MAX_TIME: Duration = Duration::from_secs(8);
const INTERVAL_NORMAL: Duration = Duration::from_millis(500);
const INTERVAL_LENTO: Duration = Duration::from_millis(1200);

// Constantes de Monitoreo
const TIMEOUT_RECV: Duration = Duration::from_millis(50);
const UBRAL_LENTITUD: Duration = Duration::from_millis(1000);
const TIMEOUT_SOSPECHOSO: Duration = Duration::from_millis(1500);
const TIMEOUT_CAIDO: Duration = Duration::from_millis(3000);

/// Estados posibles de un nodo
#[derive(Debug, Clone, Copy, PartialEq)]
enum EstadoNodo {
    Activo,
    Sospechoso,
    Caido,
}

/// Información de un nodo
struct InfoNodo {
    estado: EstadoNodo,
    ultimo_heartbeat: Instant,
    tuvo_lentitud: bool,
    tiempo_caida: Option<Duration>,
}

fn main() {
    let inicio = Instant::now();
    let (tx, rx) = mpsc::channel();

    // Spawn de nodos

    // Nodo 1: normal (500ms, siempre)
    let tx1 = tx.clone();

    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_millis(500));
            if tx1.send(1).is_err() {
                break;
            }
        }
    });

    // Nodo 2: se vuelve lento después de 3s (heartbeat cada 1200ms)
    let tx2 = tx.clone();
    thread::spawn(move || {
        loop {
            let elapsed = inicio.elapsed();
            let delay = if elapsed >= Duration::from_secs(3) && elapsed < Duration::from_secs(6) {
                LENTITUD
            } else {
                INTERVAL_TIME
            };
            thread::sleep(delay);
            if tx2.send(2).is_err() {
                break;
            }
        }
    });

    // Nodo 3: muere después de un tiempo aleatorio entre 2 y 4 segundos
    let tx3 = tx.clone();
    let lifetime = Duration::from_millis(random_range(2000..=4000));
    thread::spawn(move || {
        let start_nodo3 = Instant::now();
        while start_nodo3.elapsed() < lifetime {
            thread::sleep(INTERVAL_TIME);
            if start_nodo3.elapsed() >= lifetime {
                break;
            }
            if tx3.send(3).is_err() {
                break;
            }
        }
    });

    drop(tx);
    // Monitor
    let limit = MAX_TIME;

    let mut info_nodos = HashMap::new();
    for id in 1..=3 {
        info_nodos.insert(
            id,
            InfoNodo {
                estado: EstadoNodo::Activo,
                ultimo_heartbeat: Instant::now(),
                tuvo_lentitud: false,
                tiempo_caida: None,
            },
        );
    }
    println!("***** Iniciando Monitoreo *****");

    while inicio.elapsed() < limit {
        let elapsed = inicio.elapsed();
        let remaining = limit.saturating_sub(elapsed);
        if remaining.is_zero() {
            break;
        }

        // usamos un timeout corto de 50ms para evaluar los timeouts y no bloquear
        let recv_timeout_duration = Duration::from_millis(50).min(remaining);
        match rx.recv_timeout(recv_timeout_duration) {
            Ok(id) => {
                let now = Instant::now();
                let elapsed_total = now.duration_since(inicio).as_secs_f32();
                if let Some(info) = info_nodos.get_mut(&id) {
                    let elapsed_desde_ultimo = now.duration_since(info.ultimo_heartbeat);

                    // consideramos latido tardio si tardó más de 1.0 segundos (lo normal es ~500ms) !!
                    let es_tardio = elapsed_desde_ultimo > Duration::from_millis(1000);

                    if es_tardio {
                        info.tuvo_lentitud = true;
                        info.estado = EstadoNodo::Sospechoso;
                        info.ultimo_heartbeat = now;
                        println!(
                            "[{:.1}s] Monitor: Nodo {} heartbeat tardío ({:.1}s). Estado: SOSPECHOSO",
                            elapsed_total, id, elapsed_desde_ultimo.as_secs_f32()
                        );
                    } else {
                        let anterior_estado = info.estado;
                        info.estado = EstadoNodo::Activo;
                        info.ultimo_heartbeat = now;

                        if anterior_estado == EstadoNodo::Sospechoso || anterior_estado == EstadoNodo::Caido {
                            println!(
                                "[{:.1}s] Monitor: heartbeat de Nodo {}. Estado: ACTIVO (recuperado)",
                                elapsed_total, id
                            );
                        } else {
                            println!(
                                "[{:.1}s] Monitor: heartbeat de Nodo {}. Estado: ACTIVO",
                                elapsed_total, id
                            );
                        }
                    }
                }
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {
                // si hay timeout seguimos chequeando y esperando
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => {
                // si se desconectan los nodos, salimos
                break;
            }
        }

        // Evaluar timeouts periódicamente
        let now = Instant::now();
        let elapsed_total = now.duration_since(inicio);
        for (&id, info) in info_nodos.iter_mut() {
            let elapsed_desde_ultimo = now.duration_since(info.ultimo_heartbeat);
            match info.estado {
                EstadoNodo::Activo => {
                    if elapsed_desde_ultimo >= Duration::from_millis(1500) {
                        info.estado = EstadoNodo::Sospechoso;
                        println!(
                            "[{:.1}s] Monitor: Nodo {} sin heartbeat hace {:.1}s. Estado: SOSPECHOSO",
                            elapsed_total.as_secs_f32(), id, elapsed_desde_ultimo.as_secs_f32()
                        );
                    }
                }
                EstadoNodo::Sospechoso => {
                    if elapsed_desde_ultimo >= Duration::from_millis(3000) {
                        info.estado = EstadoNodo::Caido;
                        info.tiempo_caida = Some(elapsed_total);
                        println!(
                            "[{:.1}s] Monitor: Nodo {} sin heartbeat hace {:.1}s. Estado: CAÍDO",
                            elapsed_total.as_secs_f32(), id, elapsed_desde_ultimo.as_secs_f32()
                        );
                    }
                }
                EstadoNodo::Caido => {}
            }
        }
    }

    //Reporte final
    println!("\n**** Reporte final (8s) ****");
    let now = Instant::now();
    for id in 1..=3 {
        if let Some(info) = info_nodos.get(&id) {
            let hace_secs = now.duration_since(info.ultimo_heartbeat).as_secs_f32();
            match info.estado {
                EstadoNodo::Activo => {
                    let lentitud_str = if info.tuvo_lentitud {
                        " — tuvo episodios de lentitud"
                    } else {
                        ""
                    };
                    println!(
                        "Nodo {}: ACTIVO (último heartbeat hace {:.1}s){}",
                        id, hace_secs, lentitud_str
                    );
                }
                EstadoNodo::Sospechoso => {
                    println!(
                        "Nodo {}: SOSPECHOSO (último heartbeat hace {:.1}s)",
                        id, hace_secs
                    );
                }
                EstadoNodo::Caido => {
                    let t_caido = info.tiempo_caida.unwrap_or(Duration::ZERO).as_secs_f32();
                    println!(
                        "Nodo {}: CAÍDO (último heartbeat hace {:.1}s) — declarado caído en t={:.1}s",
                        id, hace_secs, t_caido
                    );
                }
            }
        }
    }
}
