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
        .arg_from_usage("-e <NUMBER> 'Sets the error_rate to 1^-<NUMBER>'")
        .get_matches();

    let k = matches
        .value_of("k")
        .expect("Must supply k value")
        .parse::<usize>()?;

    let e = matches
        .value_of("e")
        .expect("Must supply e value")
        .parse::<u32>()?;

    let stdin = std::io::stdin();
    let mut stdin_handle = stdin.lock();

    let mut uvs = HashMap::new();
    let mut ops = HashSet::new();

    let error_rate = 1.0 / u32::pow(10, e) as f64;

    let mut cms = CountMinSketch::new(error_rate, 99.0);

    let mut parser = Parser::new();

    let mut count: u64 = 0;

    while let Some(cms_info) = parser.parse_cms(&mut stdin_handle)? {
        count += 1;
        if !uvs.contains_key(&cms_info.uv) {
            uvs.insert(cms_info.uv.clone(), cms_info.c);
        }
        if !ops.contains(&cms_info.op) {
            ops.insert(cms_info.op.clone());
        }
        cms.put(&format!("{}:{}", cms_info.uv, cms_info.op));
    }

    let range = (error_rate * count as f64).floor() as u64;

    for (uv, c) in uvs.iter() {
        let mut output = format!("{} {}", uv, c);
        let mut found_any = false;
        for op in ops.iter() {
            let raw = format!("{}:{}", uv, op);
            // actual >= pred - (error_rate * num_of_entries)
            if let Some(pred) = cms.get(&raw) {
                if range < pred as u64 {
                    let actual = pred as u64 - range;
                    output += &format!("\t{}:{} {}", k, op, actual);
                    found_any = true;
                }
            }
        }
        if found_any {
            println!("{}", output);
        }
    }

    Ok(())
}
