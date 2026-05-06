pub fn main() {
    /// Variable que vive en el Stack
    let id: u32 = 5;

    println!("ID: {id}");

    /// Variable que vive en el Heap
    let descripcion: String = String::from("Tarea 1");

    println!("Descripción: {descripcion}");
}