use rand::random_range;
use std::{
    collections::HashMap,
    sync::mpsc::{Receiver, Sender, channel},
    thread::spawn,
};
use std::thread::sleep;
use std::time::Duration;

const NUM_CUENTAS: u32 = 10;
const HILOS_CLIENTES: usize = 100;
const MIN_SALDO_INICIAL: f64 = 1000.0;
const MAX_SALDO_INICIAL: f64 = 5000.0;
const MIN_MONTO: f64 = 100.0;
const MAX_MONTO: f64 = 1000.0;

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
    let (tx, rx) = channel();

    // lanzo el hilo del actor de la cuenta
    spawn(move || {
        if let Err(error) = hilo_actor_cuenta(id, saldo_inicial, &rx) {
            eprintln!("[Cuenta {id}] {error}");
        }
    });
    tx
}

fn hilo_actor_cuenta(
    id: u32,
    saldo_inicial: f64,
    rx: &Receiver<MensajeCuenta>,
) -> Result<(), String> {
    let mut saldo: f64 = saldo_inicial;

    while let Ok(mensaje) = rx.recv() {
        match mensaje {
            // si se quiere depositar, aumento el saldo y respondo con el nuevo saldo
            MensajeCuenta::Depositar { monto } => {
                saldo += monto;
                println!("[Cuenta {id}] Depositado ${monto:.2}. Nuevo saldo: ${saldo:.2}");
            }
            // Si se quiere extraer, me fijo ambos casos: si el monto a extraer es menor o igual al saldo, hago la extracción y respondo con éxito. Si no, respondo que no hay fondos suficientes
            MensajeCuenta::Extraer {
                monto,
                canal_respuesta,
            } => {
                if monto <= saldo {
                    saldo -= monto;
                    println!("[Cuenta {id}] Extraído ${monto:.2}. Nuevo saldo: ${saldo:.2}");
                    canal_respuesta
                        .send(RespuestaExtraccion::Exito)
                        .map_err(|error| error.to_string())?;
                } else {
                    println!(
                        "[Cuenta {id}] No se pudo extraer ${monto:.2}. Saldo insuficiente: ${saldo:.2}",
                    );
                    canal_respuesta
                        .send(RespuestaExtraccion::FondosInsuficientes)
                        .map_err(|error| error.to_string())?;
                }
            }
            // si se quiere consultar el saldo, respondo con el saldo actual
            MensajeCuenta::ConsultarSaldo { canal_respuesta } => {
                println!("[Cuenta {id}] Consultando saldo. Saldo actual: ${saldo:.2}");
                canal_respuesta
                    .send(saldo)
                    .map_err(|error| error.to_string())?;
            }
        }
    }
    println!("[Cuenta {id}] Actor de cuenta finalizado.");
    Ok(())
}

fn crear_actor_gestor(cuentas: HashMap<u32, Sender<MensajeCuenta>>) -> Sender<MensajeGestor> {
    let (tx, rx) = channel();

    spawn(move || {
        if let Err(error) = hilo_actor_gestor(&cuentas, &rx) {
            eprintln!("[Gestor] {error}");
        }
    });
    tx
}

