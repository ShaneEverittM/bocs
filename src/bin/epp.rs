use clap::App;

use epp::{
    cms::CountMinSketch,
    parser::{ParseError, Parser},
};

use log::info;
use simplelog::{CombinedLogger, WriteLogger, LevelFilter, Config};

use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

static CONFIDENCE: f64 = 99.0;

fn init_logger() {
    if !Path::exists(Path::new("./epp_logs")) {
        fs::create_dir("./epp_logs").unwrap();
    }

    let debug_file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("epp_logs/debug.log")
        .unwrap();

    let info_file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("epp_logs/info.log")
        .unwrap();

    CombinedLogger::init(vec![
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            info_file,
        ),
        WriteLogger::new(
            LevelFilter::Debug,
            Config::default(),
            debug_file,
        ),
    ]).unwrap();
}

fn init_cli() -> Result<(usize, u32), ParseError> {
    let matches = App::new("EPP")
        .version("0.1")
        .author("Shane Murphy, Elliott Allison, Maaz Adeeb")
        .arg_from_usage("-k <NUMBER> 'Sets the k-value that was used in BLANT'")
        .arg_from_usage("-e <NUMBER> 'Sets the error_rate to 1^-<NUMBER>'")
        .arg_from_usage("-v 'Enables logging, will create logging directory'")
        .get_matches();

    let k = matches
        .value_of("k")
        .expect("Must supply k value")
        .parse::<usize>()?;

    let e = matches
        .value_of("e")
        .expect("Must supply e value")
        .parse::<u32>()?;

    if matches.is_present("v") {
        init_logger()
    }
    Ok((k, e))
}


fn main() -> Result<(), ParseError> {
    let (k, exponent) = init_cli()?;

    let stdin = std::io::stdin();
    let mut stdin_handle = stdin.lock();

    let mut uvs = HashMap::new();
    let mut ops = HashSet::new();

    let e = 1.0 / u32::pow(10, exponent) as f64;

    let mut cms = CountMinSketch::new(e, CONFIDENCE);

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

    let range = (e * count as f64).floor() as u64;

    info!("Covered {} lines of input with k={}, e={} and a range of {}", count, k, e, range);

    for (uv, c) in uvs.iter() {
        let mut output = format!("{} {}", uv, c);
        let mut found_any = false;
        for op in ops.iter() {
            let raw = format!("{}:{}", uv, op);
            if let Some(pred) = cms.get(&raw) {
                if range < pred as u64 {
                    output += &format!("\t{}:{} {}", k, op, pred);
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
