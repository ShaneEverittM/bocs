use clap::App;

use epp::{
    cms::CountMinSketch,
    parser::{ParseError, Parser},
};

use log::info;
use simplelog::{CombinedLogger, Config, LevelFilter, WriteLogger};
use std::process::Command;

use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, LineWriter, Write};
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
        WriteLogger::new(LevelFilter::Info, Config::default(), info_file),
        WriteLogger::new(LevelFilter::Debug, Config::default(), debug_file),
    ])
    .unwrap();
}

fn init_cli() -> Result<(usize, u32), ParseError> {
    let matches = App::new("EPP")
        .version("0.3")
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
    let quads_path = &format!(
        "{}/epp-quads-{}.txt",
        std::env::temp_dir().to_str().unwrap(),
        std::process::id()
    );

    let unique_quads_path = &format!(
        "{}/epp-unique-quads-{}.txt",
        std::env::temp_dir().to_str().unwrap(),
        std::process::id()
    );

    let (k, exponent) = init_cli()?;

    let stdin = std::io::stdin();
    let mut stdin_handle = stdin.lock();

    let e = 1.0 / u32::pow(10, exponent) as f64;

    let mut cms = CountMinSketch::new(e, CONFIDENCE);

    let mut parser = Parser::new();

    let mut count: u64 = 0;

    let mut buffer_file = LineWriter::new(
        std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(quads_path)
            .unwrap(),
    );

    while let Some(cms_info) = parser.parse_cms(&mut stdin_handle)? {
        count += 1;
        let uvop = format!("{}:{}", cms_info.uv, cms_info.op);
        buffer_file
            .write_all(format!("{} {} {}\n", cms_info.uv, cms_info.c, cms_info.op).as_bytes())
            .unwrap();
        cms.put(&uvop);
    }

    let range = (e * count as f64).floor() as u64;

    info!(
        "Covered {} lines of input with k={}, e={} and a range of {}",
        count, k, e, range
    );

    Command::new("sort")
        .args(&["-u", "-k", "1", "-o", unique_quads_path, quads_path])
        .output()
        .unwrap();

    let mut seen = BufReader::new(File::open(unique_quads_path).unwrap());
    let mut line = String::new();
    let mut output = String::new();
    while let Ok(bytes) = seen.read_line(&mut line) {
        if bytes == 0 {
            break;
        }

        let mut found_any = false;

        let mut tokens = line.split_whitespace();
        let uv = tokens.next().unwrap();
        let c = tokens.next().unwrap();
        if uv != output {
            output = format!("{} {}", uv, c);
        }
        let op = tokens.next().unwrap();

        let raw = format!("{}:{}", uv, op);

        if let Some(pred) = cms.get(&raw) {
            output += &format!("\t{}:{} {}", k, op, pred);
            found_any = true;
        }

        if found_any {
            println!("{}", output);
        }
    }

    std::fs::remove_file(unique_quads_path).unwrap();
    std::fs::remove_file(quads_path).unwrap();

    Ok(())
}
