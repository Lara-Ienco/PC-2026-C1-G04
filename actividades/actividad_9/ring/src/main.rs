use std::{
    sync::mpsc:: {Receiver, RecvTimeoutError, Sender, channel}, 
    thread, 
    time::{Duration, Instant}};

const TIMEOUT: Duration = Duration::from_secs(3);
const CANT_NODOS: usize = 5;

#[derive(Debug,Clone)]
enum Message {
    Election{origin: usize, ids: Vec<usize>},
    Coordinator{id: usize},

    Request{origin: usize, values: Vec<f64>},
    Stop,
}

struct Process{
    id: usize,
    leader_id: Option<usize>,
    next: Sender<Message>,
    receiver: Receiver<Message>,

    in_election: bool,
    time: Instant,
    alive: bool,
}

impl Process {
    fn new(id: usize, next: Sender<Message>, receiver: Receiver<Message>) -> Self {
        Process{
            id, 
            leader_id: None, 
            next, 
            receiver,
            in_election: false,
            time: Instant::now(),
            alive: true,
        }
    }

    fn start(&mut self) {
        self.start_election();

        loop {
            // si soy lider, envio periodicamente
            // recoleccion de mediciones
            if Some(self.id) == self.leader_id && self.alive{
                if self.time.elapsed() >= TIMEOUT{
                    self.request_values();
                    self.time = Instant::now();
                }
            }

            match self.receiver.recv_timeout(TIMEOUT) {
                Ok(msg) => self.handle_message(msg),
                Err(RecvTimeoutError::Timeout) => {
                    if Some(self.id) != self.leader_id && !self.in_election {
                        self.start_election();
                    }
                },
                Err(_) => break,
            }
        }
    }

    fn start_election(&mut self) {
        if !self.in_election &&self.alive{
            self.in_election = true;
            let _ = self.next.send(Message::Election {origin: self.id, ids: vec![self.id] });
        }
    }

    fn request_values(&mut self) {
        let _ = self.next.send(Message::Request {origin: self.id, values: vec![] });
    }

    fn handle_message(&mut self, msg: Message) {
        match msg {
            Message::Election { origin, mut ids } => {
                // Si el nodo está caído, reenvía (puenteo)
                if !self.alive {
                    let _ = self.next.send(Message::Election { origin, ids });
                    return;
                }
                if !ids.contains(&self.id) {
                    ids.push(self.id);
                }
                if origin == self.id {
                   
                    if let Some(&leader) = ids.iter().max() {
                        self.leader_id = Some(leader);
                        self.in_election = false;
                        let _ = self.next.send(Message::Coordinator { id: leader });
                    }
                } else {
                    // si no soy el origen, reenvío al siguiente
                    self.in_election = true;
                    let _ = self.next.send(Message::Election { origin, ids });
                }
            },
            Message::Coordinator { id } => {
                self.leader_id = Some(id);                
                if self.id != id {
                    let  _ = self.next.send(Message::Coordinator { id }) ;
                } else {
                    if self.in_election { 
                        println!("Coordinador: {}", self.id);
                    }
                }
                self.in_election = false;
            },
            Message::Request {origin, mut values } => {
                if !self.alive{
                    let _ = self.next.send(Message::Request {origin, values });
                    return;
                }
                if origin == self.id {
                    // calculamos el promedio de la mediciones
                    let sum: f64 = (&values).into_iter().sum();
                    let avg = if values.is_empty() { 0.0 } else { sum / values.len() as f64 };
                    println!("Promedio de mediciones: {:.2}", avg);
                } else {
                    let value = rand::random_range(0.0..=10.0);
                    values.push(value);
                    let _ = self.next.send(Message::Request {origin, values});
                }
            },
            Message::Stop => {
                if self.alive{
                    println!("Coordinador recibe Stop.");
                    self.alive = false;
                    self.in_election = false;
                }
            },
        }
    }
}

fn main() {
    let mut senders = Vec::with_capacity(CANT_NODOS);
    let mut receivers = Vec::with_capacity(CANT_NODOS);
    for _ in 1..=CANT_NODOS {
        let (tx, rx) = channel();
        senders.push(tx);
        receivers.push(rx);
    }

    let mut nodes = Vec::new();
    for n in 1..=CANT_NODOS {
        let next_id = (n % CANT_NODOS) +1; 
        let next = senders[next_id-1].clone();
        let receiver = receivers.remove(0);
        nodes.push(Process::new(n, next, receiver));
    }

    let mut handles = vec![];
    for mut node in nodes {
        handles.push(thread::spawn(move || node.start()));
    }

     
    thread::sleep(Duration::from_secs(10));

    println!("Simulamos caída del Coordinador.");

    if let Some(sender) = senders.get(4) { 
         let _ = sender.send(Message::Stop);
    }

    thread::sleep(Duration::from_secs(5));
}
