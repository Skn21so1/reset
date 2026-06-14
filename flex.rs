use std::io::{self, Write};
use std::process::{Command, Stdio};

fn main() {
    loop {
        println!("1. reset ip");
        println!("2. exit");

        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let escolha: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Digite um número válido.");
                continue;
            }
        };

        match escolha {
            1 => {
                println!("Executando reset de IP...");

                if let Err(err) = resetip() {
                    println!(" Erro ao executar o reset de IP: {}", err);
                } else {
                    println!(" Reset de IP concluído com sucesso!");
                }
            }
            2 => {
                println!("Saindo...");
                break;
            }
            _ => println!("Opção inválida."),
        }
    }
}

fn resetip() -> std::io::Result<()> {
    Command::new("ipconfig")
        .args(["/release"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;

    Command::new("ipconfig")
        .args(["/renew"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;

    Command::new("netsh")
        .args(["int", "ip", "reset"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;

    Command::new("ipconfig")
        .args(["/flushdns"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;

    Command::new("netsh")
        .args(["winsock", "reset"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;

    Ok(())
}
