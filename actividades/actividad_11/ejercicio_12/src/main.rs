use tokio::sync::oneshot;
use tokio::time::{sleep, Duration};

#[derive(Debug, Clone)]
enum Message {
    Commit,
    Abort,
    Prepare,
    VoteYes,
    VoteNo,
}

#[tokio::main]
async fn main() {
    let (prepare_tx, prepare_rx) = oneshot::channel::<Message>(); // un canal para enviar el mensaje Prepare del coordinador al participante
    let (vote_tx, vote_rx) = oneshot::channel::<Message>(); // otro para enviar el mensaje VoteYes del participante al coordinador
    let (decision_tx, decision_rx) = oneshot::channel::<Message>(); // y el ultimo para enviar la decisión final del coordinador al participante
    
    let participant_handle = tokio::spawn(async move {
        // El participante espera el mensaje de Prepare del coordinador
        if let Ok(Message::Prepare) = prepare_rx.await {
            println!("[Participante] Recibiendo mensaje de Prepare del coordinador");
            
            println!("[Participante] Enviando mensaje de VoteYes al coordinador");
            let _ = vote_tx.send(Message::VoteYes); 
            
            //una vez que envio el mensaje de VoteYes, el participante queda a la espera de la decisión final del coordinador
            println!("[Participante] Esperando mensaje de Commit o Abort del coordinador");
            
            // Al cerrarse abruptamente el canal decision_tx desde el coordinador, await devuelve Err(RecvError)
            match decision_rx.await {
                Ok(msg) => println!("[Participante] Decisión final recibida: {:?}", msg),
                Err(_) => println!("[Participante] DEADLOCK: El canal oneshot se cerró abruptamente. El Coordinador cayó y el Participante queda en incertidumbre."),
            }
        }
    });
    
    let coordinator_handle = tokio::spawn(async move {
        // El coordinador envia el mensaje de Prepare al participante
        println!("[Coordinador] Enviando mensaje de Prepare...");
        let _ = prepare_tx.send(Message::Prepare);
        
        println!("[Coordinador] Esperando respuestas de los participantes");
        
        if let Ok(Message::VoteYes) = vote_rx.await {
            println!("[Coordinador] Voto recibido. Simulando caída del coordinador...");
            // dropeando el canal decision_tx simulamos la caída del coordinador
            drop(decision_tx); 
            
            sleep(Duration::from_secs(5)).await;
        }
    });
    
    let _ = tokio::try_join!(participant_handle, coordinator_handle);
}

a) ¿En qué estado del ciclo de vida queda detenido el proceso Participante?

El proceso Participante queda detenido en el estado de Incertidumbre (READY). Este estado ocurre exactamente después de haber emitido su voto afirmativo (VoteYes) y mientras se encuentra bloqueado esperando la decisión final (Commit o Abort) por parte del Coordinador.

b) ¿Por qué la tarea no puede avanzar ni finalizar libremente?
Porque la versión básica del protocolo Two-Phase Commit (2PC) es estructuralmente bloqueante ya que una vez que un Participante emite un voto afirmativo, cede completamente su autonomía para decidir sobre la transacción. No puede tomar la decisión de abortar (porque el Coordinador podría haber decidido hacer commit justo antes de caer) ni de confirmar (porque otro participante podría haber votado que no). Al cerrarse abruptamente el canal de comunicación, el proceso queda suspendido y no puede avanzar de forma segura.

c) ¿Cómo se representa este estado de bloqueo indefinido en el Grafo de Alcanzabilidad de una Red de Petri?

Se representa como un nodo terminal. En el Grafo de Alcanzabilidad, esto corresponde a un nodo del cual no sale ninguna arista. Significa que, bajo esa distribución de tokens (donde el token que representa el mensaje de respuesta del Coordinador nunca llegará a su plaza correspondiente), no existe ninguna transición habilitada que pueda ser disparada para que el sistema evolucione hacia otro estado.

d) ¿Qué propiedad fundamental del sistema se preserva a pesar de que el proceso quedó
bloqueado?

Se preserva:
Consistencia: el sistema garantiza que los datos no queden en un estado corrupto o divergente, evitando el peor escenario posible donde algunos nodos aplican la transacción y otros la descartan.
