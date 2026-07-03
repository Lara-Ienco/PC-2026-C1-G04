use rand::random_range;
use std::{
    collections::HashMap,
    sync::mpsc::{Receiver, Sender, channel},
    thread::{sleep, spawn},
    time::{Duration, Instant},
};

// --- CONSTANTES DEL SISTEMA ---
const TOTAL_TIME: Duration = Duration::from_secs(8);
const HEARTBEAT_INTERVAL: Duration = Duration::from_millis(500);
const SLOW_HEARTBEAT_INTERVAL: Duration = Duration::from_millis(1200);
const SLOW_TIME: Duration = Duration::from_secs(3);
const MIN_CRASH_TIME: u64 = 2000;
const MAX_CRASH_TIME: u64 = 4000;
const TIMEOUTS_SOSPECHOSO: u32 = 2;
const TIMEOUTS_CAIDO: u32 = 3;

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

impl InfoNodo {
    const fn new(inicio: Instant) -> Self {
        Self {
            estado: EstadoNodo::Activo,
            ultimo_heartbeat: inicio,
            tuvo_lentitud: false,
            tiempo_caida: None,
        }
    }
}

fn nodo_normal(id: i32, tx: &Sender<i32>) {
    loop {
        sleep(HEARTBEAT_INTERVAL);

        if tx.send(id).is_err() {
            break;
        }
    }
}

fn nodo_lento(id: i32, tx: &Sender<i32>, inicio: Instant) {
    while inicio.elapsed() < SLOW_TIME {
        sleep(HEARTBEAT_INTERVAL);

        if tx.send(id).is_err() {
            break;
        }
    }
    loop {
        sleep(SLOW_HEARTBEAT_INTERVAL);

        if tx.send(id).is_err() {
            break;
        }
    }
}

fn nodo_crash(id: i32, tx: &Sender<i32>, inicio: Instant) {
    let crash_time = Duration::from_millis(random_range(MIN_CRASH_TIME..=MAX_CRASH_TIME));

    while inicio.elapsed() < crash_time {
        sleep(HEARTBEAT_INTERVAL);

        if tx.send(id).is_err() {
            break;
        }
    }
}

fn spawn_nodos(tx: &Sender<i32>) -> Instant {
    let inicio = Instant::now();

    let tx1 = tx.clone();
    spawn(move || {
        nodo_normal(1, &tx1);
    });

    let tx2 = tx.clone();
    spawn(move || {
        nodo_lento(2, &tx2, inicio);
    });

    let tx3 = tx.clone();
    spawn(move || {
        nodo_crash(3, &tx3, inicio);
    });
    inicio
}

fn recv_heartbeats(
    rx: &Receiver<i32>,
    timeout: Duration,
    inicio: Instant,
    info_nodos: &mut HashMap<i32, InfoNodo>,
) {
    if let Ok(id) = rx.recv_timeout(timeout) {
        let now = Instant::now();
        let elapsed_total = now.duration_since(inicio).as_secs_f32();

        if let Some(info) = info_nodos.get_mut(&id) {
            let elapsed_desde_ultimo = now.duration_since(info.ultimo_heartbeat);

            if elapsed_desde_ultimo > TIMEOUTS_SOSPECHOSO * HEARTBEAT_INTERVAL {
                info.tuvo_lentitud = true;
                info.estado = EstadoNodo::Sospechoso;
                info.ultimo_heartbeat = now;
                println!(
                    "[{elapsed_total:.1}s] Monitor: Nodo {id} heartbeat tardío ({:.1}s). Estado: SOSPECHOSO",
                    elapsed_desde_ultimo.as_secs_f32()
                );
            } else {
                let anterior_estado = info.estado;
                info.estado = EstadoNodo::Activo;
                info.ultimo_heartbeat = now;

                if anterior_estado == EstadoNodo::Sospechoso || anterior_estado == EstadoNodo::Caido
                {
                    println!(
                        "[{elapsed_total:.1}s] Monitor: heartbeat de Nodo {id}. Estado: ACTIVO (recuperado)",
                    );
                } else {
                    println!(
                        "[{elapsed_total:.1}s] Monitor: heartbeat de Nodo {id}. Estado: ACTIVO",
                    );
                }
            }
        }
    }
}

fn chequear_ultimos_heartbeats(inicio: Instant, info_nodos: &mut HashMap<i32, InfoNodo>) {
    let now = Instant::now();
    let elapsed_total = now.duration_since(inicio);

    for (&id, info) in info_nodos {
        let elapsed_desde_ultimo = now.duration_since(info.ultimo_heartbeat);

        if elapsed_desde_ultimo >= TIMEOUTS_CAIDO * HEARTBEAT_INTERVAL {
            info.estado = EstadoNodo::Caido;

            if info.tiempo_caida.is_none() {
                info.tiempo_caida = Some(elapsed_total);
            }
            println!(
                "[{:.1}s] Monitor: Nodo {} sin heartbeat hace {:.1}s. Estado: CAÍDO",
                elapsed_total.as_secs_f32(),
                id,
                elapsed_desde_ultimo.as_secs_f32()
            );
        } else if elapsed_desde_ultimo >= TIMEOUTS_SOSPECHOSO * HEARTBEAT_INTERVAL {
            info.estado = EstadoNodo::Sospechoso;
            println!(
                "[{:.1}s] Monitor: Nodo {} sin heartbeat hace {:.1}s. Estado: SOSPECHOSO",
                elapsed_total.as_secs_f32(),
                id,
                elapsed_desde_ultimo.as_secs_f32()
            );
        }
    }
}

fn bucle_monitoreo(rx: &Receiver<i32>, inicio: Instant) -> HashMap<i32, InfoNodo> {
    let mut info_nodos = HashMap::new();

    for id in 1..=3 {
        info_nodos.insert(id, InfoNodo::new(inicio));
    }
    println!("***** Iniciando Monitoreo *****");

    loop {
        let elapsed = inicio.elapsed();
        let Some(remaining) = TOTAL_TIME.checked_sub(elapsed) else {
            break;
        };
        let recv_timeout_duration = HEARTBEAT_INTERVAL.min(remaining);

        recv_heartbeats(rx, recv_timeout_duration, inicio, &mut info_nodos);
        chequear_ultimos_heartbeats(inicio, &mut info_nodos);
    }

    info_nodos
}

fn reporte_final(info_nodos: &HashMap<i32, InfoNodo>) {
    println!("\n**** Reporte final (8s) ****");
    let now = Instant::now();

    for id in 1..=3 {
        if let Some(info) = info_nodos.get(&id) {
            let hace_secs = now.duration_since(info.ultimo_heartbeat).as_secs_f32();

            match info.estado {
                EstadoNodo::Activo | EstadoNodo::Sospechoso => {
                    if info.tuvo_lentitud {
                        println!(
                            "Nodo {id}: ACTIVO (último heartbeat hace {hace_secs:.1}s) — tuvo episodios de lentitud",
                        );
                    } else {
                        println!("Nodo {id}: ACTIVO (último heartbeat hace {hace_secs:.1}s)",);
                    }
                }
                EstadoNodo::Caido => {
                    let t_caido = info.tiempo_caida.unwrap_or(Duration::ZERO).as_secs_f32();
                    println!(
                        "Nodo {id}: CAÍDO (último heartbeat hace {hace_secs:.1}s) — declarado caído en t={t_caido:.1}s",
                    );
                }
            }
        }
    }
}

fn main() {
    let (tx, rx) = channel();
    let inicio = spawn_nodos(&tx);
    let info_nodos = bucle_monitoreo(&rx, inicio);
    reporte_final(&info_nodos);
}
