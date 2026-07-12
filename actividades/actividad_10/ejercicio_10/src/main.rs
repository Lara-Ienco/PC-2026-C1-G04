use std::{
    sync::mpsc::{Receiver, Sender, channel},
    thread::{sleep, spawn},
    time::{Duration, Instant},
};

const TIMEOUT: Duration = Duration::from_secs(1);
const CANTIDAD_PARTICIPANTES: usize = 3;
const SLEEP_TIME: Duration = Duration::from_millis(100);

#[derive(Clone)]
/// Mensajes que el coordinador puede enviar a los participantes
enum CoordinatorMessage {
    /// El coordinador le pide a los participantes que se preparen para la transacción
    Prepare,
    /// El coordinador le indica a los participantes que deben hacer commit de la transacción
    Commit,
    /// El coordinador le indica a los participantes que deben hacer rollback de la transacción
    Rollback,
}

#[derive(Clone)]
/// Mensajes que los participantes pueden enviar al coordinador
enum ParticipantResponse {
    /// El participante vota que sí a la transacción
    VoteYes,
    /// El participante vota que no a la transacción
    #[allow(dead_code)]
    VoteNo,
}

fn crear_participante(
    id: usize,
    tx_coordinador: Sender<ParticipantResponse>,
) -> Sender<CoordinatorMessage> {
    let (tx, rx) = channel();

    spawn(move || {
        // 1. Cada participante espera el mensaje de Prepare del coordinador
        if matches!(rx.recv(), Ok(CoordinatorMessage::Prepare))
            && tx_coordinador.send(ParticipantResponse::VoteYes).is_err()
        {
            println!("[Participante {id}] No se pudo enviar el voto al coordinador.");
            return;
        }

        // 2. Cada participante espera la decision final del coordinador (Commit o Rollback)
        match rx.recv() {
            Ok(CoordinatorMessage::Commit) => {
                println!("[Participante {id}] Se realizó un commit.");
            }
            Ok(CoordinatorMessage::Rollback) => {
                println!("[Participante {id}] Se realizó un rollback.");
            }
            _ => {
                println!("[Participante {id}] El mensaje recibido es desconocido.");
            }
        }
    });
    tx
}

fn enviar_prepare(tx_list: &[Sender<CoordinatorMessage>]) {
    for tx in tx_list {
        if tx.send(CoordinatorMessage::Prepare).is_err() {
            println!("[Coordinador] No se pudo enviar el mensaje de Prepare a un participante.");
        }
    }
}

fn recolectar_votos(rx: &Receiver<ParticipantResponse>) -> usize {
    let mut votos_si = 0;
    let start_time = Instant::now();

    // Esperamos los votos de los participantes con el timeout global
    while votos_si < CANTIDAD_PARTICIPANTES {
        let elapsed = start_time.elapsed();

        let Some(time_left) = TIMEOUT.checked_sub(elapsed) else {
            break;
        };
        // Esperamos la respuesta con el tiempo que nos queda del timeout global
        match rx.recv_timeout(time_left) {
            Ok(ParticipantResponse::VoteYes) => {
                println!("[Coordinador] Ha recibido un voto SI.");
                votos_si += 1;
            }
            Ok(ParticipantResponse::VoteNo) => {
                println!("[Coordinador] Ha recibido un voto NO.");
                break;
            }
            Err(_) => {
                println!("[Coordinador] Timeout alcanzado antes de recibir todos los votos.");
                break;
            }
        }
    }
    votos_si
}

fn tomar_decision(votos_si: usize) -> CoordinatorMessage {
    // Decide si hace commit o rollback dependiendo de los votos recibidos y si hubo timeout
    if votos_si == CANTIDAD_PARTICIPANTES {
        println!("[Coordinador] Todos los participantes votaron SI. Se realiza un COMMIT.");
        CoordinatorMessage::Commit
    } else {
        println!(
            "[Coordinador] Algún participante ha votado NO o hubo timeout. Se realiza un ROLLBACK."
        );
        CoordinatorMessage::Rollback
    }
}

fn enviar_decision(tx_list: &[Sender<CoordinatorMessage>], decision: &CoordinatorMessage) {
    // Le envía la decision a todos los participantes
    for tx in tx_list {
        if tx.send(decision.clone()).is_err() {
            println!("[Coordinador] No se pudo enviar la decisión a un participante.");
        }
    }
}

fn main() {
    let (tx_coordinador, rx_coordinador) = channel();

    // 1. Creamos los hilos de los participantes
    let mut tx_participantes = Vec::new();
    for id in 1..=CANTIDAD_PARTICIPANTES {
        tx_participantes.push(crear_participante(id, tx_coordinador.clone()));
    }
    drop(tx_coordinador);

    // 2. Enviamos el mensaje de Prepare a todos los participantes
    enviar_prepare(&tx_participantes);

    // 3. Esperamos los votos de los participantes con el timeout global
    let votos_si = recolectar_votos(&rx_coordinador);

    // 4. Decidimos si hacer commit o rollback
    let decision = tomar_decision(votos_si);

    // 5. Enviamos la decision a todos los participantes
    enviar_decision(&tx_participantes, &decision);

    // Pequeño sleep para que los hilos impriman antes de que termine el principal
    sleep(SLEEP_TIME);
}
