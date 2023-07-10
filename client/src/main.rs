use std::io::{self, BufRead, Write};
use std::net::TcpStream;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("localhost:8080")?; // Altere o endereço e a porta conforme necessário

    let mut input = String::new();
    let mut buffer = [0; 512];

    loop {
        print!("Digite uma mensagem: ");
        io::stdout().flush()?;

        input.clear();
        io::stdin().read_line(&mut input)?;

        stream.write_all(input.as_bytes())?;

        if input.trim() == "quit" {
            break;
        }

        let bytes_read = stream.read(&mut buffer)?;

        if bytes_read == 0 {
            println!("Conexão fechada pelo servidor.");
            break;
        }

        let response = String::from_utf8_lossy(&buffer[..bytes_read]);
        println!("Resposta do servidor: {}", response);
    }

    Ok(())
}