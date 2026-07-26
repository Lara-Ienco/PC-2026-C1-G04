use std::{
    collections::HashMap,
    sync::mpsc::{channel, Receiver, RecvTimeoutError, Sender},
    thread,
    time::{Duration, Instant},
};

const TIMEOUT: Duration = Duration::from_secs(3); 
const CANT_NODES: usize = 5;

#[derive(Debug, Clone)]
enum Message {
    Election { id: usize },
    Ok,
    Coordinator { id: usize },
    Task { nums: Vec<i32> },
    Stop,
}

struct Process {
    id: usize,
    leader_id: Option<usize>,
    senders: HashMap<usize, Sender<Message>>,
    receiver: Receiver<Message>,
    in_election: bool,
}

impl Process {
    fn new(
        id: usize,
        senders: HashMap<usize, Sender<Message>>,
        receiver: Receiver<Message>,
    ) -> Self {
        Process {
            id,
            leader_id: None,
            senders,
            receiver,
            in_election: false,
        }
    }

    fn start(&mut self) {
        self.start_election();

        loop {
            match self.receiver.recv_timeout(TIMEOUT) {
                Ok(msg) => self.handle_message(msg),
                Err(RecvTimeoutError::Timeout) => {
                    if let Some(leader) = self.leader_id {
                        if leader != self.id && !self.in_election {
                            self.start_election();
                        }
                    }
                }
                Err(_) => break,
            }

            if Some(self.id) == self.leader_id {
                self.generate_and_send_task();
            }
        }
    }

    fn handle_message(&mut self, msg: Message) {
        match msg {
            Message::Election { id } => {
                if id < self.id {
                    self.send_to(id, Message::Ok);
                    if !self.in_election {
                        self.start_election();
                    }
                }
            }
            Message::Ok => {
                // recibimos Ok de un mayor
            }
            Message::Coordinator { id } => {
                println!("[P{}] Nuevo Coordinador: {}", self.id, id);
                self.leader_id = Some(id);
                self.in_election = false;
            }
            Message::Task { nums } => {
                if Some(self.id) == self.leader_id {
                    let sum: i32 = nums.iter().sum();
                    //podemos imprimir resultados
                } else {
                    let processed: Vec<i32> = nums.iter().map(|&n| n * 2).collect();
                    if let Some(leader) = self.leader_id {
                        self.send_to(leader, Message::Task { nums: processed });
                    }
                }
            }
            Message::Stop => {
                if self.leader_id == Some(self.id) {
                    println!(
                        "[P{}] Falla del Coordinador",
                        self.id
                    );
                    thread::sleep(Duration::from_secs(15));
                    println!("[P{}] Vuelve el Coordinador. ", self.id);
                    self.in_election = false;
                    self.start_election();
                }
            }
        }
    }

    fn send_to(&self, id: usize, msg: Message) {
        if let Some(sender) = self.senders.get(&id) {
            let _ = sender.send(msg);
        }
    }

    fn send_all(&self, msg: Message) {
        for (_, sender) in self.senders.iter() {
            let _ = sender.send(msg.clone());
        }
    }

    fn start_election(&mut self) {
        if self.in_election {
            return;
        }
        self.in_election = true;

        for (&id, sender) in self.senders.iter() {
            if id > self.id {
                let _ = sender.send(Message::Election { id: self.id });
            }
        }

        let time_to_wait = Instant::now() + TIMEOUT;
        while Instant::now() < time_to_wait {
            match self.receiver.recv_timeout(Duration::from_millis(100)) {
                Ok(Message::Ok) => {
                    self.in_election = false;
                    return;
                }
                Ok(Message::Coordinator { id }) => {
                    self.leader_id = Some(id);
                    self.in_election = false;
                    return;
                }
                Ok(other) => {
                    self.handle_message(other);
                    if let Some(leader) = self.leader_id {
                        if leader != self.id {
                            self.in_election = false;
                            return;
                        }
                    }
                }
                Err(RecvTimeoutError::Timeout) => continue,
                Err(_) => {
                    self.in_election = false;
                    return;
                }
            }
        }

        println!("[P{}] Soy nuevo Coordinador.", self.id);
        self.leader_id = Some(self.id);
        self.send_all(Message::Coordinator { id: self.id });
        self.in_election = false;
    }

    fn generate_and_send_task(&mut self) {
        for (&id, sender) in self.senders.iter() {
            if id != self.id {
                let num1 = rand::random_range(1..=10);
                let num2 = rand::random_range(1..=10);
                let nums = vec![num1, num2];
                let _ = sender.send(Message::Task { nums });
            }
        }
    }
}

fn main() {
    let nodes = CANT_NODES;
    let mut senders = HashMap::new();
    let mut receivers = HashMap::new();

    for i in 1..=nodes {
        let (tx, rx) = channel();
        senders.insert(i, tx);
        receivers.insert(i, rx);
    }

    let mut nodes_channels = HashMap::new();
    for i in 1..=nodes {
        let mut node_channel = HashMap::new();
        for j in 1..=nodes {
            if i != j {
                if let Some(s) = senders.get(&j) {
                    node_channel.insert(j, s.clone());
                }
            }
        }
        nodes_channels.insert(i, node_channel);
    }

    let mut handles = vec![];
    for i in 1..=nodes {
        if let Some(senders) = nodes_channels.remove(&i) {
            if let Some(receiver) = receivers.remove(&i) {
                let mut p = Process::new(i, senders, receiver);
                handles.push(thread::spawn(move || p.start()));
            }
        }
    }

    thread::sleep(Duration::from_secs(3));
    
    thread::sleep(Duration::from_secs(5));
    println!("Simulamos caída del Coordinador");
    for (_, sender) in senders.iter() {
        let _ = sender.send(Message::Stop);
    }

    thread::sleep(Duration::from_secs(20));
}