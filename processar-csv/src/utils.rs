use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Write};
use csv::{ReaderBuilder, Reader};
use encoding_rs::WINDOWS_1252;
use encoding_rs_io::DecodeReaderBytesBuilder;
use models::AppError;

// Sua função de CLI
pub fn read_string (prompt: &str) -> String {
    print!("{}", prompt);
    std::io::stdout().flush().unwrap();
    let mut buffer = String::new();
    match std::io::stdin().read_line(&mut buffer){
        Ok(_) => buffer.trim().to_string(), 
        Err(_) => {
            println!("Erro de leitura");
            std::process::exit(1);
        },
    }
}

// Suas funções de leitura de CSV
pub fn open_win1252_reader(path: String) -> Result<impl Read, AppError> {
    let file = File::open(path)?;
    let transcoded_reader = DecodeReaderBytesBuilder::new()
        .encoding(Some(WINDOWS_1252))
        .build(file);
    Ok(transcoded_reader)
}
//encode WINDOWS_1252 => UTF-8

pub fn skip_first_line<R: Read>(buf_reader: &mut BufReader<R>) -> Result<(), io::Error> {
    let mut discarded_line = String::new();
    buf_reader.read_line(&mut discarded_line)?;
    Ok(())
}
//Pular primeira linha

pub fn build_csv_reader<R: Read>(reader: R) -> Reader<R> {
    ReaderBuilder::new()
        .delimiter(b';')
        .has_headers(true)
        .flexible(true)
        .trim(csv::Trim::All)
        .from_reader(reader)
}
