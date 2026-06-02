use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

fn counter(base: usize) -> usize {
    let counter = Arc::new(AtomicUsize::new(base));
    let mut handles = vec![];

    // a) Crear 10 hilos.
    for _ in 0..10 {
        let c = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // b) Cada hilo debe incrementar el contador una vez.
            // c) Utilizar fetch_add()
            c.fetch_add(1, Ordering::Relaxed)
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    counter.load(Ordering::Relaxed)
}
fn main() {
    let counter = counter(0);

    // d) Mostrar el valor final.
    println!("Valor final: {}", counter);
}

// f) Agregar un test unitario verificando que el resultado final sea exactamente: 10
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter_adds_10_succesfully() {
        assert_eq!(counter(0), 10)
    }
}