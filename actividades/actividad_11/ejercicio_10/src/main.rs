use std::collections::{HashSet, VecDeque};

/// Representa una Red de Petri con su marcado actual, matrices Pre y Post
// Marcado inicial: [P1_NC, P1_Esp, P1_CS, P2_NC, P2_Esp, P2_CS, Mutex]
const START: [usize; 7] = [1, 0, 0, 1, 0, 0, 1];

/// Implementación de la Red de Petri
#[derive(Clone)]
struct PetriNet {
    marking: Vec<usize>,
    pre_matrix: Vec<Vec<usize>>,  // pre_matrix[transicion][lugar]
    post_matrix: Vec<Vec<usize>>, // post_matrix[transicion][lugar]
}

/// Implementación de métodos para la Red de Petri
impl PetriNet {
    /// Crea una nueva Red de Petri con el marcado inicial y las matrices Pre y Post
    fn new(initial_marking: Vec<usize>, pre: Vec<Vec<usize>>, post: Vec<Vec<usize>>) -> Self {
        Self {
            marking: initial_marking,
            pre_matrix: pre,
            post_matrix: post,
        }
    }

    /// Verifica si la transición dada está sensibilizada según el marcado actual M(p) >= Pre(t, p)
    fn is_enabled(&self, transition: usize) -> bool {
        self.pre_matrix.get(transition).map_or(false, |pre_row| {
            pre_row
                .iter()
                .zip(&self.marking)
                .all(|(&pre_val, &mark_val)| mark_val >= pre_val)
        })
    }

    /// Dispara la transición dada y actualiza el marcado actual según M' = M - Pre(t) + Post(t)
    fn fire(&mut self, transition: usize) -> Result<(), &'static str> {
        if !self.is_enabled(transition) {
            return Err("Transición no habilitada para el marcado actual.");
        }

        for (p, mark) in self.marking.iter_mut().enumerate() {
            *mark = *mark - self.pre_matrix[transition][p] + self.post_matrix[transition][p];
        }

        Ok(())
    }

    /// Calcula un marcado sucesor directo al aplicar una transición sin modificar el estado actual
    fn next_marking(&self, transition: usize) -> Option<Vec<usize>> {
        if !self.is_enabled(transition) {
            return None;
        }

        let next = self
            .marking
            .iter()
            .enumerate()
            .map(|(p, &mark)| {
                mark - self.pre_matrix[transition][p] + self.post_matrix[transition][p]
            })
            .collect();

        Some(next)
    }

    /// Explora todo el espacio de marcados alcanzables R(M0) usando BFS
    fn reachable_markings(&self) -> Vec<Vec<usize>> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut reachables = Vec::new();

        visited.insert(self.marking.clone());
        queue.push_back(self.marking.clone());
        reachables.push(self.marking.clone());

        let num_transitions = self.pre_matrix.len();

        while let Some(current) = queue.pop_front() {
            // Creo una red temporal liviana para consultar transiciones desde el marcado desencolado
            let state = PetriNet {
                marking: current,
                pre_matrix: self.pre_matrix.clone(),
                post_matrix: self.post_matrix.clone(),
            };

            for t in 0..num_transitions {
                if let Some(next_state) = state.next_marking(t) {
                    if visited.insert(next_state.clone()) {
                        queue.push_back(next_state.clone());
                        reachables.push(next_state);
                    }
                }
            }
        }
        reachables // devuelvo todos los marcados alcanzables 
    }
}

fn main() {
    // paso la constante START y las matrices PRE y POST a vectores para crear la red de Petri
    let start = START.to_vec();

    // Matrices Pre y Post creadas directamente con vec![] (simple y directo)
    // Transiciones:
    // t0: pedir1  | t1: entrar1 | t2: salir1
    // t3: pedir2  | t4: entrar2 | t5: salir2
    let pre_matrix = vec![
        vec![1, 0, 0, 0, 0, 0, 0], // t0: pedir1
        vec![0, 1, 0, 0, 0, 0, 1], // t1: entrar1 (requiere P1_Esp y Mutex)
        vec![0, 0, 1, 0, 0, 0, 0], // t2: salir1
        vec![0, 0, 0, 1, 0, 0, 0], // t3: pedir2
        vec![0, 0, 0, 0, 1, 0, 1], // t4: entrar2 (requiere P2_Esp y Mutex)
        vec![0, 0, 0, 0, 0, 1, 0], // t5: salir2
    ];

    let post_matrix = vec![
        vec![0, 1, 0, 0, 0, 0, 0], // t0
        vec![0, 0, 1, 0, 0, 0, 0], // t1
        vec![1, 0, 0, 0, 0, 0, 1], // t2 (devuelve token a P1_NC y a Mutex)
        vec![0, 0, 0, 0, 1, 0, 0], // t3
        vec![0, 0, 0, 0, 0, 1, 0], // t4
        vec![0, 0, 0, 1, 0, 0, 1], // t5 (devuelve token a P2_NC y a Mutex)
    ];

    // creo la red de Petri y calculo los marcados alcanzables
    let net = PetriNet::new(start, pre_matrix, post_matrix);
    let states = net.reachable_markings();

    println!("=== EXPLORACIÓN DE ESTADOS ALCANZABLES ===");
    println!("Cantidad total de marcados en R(M0): {}", states.len());
    for (idx, state) in states.iter().enumerate() {
        println!("M{:02}: {:?}", idx, state);
    }

    // M(P1_CS) + M(P2_CS) <= 1 --> Compruebo Safety
    // P1_CS está en el índice 2 y P2_CS está en el índice 5
    let is_safe = states.iter().all(|m| m[2] + m[5] <= 1);

    println!("\n=== COMPROBACIÓN DE SAFETY ===");
    if is_safe {
        println!(
            "PROPIEDAD CUMPLIDA: Ningún marcado alcanzable permite que ambos procesos estén en Sección Crítica al mismo tiempo."
        );
    } else {
        println!("VIOLACIÓN DE SAFETY: Se detectó un estado con exclusión mutua rota.");
    }
}
