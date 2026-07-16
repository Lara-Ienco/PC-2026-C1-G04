use std::{
    sync::mpsc::{Receiver, Sender, channel},
    thread::{spawn, JoinHandle, sleep},
    time::{Duration, Instant},
};

const TIMEOUT: Duration = Duration::from_secs(5);
const CANTIDAD_PARTICIPANTES: usize = 3;

#[derive(Clone, Copy, PartialEq)]
enum Scenario {
    ParticipantVotesNo,
    ParticipantDelay,
    CoordinatorForcesRollback,
}

#[derive(Clone)]
enum CoordinatorMessage {
    Prepare,
    Commit,
    Rollback,
}

#[derive(Clone)]
enum ParticipantResponse {
    VoteYes,
    VoteNo,
}

fn crear_participante(
    id: usize,
    tx_coordinador: Sender<ParticipantResponse>,
    scenario: Scenario,
) -> (Sender<CoordinatorMessage>, JoinHandle<()>) {
    let (tx, rx) = channel();

    let handle = spawn(move || {
        // 1. Cada participante espera el mensaje de Prepare
        let Ok(CoordinatorMessage::Prepare) = rx.recv() else {
            return;
        };

        // Simulación de comportamientos según el escenario
        let voto = match scenario {
            // Caso 11.a: El participante 2 vota NO
            Scenario::ParticipantVotesNo if id == 2 => {
                println!("[Participante {id}] Votando NO por diseño de escenario.");
                ParticipantResponse::VoteNo
            }
            // Caso 11.b: El participante 3 demora más de 5 segundos (> 5s)
            Scenario::ParticipantDelay if id == 3 => {
                println!("[Participante {id}] Demorando respuesta por 6 segundos...");
                sleep(Duration::from_secs(6));
                println!("[Participante {id}] Enviando voto tarde (YES).");
                ParticipantResponse::VoteYes
            }
            // Por defecto votan YES
            _ => {
                println!("[Participante {id}] Votando YES.");
                ParticipantResponse::VoteYes
            }
        };

        if tx_coordinador.send(voto).is_err() {
            println!("[Participante {id}] No se pudo enviar el voto al coordinador.");
            return;
        }

        // 2. Espera la decisión final
        match rx.recv() {
            Ok(CoordinatorMessage::Commit) => {
                println!("[Participante {id}] Se realizó un COMMIT.");
            }
            Ok(CoordinatorMessage::Rollback) => {
                println!("[Participante {id}] Se realizó un ROLLBACK.");
            }
            _ => {
                println!("[Participante {id}] El mensaje recibido es desconocido o el canal se cerró.");
            }
        }
    });
    (tx, handle)
}

fn enviar_prepare(tx_list: &[Sender<CoordinatorMessage>]) {
    for tx in tx_list {
        let _ = tx.send(CoordinatorMessage::Prepare);
    }
}

fn recolectar_votos(rx: &Receiver<ParticipantResponse>) -> (usize, bool) {
    let mut votos_si = 0;
    let mut hubo_no = false;
    let start_time = Instant::now();

    while votos_si < CANTIDAD_PARTICIPANTES && !hubo_no {
        let elapsed = start_time.elapsed();
        let Some(time_left) = TIMEOUT.checked_sub(elapsed) else {
            println!("[Coordinador] Timeout alcanzado antes de procesar más votos.");
            break;
        };

        match rx.recv_timeout(time_left) {
            Ok(ParticipantResponse::VoteYes) => {
                println!("[Coordinador] Ha recibido un voto SI.");
                votos_si += 1;
            }
            Ok(ParticipantResponse::VoteNo) => {
                println!("[Coordinador] Ha recibido un voto NO.");
                hubo_no = true;
                break;
            }
            Err(_) => {
                println!("[Coordinador] Timeout alcanzado esperando respuestas.");
                break;
            }
        }
    }
    (votos_si, hubo_no)
}

fn tomar_decision(votos_si: usize, hubo_no: bool, scenario: Scenario) -> CoordinatorMessage {
    // Caso 11.c: El coordinador decide hacer Rollback arbitrariamente aunque todos votaran YES
    if scenario == Scenario::CoordinatorForcesRollback {
        println!("[Coordinador] Escenario forzado: El Coordinador decide unilateralmente hacer un ROLLBACK.");
        return CoordinatorMessage::Rollback;
    }

    if votos_si == CANTIDAD_PARTICIPANTES && !hubo_no {
        println!("[Coordinador] Todos los participantes votaron SI. Se realiza un COMMIT.");
        CoordinatorMessage::Commit
    } else {
        println!(
            "[Coordinador] Algún participante votó NO o hubo un timeout. Se realiza un ROLLBACK."
        );
        CoordinatorMessage::Rollback
    }
}

fn enviar_decision(tx_list: &[Sender<CoordinatorMessage>], decision: &CoordinatorMessage) {
    for tx in tx_list {
        let _ = tx.send(decision.clone());
    }
}

fn ejecutar_simulacion(scenario: Scenario, nombre: &str) {
    println!("\n=== SIMULANDO ESCENARIO: {nombre} ===");
    let (tx_coordinador, rx_coordinador) = channel();

    let mut tx_participantes = Vec::new();
    let mut participant_handles = Vec::new();
    for id in 1..=CANTIDAD_PARTICIPANTES {
        let (tx, handle) = crear_participante(id, tx_coordinador.clone(), scenario);
        tx_participantes.push(tx);
        participant_handles.push(handle);
    }
    drop(tx_coordinador);

    enviar_prepare(&tx_participantes);

    let (votos_si, hubo_no) = recolectar_votos(&rx_coordinador);

    let decision = tomar_decision(votos_si, hubo_no, scenario);

    enviar_decision(&tx_participantes, &decision);

    for handle in participant_handles {
        if handle.join().is_err() {
            println!("[Coordinador] Un participante terminó con pánico.");
        }
    }
}

fn main() {
    ejecutar_simulacion(Scenario::ParticipantVotesNo, "a) Un participante responde NO");
    ejecutar_simulacion(Scenario::ParticipantDelay, "b) Un participante demora más de 5 segundos");
    ejecutar_simulacion(Scenario::CoordinatorForcesRollback, "c) El Coordinador decide hacer Rollback");
}