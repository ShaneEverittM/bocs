use thiserror::Error;

mod hasher;
mod parser;

#[derive(Error, Debug)]
enum EPPError {
    #[error(transparent)]
    ParserError(#[from] parser::ParserError),
}

fn main() -> Result<(), EPPError> {
    let mut motifs = Vec::new();
    while let Some(mi) = parser::parse_motif(&mut std::io::stdin().lock())? {
        motifs.push(mi);
    }
    dbg!(&motifs);
    Ok(())
}
