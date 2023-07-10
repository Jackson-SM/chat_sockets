# Documentação do Servidor TCP
## Descrição
> Este é um servidor TCP básico escrito em Rust. Ele recebe mensagens dos clientes e envia uma resposta simples de confirmação.

## Dependências
- **std::io::{Read, Write}**: Fornece funcionalidades de entrada/saída para leitura e escrita.

- **std::net::{TcpListener, TcpStream}**: Permite a criação de um servidor TCP e a comunicação com os clientes.

- **std::thread**: Permite a execução simultânea de várias tarefas.
## Funções Principais
**handle_client**
```rust
fn handle_client(mut stream: TcpStream) {
    // ...
}
```
> Essa função é responsável por lidar com as requisições de cada cliente conectado ao servidor. Ela recebe um objeto TcpStream que representa a conexão com o cliente e executa um loop para ler as mensagens enviadas pelo cliente, processá-las, se necessário, e enviar uma resposta.

**main**
```rust
fn main() -> std::io::Result<()> {
    // ...
}
```
> A função main é o ponto de entrada do programa. Ela cria um objeto TcpListener que escuta as conexões na porta especificada e entra em um loop para lidar com cada conexão recebida. Para cada conexão, um novo thread é criado para chamar a função handle_client e lidar com a conexão de forma assíncrona.

## Execução
> Para executar o servidor, é necessário chamar a função main. O servidor irá escutar na porta especificada e aguardar as conexões dos clientes. Ao receber uma mensagem de um cliente, o servidor imprimirá a mensagem e enviará uma resposta de confirmação.

# Documentação do Cliente TCP

## Descrição
> Este é um cliente TCP básico escrito em Rust. Ele se conecta a um servidor TCP, envia mensagens para o servidor e imprime as respostas recebidas.

## Dependências
- **std::io::{self, BufRead, Write}**: Fornece funcionalidades de entrada/saída para leitura e escrita.

- **std::net::TcpStream**: Permite a conexão com um servidor TCP.

- **std::io::Read**: Fornece funcionalidades de leitura.
## Funções Principais
**main**
```rust
fn main() -> std::io::Result<()> {
    // ...
}
```
> A função main é o ponto de entrada do programa. Ela cria um objeto TcpStream que se conecta ao servidor especificado. Em seguida, entra em um loop onde solicita ao usuário uma mensagem para ser enviada ao servidor. A mensagem é lida do console, enviada ao servidor e a resposta do servidor é impressa no console. O loop continua até que o usuário digite "quit" como mensagem de saída.

## Execução
> Para executar o cliente, é necessário chamar a função main. O cliente irá se conectar ao servidor na porta especificada e permitirá que o usuário digite mensagens para serem enviadas ao servidor. A resposta do servidor será impressa no console.

> Essa é uma documentação básica para ajudar os usuários a entenderem o funcionamento do servidor e do cliente TCP escritos em Rust. Você pode adicionar mais informações e detalhes conforme necessário. Certifique-se de incluir qualquer instrução adicional para compilar e executar o código, se necessário.