// Simulamos 2 sensores:
//  1. Sensor color: Detecta el color
//  2. Sensor opacidad: Determina la opacidad del color

use tokio::time::{sleep, Duration, Instant};

async fn sensor_color() -> String {
  println!("Sensor Color: Iniciando deteccion de color");
  
  // Simulacion deteccion color
  sleep(Duration::from_secs(1)).await;

  let color_detectado = "Rojo".to_string();
  println!("Sensor Color: Se detecto el color {}", color_detectado);
  color_detectado
}

async fn sensor_opacidad() -> f64 {
  println!("Sensor Opacidad: Iniciando deteccion de opacidad");

  // Simulacion deteccion opacidad
  sleep(Duration::from_secs(1)).await;

  let opacidad = 45.23;
  println!("Sensor Opacidad: El porcentaje de opacidad es del {}%", opacidad);
  opacidad
}

#[tokio::main]
async fn main() {
  // Arrancamos a medir el tiempo
  let inicio = Instant::now();

  // Ahora corremos ambos sensores en distintas tareas concurrentemente
  let (color_detectado, opacidad_detectada) = tokio::join!(sensor_color(), sensor_opacidad());

  // Tomamos el tiempo final
  let tiempo_total = inicio.elapsed();

  // Tomamos resultados
  println!("\n-------RESULTADOS---------");
  println!("Color obtenido: {}", color_detectado);
  println!("Opacidad detectada: {}%", opacidad_detectada);
  println!("Tiempo total: {:.2?}", tiempo_total);
}