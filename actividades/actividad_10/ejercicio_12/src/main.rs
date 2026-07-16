use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::PathBuf,
    sync::mpsc::{Receiver, Sender, channel},
    thread::{spawn, JoinHandle},
    time::{Duration, Instant},
};

const TIMEOUT: Duration = Duration::from_secs(1);
const CANTIDAD_PARTICIPANTES: usize = 3;

#[derive(Clone)]
enum CoordinatorMessage {
    Prepare,
    Commit,
    Rollback,
}

#[derive(Clone)]
enum ParticipantResponse {
    VoteYes,
    #[allow(dead_code)]
    VoteNo,
}

/// Estructura para manejar el Write-Ahead Log (WAL) de cada participante
struct WriteAheadLog {
    file_path: PathBuf,
}

/// Implementación de métodos para el Write-Ahead Log (WAL)
impl WriteAheadLog {
    fn new(participant_id: usize) -> Self {
        let file_path = PathBuf::from(format!("participant_{}_wal.log", participant_id));
        // limpiamos logs viejos para iniciar limpios en cada ejecución
        let _ = fs::remove_file(&file_path);
        Self { file_path }
    }

    fn write_entry(&self, entry: &str) {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_path)
            .expect("Error al abrir el archivo WAL");

        writeln!(file, "{}", entry).expect("Error al escribir en el WAL");
        // sincronizamos el archivo con el disco para asegurar que los datos se escriban inmediatamente
        file.sync_all().expect("Error al sincronizar el WAL con el disco");
    }
}

fn crear_participante(
    id: usize,
    tx_coordinador: Sender<ParticipantResponse>,
) -> (Sender<CoordinatorMessage>, JoinHandle<()>) {
    let (tx, rx) = channel();

    let handle = spawn(move || {
        // inicializamos el archivo WAL del participante
        let wal = WriteAheadLog::new(id);

        // 1. cada participante espera el mensaje de Prepare del coordinador
        if matches!(rx.recv(), Ok(CoordinatorMessage::Prepare)) {
            
            // --- REGISTRO PREPARED ANTES DE ENVIAR EL VOTO YES ---
            wal.write_entry("Prepared");
            println!("[Participante {id}] Registrado 'Prepared' en WAL. Enviando voto YES.");

            if tx_coordinador.send(ParticipantResponse::VoteYes).is_err() {
                println!("[Participante {id}] No se pudo enviar el voto al coordinador.");
                return;
            }
        }

        // 2. cada participante espera la decisión final del coordinador
        match rx.recv() {
            Ok(CoordinatorMessage::Commit) => {
                // --- REGISTRO COMMIT ANTES DE EJECUTAR LA CONFIRMACIÓN ---
                wal.write_entry("Commit");
                println!("[Participante {id}] Registrado 'Commit' en WAL. Se realizó un commit.");
            }
            Ok(CoordinatorMessage::Rollback) => {
                // --- REGISTRO ROLLBACK ANTES DE ABORTAR ---
                wal.write_entry("Rollback");
                println!("[Participante {id}] Registrado 'Rollback' en WAL. Se realizó un rollback.");
            }
            _ => {
                println!("[Participante {id}] El mensaje recibido es desconocido.");
            }
        }
    });
    (tx, handle)
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

    while votos_si < CANTIDAD_PARTICIPANTES {
        let elapsed = start_time.elapsed();

        let Some(time_left) = TIMEOUT.checked_sub(elapsed) else {
            break;
        };
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
    for tx in tx_list {
        if tx.send(decision.clone()).is_err() {
            println!("[Coordinador] No se pudo enviar la decisión a un participante.");
        }
    }
}

fn main() {
    println!("=== INICIANDO SIMULACIÓN CON WRITE-AHEAD LOG (WAL) ===");
    let (tx_coordinador, rx_coordinador) = channel();

    let mut tx_participantes = Vec::new();
    let mut participant_handles = Vec::new();
    for id in 1..=CANTIDAD_PARTICIPANTES {
        let (tx, handle) = crear_participante(id, tx_coordinador.clone());
        tx_participantes.push(tx);
        participant_handles.push(handle);
    }
    drop(tx_coordinador);

    enviar_prepare(&tx_participantes);

    let votos_si = recolectar_votos(&rx_coordinador);

    let decision = tomar_decision(votos_si);

    enviar_decision(&tx_participantes, &decision);

    for handle in participant_handles {
        if handle.join().is_err() {
            println!("[Coordinador] Un participante terminó con pánico.");
        }
    }
    println!("=== SIMULACIÓN FINALIZADA CON ÉXITO ===");
    println!("Los archivos 'participant_*_wal.log' han sido actualizados en disco.");
}