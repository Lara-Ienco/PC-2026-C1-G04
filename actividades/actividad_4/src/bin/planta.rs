use futures::future::join_all;
use std::sync::{Arc, Mutex};
use tokio::{sync::Semaphore, task::JoinHandle};

const MAX_BOTELLAS: usize = 50;
const N: usize = 10;
const M: usize = 10;
const K: usize = 10;

#[derive(Copy, Clone)]
enum Botella {
    Vacia,
    Llena,
}

impl Botella {
    fn llenar(&mut self) {
        *self = Botella::Llena;
    }
}

struct Cinta {
    buffer: Mutex<[Option<Botella>; MAX_BOTELLAS]>,
}

impl Cinta {
    fn new() -> Cinta {
        let buffer = Mutex::new([None; MAX_BOTELLAS]);
        Cinta { buffer }
    }

    fn colocar_botella(&self, botella: Botella) -> Result<(), String> {
        let mut buffer = self.buffer.lock().map_err(|_| "Error al obtener lock")?;

        buffer
            .iter_mut()
            .find(|slot| slot.is_none())
            .map(|slot| *slot = Some(botella))
            .ok_or_else(|| "No hay espacio para la botella".to_string())
    }

    fn sacar_botella_vacia(&self) -> Result<Botella, String> {
        let mut buffer = self.buffer.lock().map_err(|_| "Error al obtener lock")?;

        let botella = buffer
            .iter_mut()
            .find(|hueco| matches!(hueco, Some(Botella::Vacia)))
            .and_then(|hueco| hueco.take())
            .ok_or("No se encontró ninguna botella vacía")?;

        Ok(botella)
    }

    fn sacar_botella_llena(&self) -> Result<Botella, String> {
        let mut buffer = self.buffer.lock().map_err(|_| "Error al obtener lock")?;

        let botella = buffer
            .iter_mut()
            .find(|hueco| matches!(hueco, Some(Botella::Llena)))
            .and_then(|hueco| hueco.take())
            .ok_or("No se encontró ninguna botella vacía")?;

        Ok(botella)
    }
}

fn empaquetar(botella: Botella) {
    // Empaquetado
}

async fn soplador(
    cinta: Arc<Cinta>,
    semaforo_huecos: Arc<Semaphore>,
    semaforo_botellas_vacias: Arc<Semaphore>,
) -> Result<(), String> {
    loop {
        let botella = Botella::Vacia;

        let permiso = semaforo_huecos
            .acquire()
            .await
            .map_err(|_| "Error al adquirir semáforo")?;
        permiso.forget();

        cinta.colocar_botella(botella)?;

        semaforo_botellas_vacias.add_permits(1);
    }
}

async fn llenador(
    cinta: Arc<Cinta>,
    semaforo_botellas_vacias: Arc<Semaphore>,
    semaforo_botellas_llenas: Arc<Semaphore>,
) -> Result<(), String> {
    loop {
        let permiso = semaforo_botellas_vacias
            .acquire()
            .await
            .map_err(|_| "Error al adquirir semáforo")?;
        permiso.forget();

        let mut botella = cinta.sacar_botella_vacia()?;
        botella.llenar();
        cinta.colocar_botella(botella)?;

        semaforo_botellas_llenas.add_permits(1);
    }
}

async fn empaquetador(
    cinta: Arc<Cinta>,
    semaforo_huecos: Arc<Semaphore>,
    semaforo_botellas_llenas: Arc<Semaphore>,
) -> Result<(), String> {
    loop {
        let permiso = semaforo_botellas_llenas
            .acquire()
            .await
            .map_err(|_| "Error al adquirir semáforo")?;
        permiso.forget();

        let botella = cinta.sacar_botella_llena()?;
        empaquetar(botella);

        semaforo_huecos.add_permits(1);
    }
}

#[tokio::main]
pub async fn main() {
    let cinta = Arc::new(Cinta::new());

    let semaforo_huecos = Arc::new(Semaphore::new(50));
    let semaforo_botellas_vacias = Arc::new(Semaphore::new(0));
    let semaforo_botellas_llenas = Arc::new(Semaphore::new(0));

    let mut procesos: Vec<JoinHandle<Result<(), String>>> = Vec::new();

    for _ in 0..N {
        let cinta = cinta.clone();
        let semaforo_huecos = semaforo_huecos.clone();
        let semaforo_botellas_vacias = semaforo_botellas_vacias.clone();

        let proceso = tokio::spawn(async move {
            soplador(cinta, semaforo_huecos, semaforo_botellas_vacias).await
        });
        procesos.push(proceso);
    }

    for _ in 0..M {
        let cinta = cinta.clone();
        let semaforo_botellas_vacias = semaforo_botellas_vacias.clone();
        let semaforo_botellas_llenas = semaforo_botellas_llenas.clone();

        let proceso = tokio::spawn(async move {
            llenador(cinta, semaforo_botellas_vacias, semaforo_botellas_llenas).await
        });
        procesos.push(proceso);
    }

    for _ in 0..K {
        let cinta = cinta.clone();
        let semaforo_huecos = semaforo_huecos.clone();
        let semaforo_botellas_llenas = semaforo_botellas_llenas.clone();

        let proceso = tokio::spawn(async move {
            empaquetador(cinta, semaforo_huecos, semaforo_botellas_llenas).await
        });
        procesos.push(proceso);
    }

    join_all(procesos).await;
}
