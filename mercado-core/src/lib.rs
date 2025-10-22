use models::AppAtivo;
use models::Ordem;
use models::AppError;
use std::fs::File;
use std::io::BufWriter;
use std::io::Read;
use serde::de::DeserializeOwned;
use csv::Reader;
#[derive(Debug, Clone)]
pub struct TaskManager {
    pub task: Vec<Ordem>,
    pub futuro: Vec<Ordem>,
    pub acao: Vec<Ordem>
}

impl TaskManager {
    pub fn new() -> Self {
        TaskManager { 
            task: Vec::new(), 
            futuro: Vec::new(), 
            acao: Vec::new()
        }
    }
    pub fn ordernar_read<R: Read>(&mut self, s: Reader<R>) -> Result<(), AppError> 
        where 
            Ordem: DeserializeOwned,
    {
        let valido = s
            .into_deserialize::<Ordem>()
            .filter_map(|f| {
                match f {
                    Ok(ordem) => ordem.status_check().ok(),
                    Err(_) => None
                }
            })
            .collect::<Vec<Ordem>>();
        self.task = valido;
        
        Ok(())
    }
    pub fn salvar_json(&self, path: &str) -> Result<(), AppError> {
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &self.task)?;
        Ok(())
    }

    pub fn futuro_check(&mut self, opcao: &str) {
        match opcao {
            "2" => {
                self.futuro = self.task
                    .iter()
                    .filter(|ordem|
                        ordem.ativo == AppAtivo::WinV25
                    )
                    .cloned()
                    .collect::<Vec<Ordem>>()
            },//salvar futuro
            "3" => {
                self.acao = self.task
                    .iter()
                    .filter(|ordem|
                        ordem.ativo != AppAtivo::WinV25
                    )
                    .cloned()
                    .collect::<Vec<Ordem>>()                
            },//salvar acao
            _ => println!("opcao invalida")
        }
    }
}