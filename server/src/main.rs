use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;


fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    loop {
        match stream.read(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    break;
                }
                let message = String::from_utf8_lossy(&buffer[..bytes_read]);
                println!("Mensagem recebida: {}", message);

                // Processar a mensagem recebida, se necessário
                
                stream.write_all(b"Recebido!");
            }
            Err(e) => {
                eprintln!("Erro ao ler do socket: {}", e);
                break;
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("localhost:8080")?; // Altere o endereço e a porta conforme necessário
    println!("Servidor escutando em {}", listener.local_addr()?);
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Nova conexão: {}", stream.peer_addr()?);
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Erro ao aceitar conexão: {}", e);
            }
        }
    }
    
    Ok(())
}