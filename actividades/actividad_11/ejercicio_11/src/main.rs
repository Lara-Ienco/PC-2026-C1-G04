use tokio::sync::mpsc;

const NUMERO_DE_ELEMENTOS: usize = 10;
const CAPACIDAD: usize = 3;

#[tokio::main]
async fn main() {
    // Creamos un canal con capacidad limitada
    let (tx, mut rx) = mpsc::channel::<usize>(CAPACIDAD);
    // Luego, creamos la tarea Productora
    let producer_handle = tokio::spawn(async move {
        for i in 1..=NUMERO_DE_ELEMENTOS {
            println!("[Productor] Estoy produciendo: {}", i);

            if let Err(e) = tx.send(i).await {
                println!("[Productor] Error: el canal se cerró. {}", e);
                break;
            }
            /*
            ----------------------------------------------------------------------------------------------------
            A) ¿En qué momento exacto la tarea Productora queda suspendida asincrónicamente?
            La tarea Productora se va a suspender en el metodo send(i); cuando ya hay 3 elementos en el canal
            (porque asi lo definimos en CAPACIDAD), no puede enviar mas elementos hasta que el consumidor
            reciba alguno de los elementos enviados antes y libere espacio en el canal.
            Cuando esto pasa, la tarea Productora se reanuda y puede enviar el siguiente elemento.
            ----------------------------------------------------------------------------------------------------
            */
            println!("[Productor] Estoy enviando: {}", i);
        }
        println!("[Productor] Producción finalizada");
    });
    // Y la tarea Consumidora
    let consumer_handle = tokio::spawn(async move {
        for _ in 1..=NUMERO_DE_ELEMENTOS {
            match rx.recv().await {
                Some(value) => {
                    /*
                    ----------------------------------------------------------------------------------------------------
                    B) ¿Qué acción del Consumidor habilita nuevamente la ejecución del Productor?
                    La accion del Consumidor que habilita nuevamente la ejecución del Productor es el metodo recv().
                    Cuando el consumidor recibe un elemento del canal, libera espacio en el canal,
                    lo que permite que el productor pueda seuir enviando elementos.
                    ----------------------------------------------------------------------------------------------------
                    */
                    println!("[Consumidor] Estoy recibiendo: {}", value);
                }
                None => {
                    println!("[Consumidor] Canal cerrado, finalizando");
                    break;
                }
            }
        }
        println!(
            "[Consumidor] Consumo limitado a {} elementos, finalizando",
            NUMERO_DE_ELEMENTOS
        );
    });
    // Finalmente, esperamos a que ambas tareas terminen
    let _ = tokio::try_join!(producer_handle, consumer_handle);
}
/*
----------------------------------------------------------------------------------------------------
C) Explicá cómo se interpreta este comportamiento dinámico mediante el intercambio de tokens
(P_vacíos y P_llenos) en la Red de Petri equivalente.

En una Red de Petri equivalente, podemos interpretar el comportamiento dinámico del Productor y el
Consumidor mediante el uso de tokens que representan los recursos disponibles en el sistema.
P_vacíos representa los espacios vacíos en el canal, mientras que P_llenos representa los elementos
almacenados en el canal.
Cuando el Productor envía un elemento al canal, se consume un token de P_vacíos y se genera un token
en P_llenos, indicando que hay un elemento disponible para el Consumidor.

Entonces, cuando el canal se llena (CAPACIDAD = 3), el lugar P_vacíos se queda sin tokens. Esto provoca
que la transición de producir quede deshabilitada, lo que se traduce en la suspensión asincrónica del
.await en el código. En el momento en que el Consumidor recibe un elemento con recv(), consume un token
de P_llenos y devuelve un token a P_vacíos. Este nuevo token habilita de nuevo la transición del
Productor, permitiéndole reanudar su ejecución y enviar el siguiente elemento. Todo esto manteniendo
siempre constante la suma total de tokens (P_vacíos + P_llenos = 3).
----------------------------------------------------------------------------------------------------
*/
