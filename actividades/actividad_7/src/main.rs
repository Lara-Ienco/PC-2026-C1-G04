use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use rand::Rng;
use std::collections::HashMap;

const TOTAL_INICIAL: f64 = 0.0;
const NUM_CUENTAS: u32 = 10;
const HILOS_CLIENTES: usize = 100;
const MIN_SALDO_INICIAL: f64 = 1000.0;
const MAX_SALDO_INICIAL: f64 = 5000.0;

#[derive(Debug)]
enum RespuestaExtraccion {
    /// Extracción exitosa
    Exito,
    /// No se pudo completar la extracción por falta de fondos
    FondosInsuficientes,
}

#[derive(Debug)]
enum MensajeCuenta {
    /// Depositar una cantidad de dinero en la cuenta
    Depositar { monto: f64 },
    /// Extraer una cantidad de dinero de la cuenta, con un canal para recibir la respuesta
    Extraer {
        monto: f64,
        canal_respuesta: Sender<RespuestaExtraccion>,
    },
    /// Consultar el saldo actual de la cuenta, con un canal para recibir la respuesta
    ConsultarSaldo { canal_respuesta: Sender<f64> },
}

#[derive(Debug)]
enum MensajeGestor {
    /// Iniciar una transferencia de dinero entre dos cuentas, con un canal para recibir la respuesta
    IniciarTransferencia {
        origen_id: u32,
        destino_id: u32,
        monto: f64,
    },
    /// Obtener estadísticas de las transferencias realizadas, con un canal para recibir la respuesta
    ObtenerEstadisticas { canal_respuesta: Sender<(u32, u32)> }, // Tupla (Exitosas, Fallidas)
}

fn crear_actor_cuenta(id: u32, saldo_inicial: f64) -> Sender<MensajeCuenta> {
    let (tx, rx): (Sender<MensajeCuenta>, Receiver<MensajeCuenta>) = mpsc::channel();

    // lanzo el hilo del actor de la cuenta
    thread::spawn(move || {
        let mut saldo: f64 = saldo_inicial;

        while let Ok(mensaje) = rx.recv() {
            match mensaje {
                // si se quiere depositar, aumento el saldo y respondo con el nuevo saldo
                MensajeCuenta::Depositar { monto } => {
                    saldo += monto;
                    println!(
                        "[Cuenta {}] Depositado ${:.2}. Nuevo saldo: ${:.2}",
                        id, monto, saldo
                    );
                }
                // si se quiere extraer, me fijo ambos casos: si el monto a extraer es menor o igual al saldo, hago la extracción y respondo con éxito. Si no, respondo que no hay fondos suficientes
                MensajeCuenta::Extraer {
                    monto,
                    canal_respuesta,
                } => {
                    if monto <= saldo {
                        saldo -= monto;
                        println!(
                            "[Cuenta {}] Extraido ${:.2}. Nuevo saldo: ${:.2}",
                            id, monto, saldo
                        );
                        let _ = canal_respuesta.send(RespuestaExtraccion::Exito);
                    } else {
                        println!(
                            "[Cuenta {}] No se pudo extraer ${:.2}. Saldo insuficiente: ${:.2}",
                            id, monto, saldo
                        );
                        let _ = canal_respuesta.send(RespuestaExtraccion::FondosInsuficientes);
                    }
                }
                // si se quiere consultar el saldo, respondo con el saldo actual
                MensajeCuenta::ConsultarSaldo { canal_respuesta } => {
                    println!(
                        "[Cuenta {}] Consultando saldo. Saldo actual: ${:.2}",
                        id, saldo
                    );
                    let _ = canal_respuesta.send(saldo);
                }
            }
        }
        println!("[Cuenta {}] Actor de cuenta finalizado.", id);
    });
    tx
}


fn crear_actor_gestor(cuentas: HashMap<u32, Sender<MensajeCuenta>>) -> Sender<MensajeGestor> {
    let (tx, rx): (Sender<MensajeGestor>, Receiver<MensajeGestor>) = mpsc::channel();
    //hilo....
    //y aca toda la logica
    tx
}
fn main() {
    let mut rng = rand::thread_rng();
    let mut cuentas = HashMap::new(); // Mapa para almacenar los canales de las cuentas. o un vector de tuplas??? VER
    let mut total_inicial = TOTAL_INICIAL;

    for id in 1..=NUM_CUENTAS {
        let saldo_inicial = rng.gen_range(MIN_SALDO_INICIAL..MAX_SALDO_INICIAL).round();
        total_inicial += saldo_inicial;

        let tx_cuenta = crear_actor_cuenta(id, saldo_inicial);
        cuentas.insert(id, tx_cuenta);
    }

    println!("Total inicial en todas las cuentas: ${:.2}", total_inicial);
    
    // CREAR GESTOR Y HACER TRANSFERENCIAS ENTRE CUENTAS
    let tx_gestor = crear_actor_gestor(cuentas.clone()); //VER DE CAMBIAR CUANDO LO IMPLEMENTEN, ES SOLO UNA IDEA

    let mut hilos_clientes =  vec![];
    for _ in 0..HILOS_CLIENTES {
        // Crear hilos que realicen transferencias entre cuentas
        let tx_gestor_clon = tx_gestor.clone();
        let handle = thread::spawn(move || {
            // aca tendriamos que tipo generar un origen y destino id random (entra 1 y 10, pq son 10 cuentas)
            // y hacer algo edl estido MensajeGestor::IniciarTransferenciaal tx_gestor_clonado
        });
        hilos_clientes.push(handle);
    }

    for handle in hilos_clientes {
        let _ = handle.join();
    }

    // Chequeos?
    // consultar saldo de cada cuenta y sumar para ver que el total final es igual al total inicial
    // consultar estadísticas de transferencias realizadas
    // etc
}
