use epp_rust::{cms, parser};
use thiserror::Error;

#[derive(Error, Debug)]
enum EPPError {
    #[error(transparent)]
    Parse(#[from] parser::ParserError),
}

fn main() -> Result<(), EPPError> {
    let mut motifs = Vec::new();
    while let Some(mi) = parser::parse_motif(&mut std::io::stdin().lock())? {
        motifs.push(mi);
    }
    dbg!(&motifs);
    let mut cms = cms::CountMinSketch::new(1e-5, 99.99);
    for motif in motifs.iter() {
        cms.put(&motif.raw);
    }
    let count = cms.get(&motifs[0].raw);
    dbg!(count);
    Ok(())
}
