use futures::future::join_all;
use std::array::from_fn;
use std::sync::Arc;
use tokio::sync::Semaphore;

const N: usize = 4;
const M: usize = 2;

fn calcular_rebotes_de_luz() {
    // Calculo de rebotes de luz
}

fn dibujar_pixel() {
    // Dibujo de pixel
}

async fn proceso(
    numero: usize,
    pareja1: usize,
    pareja2: usize,
    semaforos: Arc<[[Semaphore; N]; M]>,
) -> Result<(), String> {
    calcular_rebotes_de_luz();

    semaforos[0][pareja1].add_permits(1);
    let permiso = semaforos[0][numero]
        .acquire()
        .await
        .map_err(|error| error.to_string())?;
    permiso.forget();

    semaforos[1][pareja2].add_permits(1);
    let permiso = semaforos[1][numero]
        .acquire()
        .await
        .map_err(|error| error.to_string())?;
    permiso.forget();

    dibujar_pixel();

    Ok(())
}

#[tokio::main]
async fn main() {
    let semaforos = Arc::new(from_fn(|_| from_fn(|_| Semaphore::new(0))));

    let configuraciones = vec![(0, 1, 2), (1, 0, 3), (2, 3, 0), (3, 2, 1)];

    let mut procesos = vec![];

    for (num, p1, p2) in configuraciones {
        let semaforos_clone = semaforos.clone();

        procesos.push(tokio::spawn(async move {
            proceso(num, p1, p2, semaforos_clone).await
        }));
    }

    join_all(procesos).await;
}
