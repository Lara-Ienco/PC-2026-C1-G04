use rand::{random_bool, random_range};
use std::{
    sync::mpsc::{Receiver, Sender, channel},
    thread::{sleep, spawn},
    time::{Duration, Instant},
};

const REPLICAS: usize = 5;
const QUORUM: usize = 3;
const TIMEOUT: Duration = Duration::from_secs(1);
const RONDAS: usize = 3;

const VALOR: i32 = 42;
const FALLA: f64 = 0.3;
const MIN_LATENCIA: u64 = 100;
const MAX_LATENCIA: u64 = 800;

fn lanzar_replica(id: usize, tx: Sender<(usize, i32)>) -> Sender<i32> {
    let (tx_envio, rx_envio) = channel();

    spawn(move || {
        // El nodo falla aleatoriamente
        if random_bool(FALLA) {
            println!("Réplica {id}: falla simulada, no respondo.");
            return; // Terminamos el hilo sin replicar nada.
        }
        // El nodo confirma exitosamente (con latencia variable)
        if let Ok(msg) = rx_envio.recv() {
            let latencia = Duration::from_millis(random_range(MIN_LATENCIA..=MAX_LATENCIA));
            sleep(latencia);
            println!(
                "Réplica {}: recibí valor {}, confirmado. (latencia: {}ms)",
                id,
                msg,
                latencia.as_millis()
            );
            let _ = tx.send((id, msg));
        }
    });
    tx_envio
}

fn esperar_confirmaciones(rx: &Receiver<(usize, i32)>) -> usize {
    let mut confirmaciones = 0;
    let tiempo_inicial = Instant::now();

    loop {
        if confirmaciones >= QUORUM {
            return confirmaciones;
        }
        let tiempo_transcurrido = tiempo_inicial.elapsed();

        let Some(tiempo_restante) = TIMEOUT.checked_sub(tiempo_transcurrido) else {
            println!("Se acabo el timeout");
            return confirmaciones;
        };
        if let Ok((id, _)) = rx.recv_timeout(tiempo_restante) {
            confirmaciones += 1;
            println!("Coordinador: confirmación recibida de Réplica {id}.");
        }
    }
}

fn escritura_con_quorum() {
    // Loop: ejecutar 3 rondas de escritura
    for i in 1..=RONDAS {
        println!("=== Ronda {i} ===");

        // Channel para confirmaciones
        let (tx, rx) = channel();

        let msg = VALOR; // mensaje a enviar
        let mut tx_envios = Vec::new();

        println!("Coordinador: enviando valor {msg} a {REPLICAS} réplicas...");

        // Lanzamos 5 threads réplicas
        for j in 1..=REPLICAS {
            let tx_envio = lanzar_replica(j, tx.clone());
            tx_envios.push(tx_envio);
        }
        // Enviamos el valor a los nodos de la red
        for tx in &tx_envios {
            let _ = tx.send(msg);
        }
        // Coordinador espera confirmaciones hasta alcanzar el QUORUM
        let confirmados = esperar_confirmaciones(&rx);

        if confirmados >= QUORUM {
            println!(
                "Coordinador: quorum alcanzado ({confirmados}/{REPLICAS}). Escritura EXITOSA."
            );
        } else {
            println!(
                "Coordinador: quorum alcanzado ({confirmados}/{REPLICAS}). Escritura FALLIDA."
            );
        }
    }
}

fn main() {
    escritura_con_quorum();
}
