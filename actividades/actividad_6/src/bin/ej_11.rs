use std::thread;
fn main() {
   let vector: Vec<i32> = (1..=100).collect();
   let(izq, der) = vector.split_at(50);
   let suma_total = thread::scope(|s| {
       let h1 = s.spawn(|| {
           izq.iter().sum::<i32>()
       });
       let h2 = s.spawn(|| {  
           der.iter().sum::<i32>()
       });
       h1.join().unwrap() + h2.join().unwrap()
   });
   println!("La suma total es: {}", suma_total);
}