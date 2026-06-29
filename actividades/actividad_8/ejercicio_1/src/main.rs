use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::{Duration, Instant};


const REPLICAS: usize = 5;
const QUORUM: usize = 3;
const TIMEOUT: Duration = Duration::from_secs(1);
const RONDAS: usize = 3;

const VALOR: i32 = 42;
const FALLA: f64 = 0.3;
const MIN_LATENCIA: u64 = 100;
const MAX_LATENCIA: u64 = 800;

fn lanzar_replica(id: usize, tx_clone: Sender<(usize, i32)>, tx_envios: &mut Vec<Sender<i32>>) {
    let (tx_envio, rx_envio) = mpsc::channel::<i32>();
    tx_envios.push(tx_envio);
    thread::spawn(move || {
        // El nodo falla aleatoriamente
        if rand::random_bool(FALLA){
            println!("Réplica {}: falla simulada, no respondo.",id);
            return; // Terminamos el hilo sin replicar nada.
        }
        
        // El nodo confirmar exitosamente (con latencia variable)
        if let Ok(msg) = rx_envio.recv(){
            let latencia = Duration::from_millis(rand::random_range(MIN_LATENCIA..=MAX_LATENCIA));
            thread::sleep(latencia);
            println!("Réplica {}: recibí valor {}, confirmado. (latencia: {}ms)", id, msg, latencia.as_millis());
            let _ = tx_clone.send((id,msg));
        }
    });
}

fn esperar_confirmaciones(rx: Receiver<(usize, i32)>) -> usize{
    let mut confirmaciones = 0;
    let tiempo_inicial = Instant::now();
    loop {
        let tiempo_transcurrido = tiempo_inicial.elapsed();
        
        if tiempo_transcurrido >= TIMEOUT {
            println!("Se acabo el timeout");
            return confirmaciones;
        }
        
        let tiempo_restante = TIMEOUT - tiempo_transcurrido; 
        match rx.recv_timeout(tiempo_restante) {
            Ok((id, _)) => {
                confirmaciones += 1;
                println!("Coordinador: confirmación recibida de Réplica {}.", id);
                if confirmaciones >= QUORUM {
                    return confirmaciones;
                }
            },
            Err(_) => return confirmaciones,
        }
    }
}
fn escritura_con_quorum() {
    //Loop: ejecutar 3 rondas de escritura
    for i in 1..=RONDAS{
        println!("=== Ronda {} ===", i);

        // Channel para confirmaciones
        let (tx, rx) = mpsc::channel::<(usize,i32)>();

        let msg = VALOR;       // mensaje a enviar
        let mut tx_envios: Vec<Sender<i32>>  = Vec::new(); 

        println!("Coordinador: enviando valor {} a {} réplicas...", msg, REPLICAS);

        // Lanzamos 5 threads réplicas
        for j in 1..=REPLICAS{
            lanzar_replica(j, tx.clone(), &mut tx_envios);
        }

        // Enviamos el valor a los nodos de la red
        for tx in &tx_envios {
            let _ = tx.send(msg);
        }
        drop(tx);

        // Coordinador espera confirmaciones hasta alcanzar el QUORUM
        let confirmados = esperar_confirmaciones(rx);
        if confirmados >= QUORUM {
            println!("Coordinador: quorum alcanzado ({}/{}). Escritura EXITOSA.", confirmados, REPLICAS);
        } else {
            println!("Coordinador: quorum alcanzado ({}/{}). Escritura FALLIDA.", confirmados, REPLICAS);
        }
    }
}

fn main() {
    escritura_con_quorum();
}
