use std::{
    thread::spawn, sync::mpsc::{channel, Receiver, Sender}
};

fn productor(tx: &Sender<u8>) -> Result<(), String> {
    for numero in 1..=10 {
        tx.send(numero).map_err(|error| error.to_string())?;
    }
    Ok(())
}

fn consumidor(rx: &Receiver<u8>) -> u8 {
    let mut suma = 0;

    for numero in rx {
        suma += numero;
    }
    suma
}

fn main() {
    let (tx, rx) = channel();

    let handle_productor = spawn(move || productor(&tx));
    let handle_consumidor = spawn(move || consumidor(&rx));

    if let Err(error) = handle_productor.join() {
        eprintln!("{error:?}");
    }
    match handle_consumidor.join() {
        Ok(suma) => println!("Suma total = {suma}"),
        Err(error) => eprintln!("{error:?}"),
    }
}