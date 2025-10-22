mod utils;
use std::io::BufReader;
use models::AppError;
use mercado_core::TaskManager;
use utils::{ open_win1252_reader, skip_first_line, build_csv_reader, read_string };

type AppResult<T> = Result<T, AppError>;


fn ler_processar() -> AppResult<TaskManager> {
    let mut manager = TaskManager::new();
    let file_path = read_string("Caminho do CSV: ");
    let format = open_win1252_reader(file_path)?;
    let mut buf_reader = BufReader::new(format);
    skip_first_line(&mut buf_reader)?; // Remova se não precisar
    let csv_reader = build_csv_reader(buf_reader);
    manager.ordernar_read(csv_reader)?; // Popula manager.task
    Ok(manager)
}

fn prompt_salvar_json(manager: &TaskManager) -> AppResult<()> {
    loop {
        let opcao_str = read_string("Deseja salvar o resultado em JSON? (s/n): ");
        match opcao_str.trim().to_lowercase().as_str() {
            "sim" | "s" | "yes" | "y" | "" => {
                let output_path_str = read_string("Digite o caminho para salvar o JSON: ");
                let output_path = output_path_str.trim();
                if output_path.is_empty() {
                    println!("Caminho inválido. Tente novamente.");
                    continue; 
                }
                match manager.salvar_json(output_path) {
                    Ok(_) => println!("Arquivo salvo com sucesso em '{}'!", output_path),
                    Err(e) => eprintln!("Erro ao salvar o arquivo JSON: {}", e),
                }
                return Ok(()); 
            }
            "não" | "nao" | "n" | "no" => {
                println!("Ok, não salvando o arquivo.");
                return Ok(()); 
            }
            _ => {
                println!("Opção inválida. Por favor, digite 's' ou 'n'.");
                
            }
        }
    }
}


fn main() -> AppResult<()> {
    // 1. Ler e processar o CSV
    match ler_processar() {
        Ok(manager) => {
            println!("Ordens processadas com sucesso.");
            println!("Total de Ordens Válidas: {}", manager.task.len());

            prompt_salvar_json(&manager)?; 
        }
        Err(e) => {
            eprintln!("Erro durante o processamento: {}", e); 
            return Err(e);
        }
    }
    Ok(()) 
}