fn hilo_actor_gestor(
    cuentas: &HashMap<u32, Sender<MensajeCuenta>>,
    rx: &Receiver<MensajeGestor>,
) -> Result<(), String> {
    let (tx_repuesta, rx_respuesta) = channel();
    let mut estadisticas = (0, 0);

    while let Ok(mensaje) = rx.recv() {
        match mensaje {
            MensajeGestor::IniciarTransferencia {
                origen_id,
                destino_id,
                monto,
            } => {
                println!(
                    "[Gestor] Iniciando transferencia de {origen_id} a {destino_id} por {monto}"
                );
                let tx_cuenta_origen = cuentas
                    .get(&origen_id)
                    .ok_or("Cuenta de origen no encontrada")?;
                tx_cuenta_origen
                    .send(MensajeCuenta::Extraer {
                        monto,
                        canal_respuesta: tx_repuesta.clone(),
                    })
                    .map_err(|error| error.to_string())?;
                let respuesta_extraccion =
                    rx_respuesta.recv().map_err(|error| error.to_string())?;

                if matches!(
                    respuesta_extraccion,
                    RespuestaExtraccion::FondosInsuficientes
                ) {
                    println!("[Gestor] Fondos insuficientes");
                    estadisticas.1 += 1;
                } else {
                    let tx_cuenta_destino = cuentas
                        .get(&destino_id)
                        .ok_or("Cuenta de destino no encontrada")?;
                    tx_cuenta_destino
                        .send(MensajeCuenta::Depositar { monto })
                        .map_err(|error| error.to_string())?;

                    println!("[Gestor] Transferencia exitosa");
                    estadisticas.0 += 1;
                }
            }
            MensajeGestor::ObtenerEstadisticas { canal_respuesta } => {
                println!("[Gestor] Enviando estadísticas");
                canal_respuesta
                    .send((estadisticas.0, estadisticas.1))
                    .map_err(|error| error.to_string())?;
            }
        }
    }
    println!("[Gestor] Gestor de cuentas finalizado.");
    Ok(())
}

fn hilo_cliente(tx: &Sender<MensajeGestor>) -> Result<(), String> {
    let origen_id = random_range(0..NUM_CUENTAS);
    let mut destino_id = origen_id;

    while destino_id == origen_id {
        destino_id = random_range(0..NUM_CUENTAS);
    }
    let monto = random_range(MIN_MONTO..=MAX_MONTO);
    tx.send(MensajeGestor::IniciarTransferencia {
        origen_id,
        destino_id,
        monto,
    })
    .map_err(|error| error.to_string())
}

fn main() {
    let mut cuentas = HashMap::new(); // Mapa para almacenar los canales de las cuentas.
    let mut total_inicial = 0.0;

    for id in 0..NUM_CUENTAS {
        let saldo_inicial = random_range(MIN_SALDO_INICIAL..=MAX_SALDO_INICIAL).round();
        total_inicial += saldo_inicial;

        let tx_cuenta = crear_actor_cuenta(id, saldo_inicial);
        cuentas.insert(id, tx_cuenta);
    }
    println!("Total inicial en todas las cuentas: ${total_inicial:.2}");

    let tx_gestor = crear_actor_gestor(cuentas.clone());

    let mut hilos_clientes = vec![];

    for num in 0..HILOS_CLIENTES {
        let tx_gestor_clon = tx_gestor.clone();

        let handle = spawn(move || {
            if let Err(error) = hilo_cliente(&tx_gestor_clon) {
                eprintln!("[Cliente {num}] {error}");
            }
        });
        hilos_clientes.push(handle);
    }
    for handle in hilos_clientes {
        if let Err(error) = handle.join() {
            eprintln!("[Principal] {error:?}");
        }
    }
    let (tx, rx) = channel();

    if let Err(error) = tx_gestor.send(MensajeGestor::ObtenerEstadisticas {
        canal_respuesta: tx,
    }) {
        eprintln!("[Principal] {error:?}");
        return;
    }
    let estadisticas = match rx.recv() {
        Ok(estadisticas) => estadisticas,
        Err(error) => {
            eprintln!("[Principal] {error:?}");
            return;
        }
    };
    println!(
        "[Principal] Transferencias exitosas: {}. Transferencias con fondos insuficientes: {}",
        estadisticas.0, estadisticas.1
    );

    let mut total_final = 0.0;

    for (_, tx_cuenta) in cuentas {
        let (tx, rx) = channel();

        if let Err(error) = tx_cuenta.send(MensajeCuenta::ConsultarSaldo {
            canal_respuesta: tx,
        }) {
            eprintln!("[Principal] {error:?}");
        }
        let saldo = match rx.recv() {
            Ok(saldo) => saldo,
            Err(error) => {
                eprintln!("[Principal] {error:?}");
                return;
            }
        };
        total_final += saldo;
    }
    println!("[Principal] Total inicial: {total_inicial}. Total final: {total_final}");
    drop(tx_gestor); // Liberamos el canal para que termine el hilo del gestor y se liberen los canales de las cuentas
    sleep(Duration::from_secs(1)); // Esperamos a que terminen todos los hilos
}
