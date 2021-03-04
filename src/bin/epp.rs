use epp_rust::{cms, parser};
use std::collections::{HashMap, HashSet};

fn main() -> Result<(), parser::ParseError> {
    let stdin = std::io::stdin();
    let mut stdin_handle = stdin.lock();

    let mut uvs = HashMap::new();
    let mut ops = HashSet::new();

    let mut cms = cms::CountMinSketch::new(1e-5, 99.99);

    while let Some(cms_info) = parser::parse_cms(&mut stdin_handle)? {
        uvs.insert(cms_info.uv.clone(), cms_info.c);
        ops.insert(cms_info.op.clone());
        cms.put(&format!("{}:{}", cms_info.uv, cms_info.op));
    }

    let k = 4;

    for (uv, c) in uvs.iter() {
        print!("{} {}", uv, *c as u32);
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
