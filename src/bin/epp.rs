use epp_rust::{cms, parser};

fn main() -> Result<(), parser::ParserError> {
    let mut motifs = Vec::new();

    let stdin = std::io::stdin();
    let mut stdin_handle = stdin.lock();

    while let Some(mi) = parser::parse_motif(&mut stdin_handle)? {
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
