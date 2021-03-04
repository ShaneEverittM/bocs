use epp_rust::{cms, parser};

fn main() -> Result<(), parser::ParseError> {
    let mut raws = Vec::new();

    let stdin = std::io::stdin();
    let mut stdin_handle = stdin.lock();

    while let Some(raw) = parser::parse_raw(&mut stdin_handle)? {
        raws.push(raw);
    }

    dbg!(&raws);

    let mut cms = cms::CountMinSketch::new(1e-5, 99.99);

    for raw in raws.iter() {
        cms.put(&raw);
    }

    let count = cms.get(&raws[0]);

    dbg!(count);

    Ok(())
}
