use std::sync::{Arc, Mutex, Condvar}; // Arc para compatir el Mutex entre hilos y variable de condicion
use std::collections::VecDeque; // Estructura del buffer
use std::thread; // Para crear hilos de la lib estandar
use rand::Rng; // Para generar num random

const CAPACIDAD: usize = 5; // Capacidad del buffer

// -----------IMPLEMENTACION SEMAFORO-------------
struct Semaforo {
  valor: Mutex<usize>, // Valor del semaforo protegido en Mutex para que las operaciones p(S) y v(S) sean atomicas
  condvar: Condvar,
}

impl Semaforo {

  // Constructor del semaforo
  fn new(valor_inicial: usize) -> Self {
    Semaforo {
      valor: Mutex::new(valor_inicial),
      condvar: Condvar::new(),
    }
  }

  // Operacion ATOMICA p(S) = wait(S)
  fn wait(&self) {
    // Tomamos valor del semaforo protegido en Mutex para que sea todo ATOMICO
    let mut valor = self.valor.lock().unwrap();
    // Si el valor del semaforo es 0 entonces el proceso/hilo queda bloqueado en la lista de espera L del semaforo
      while *valor == 0 {
        valor = self.condvar.wait(valor).unwrap();
      }
      // Si no decrementamos el valor del semaforo
      *valor -= 1;
  }

  // Operacion ATOMICA v(S) = signal(S)
  fn signal(&self) {
    // Tomamos valor del semaforo protegido en Mutex para que sea todo ATOMICO
    let mut valor = self.valor.lock().unwrap();
    // Incrementamos el valor del semaforo +1
    *valor += 1;
    // Desperamos a un proceso random en la lista de espera L del semaforo
    self.condvar.notify_one();
  }
}


// ------------MAIN---------------
fn main(){
  // Estado inicial semaforos (mutex, espacios, items)
  let sem_mutex = Arc::new(Semaforo::new(1));
  let sem_espacios = Arc::new(Semaforo::new(CAPACIDAD));
  let sem_items = Arc::new(Semaforo::new(0));

  // Buffer protegido por Mutex
  let buffer = Arc::new(Mutex::new(VecDeque::<u32>::new()));

  // Para agregar hilos
  let mut handles = vec![];

  // ------------PRODUCTOR (1)------------
  // Semaforos del productor + buffer
  let mutex_prod = Arc::clone(&sem_mutex);
  let espacios_prod = Arc::clone(&sem_espacios);
  let items_prod = Arc::clone(&sem_items);
  let buffer_prod = Arc::clone(&buffer);

  // Hilo productor
  let productor = thread::spawn(move || {
    // Productor genera 20 numeros aleatorios [1,100]
    for _ in 0..=20 {
      let numero_random = rand::thread_rng().gen_range(1..=100);
      espacios_prod.wait(); // Productor hace p(sem_espacios) para consultar si hay espacio en el buffer sino queda bloqueado
      mutex_prod.wait(); // Productor hace p(sem_mutex) para acceder a la zona critica del buffer

      // Solicitamos el mutex del buffer y agregamos al buffer el item generado
      let mut buf = buffer_prod.lock().unwrap();
      buf.push_back(numero_random);
      println!("Productor: Genere {}", numero_random);
      drop(buf);

      mutex_prod.signal(); // Productor hace v(sem_mutex) para salir de la zona critica del buffer y comunicarselo a los demas
      items_prod.signal(); // Productor hace v(sem_items) para comunicar que hay un item mas en el buffer
    }

    println!("Productor: Termine de generar numeros aleatorios");
  });
  handles.push(productor);


  // -----------CONSUMIDORES (2)--------------
  for id in 1..=2 {
    // Semaforos de consumidor + buffer
    let mutex_cons = Arc::clone(&sem_mutex);
    let espacios_cons = Arc::clone(&sem_espacios);
    let items_cons = Arc::clone(&sem_items);
    let buffer_cons = Arc::clone(&buffer);

    // Hilo consumidor
    let consumidor = thread::spawn(move || {
      let mut numeros_procesados = 0u32;

      loop {
        items_cons.wait(); // Consumidor hace p(sem_items) para consultar si hay items en el buffer sino queda bloqueado
        mutex_cons.wait(); // Consumidor hace p(sem_mutex) para acceder a la zona critica del buffer

        // Solicitamos el mutex del buffer y consumimos un item del buffer
        let mut buf = buffer_cons.lock().unwrap();
        let numero = buf.pop_front().unwrap();
        println!("Consumidor {}: Procese {}", id, numero);
        drop(buf);

        mutex_cons.signal(); // Consumidor hace v(sem_mutex) para salir de la zona critica del buffer y comunicarselo a los demas
        espacios_cons.signal(); // Consumidor hace v(sem_espacios) para comunicar que hay un espacio libre en el buffer

        // Si el consumidor proceso la mitad de items generados entonces termina su parte
        numeros_procesados += 1;
        if numeros_procesados == 20/2 {
          println!("Consumidor {}: Termine mi trabajo", id);
          break;
        }
      }
    });
    handles.push(consumidor);
  }

  // Hilo principal espera a los 3 hilos (1 productor y 2 consumidores)
  for hilo in handles {
    hilo.join().unwrap();
  }

  println!("Fin de ejecucion.")
}