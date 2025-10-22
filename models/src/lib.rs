use serde::{Deserialize, Serialize};
use std::num::ParseIntError;
use thiserror::Error;

#[derive(Debug, Deserialize, Serialize, Clone)]   
pub struct Ordem { 
    //enum nome_ativo
    #[serde(rename = "Ativo")]
    pub ativo: AppAtivo,  
    //[c, v] ou NULL
    #[serde(rename = "Lado")]
    pub lado: Option<String>,  
    //enum Status
    #[serde(rename = "Status")]
    pub status: AppStatus, 
    //data
    #[serde(rename = "Criação")]
    pub criacao: String,
    //tempo na operacao = criacao - ultima att
    #[serde(rename = "Última Atualização")]
    pub ultima_atualizacao: Option<String>,  
    //str -> f64
    #[serde(rename = "Preço")]
    pub preco: Option<String>,
    //str -> f64
    #[serde(rename = "Preço Stop")]
    pub preco_stop: Option<String>,
    //contratos
    #[serde(rename = "Qtd")]
    pub qtd: Option<String>,
    
    #[serde(rename = "Preço Médio")]
    pub preco_medio: Option<String>,
    
    #[serde(rename = "Qtd Executada")]
    pub qtd_executada: Option<String>,
    
    #[serde(rename = "Qtd restante")]
    pub qtd_restante: Option<String>,
    
    #[serde(rename = "Total")]
    pub total: Option<String>,  
    
    #[serde(rename = "Total Executado")]
    pub total_executado: Option<String>,
     
    #[serde(rename = "Carteira")]
    pub carteira: Option<String>,   
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum AppStatus {
    Aberta,
    Executada,
    Cancelada, 
    Cancel,    // Descarte 
    Trade,     // Descarte

    #[serde(rename = "Execução Parcial Cancelada")]
    ExecucaoParcialCancelada,

    #[serde(other)]
    Desconhecido, // Erro
}
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum AppAtivo {
    #[serde(rename = "WINV25")]
    WinV25,

    Outro(String)
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Erro de IO: {0}")]
    Io(#[from] std::io::Error),

    #[error("Error de Parse CSV: {0}")]
    Parse(#[from] csv::Error),

    #[error("Config inválida: {0}")]
    ConfigInvalida(String),

    #[error("Erro de Parse Int: {0}")]
    ParseInt(#[from] ParseIntError),

    
    #[error("Status da ordem indica que deve ser descartada)")]
    StatusDescartavel,

    #[error("Erro Dinâmico: {0}")]
    Dynamic(#[from] Box<dyn std::error::Error + Send + Sync + 'static>),

    #[error("Erro de Parse/Serialização JSON: {0}")]
    JsonProcessing(#[from] serde_json::Error),
}

impl Ordem {
    /// Verifica o status da ordem e retorna erro se for inválido ou descartável
    pub fn status_check(self) -> Result<Self, AppError> {
        match self.status {
            AppStatus::Trade | AppStatus::Desconhecido | AppStatus::Cancel => Err(AppError::StatusDescartavel),
            _ => Ok(self)
        }
    }
}



