use clap::App;
use epp::{
    cms::CountMinSketch,
    parser::{ParseError, Parser},
};
use std::collections::{HashMap, HashSet};

fn main() -> Result<(), ParseError> {
    let matches = App::new("EPP")
        .version("0.1")
        .author("Shane Murphy, Elliott Allison, Maaz Adeeb")
        .arg_from_usage("-k <NUMBER> 'Sets the k-value that was used in BLANT'")
        .get_matches();

    let k = matches
        .value_of("k")
        .expect("Must supply k value")
        .parse::<usize>()?;

    let stdin = std::io::stdin();
    let mut stdin_handle = stdin.lock();

    let mut uvs = HashMap::new();
    let mut ops = HashSet::new();

    let mut cms = CountMinSketch::new(1e-5, 99.99);

    let mut parser = Parser::new();

    while let Some(cms_info) = parser.parse_cms(&mut stdin_handle)? {
        uvs.insert(cms_info.uv.clone(), cms_info.c);
        ops.insert(cms_info.op.clone());
        cms.put(&format!("{}:{}", cms_info.uv, cms_info.op));
    }

    for (uv, c) in uvs.iter() {
        print!("{} {}", uv, c);
        for op in ops.iter() {
            let raw = format!("{}:{}", uv, op);
            if let Some(count) = cms.get(&raw) {
                print!("    {}:{} {}", k, op, count);
            }
        }
        println!()
    }

    Ok(())
}
