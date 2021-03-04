use itertools::Itertools;
use std::io::BufRead;
use std::result::Result;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParserError {
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error(transparent)]
    ParseIntErr(#[from] std::num::ParseIntError),
    #[error("Invalid format around `{0}`")]
    InvalidFormat(String),
}

impl From<&str> for ParserError {
    fn from(s: &str) -> Self {
        Self::InvalidFormat(String::from(s))
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct MotifInfo {
    pub raw: String,
    u_prefix: String,
    u: u32,
    v_prefix: String,
    v: u32,
    o: u32,
    p: u32,
}

fn parse_node(tok: &str) -> Result<(String, u32), ParserError> {
    if let Some(idx) = tok.find(&['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'][..]) {
        Ok((tok[..idx].to_owned(), tok[(idx + 1)..].parse::<u32>()?))
    } else {
        Err("no numbers in node".into())
    }
}

pub fn parse_motif<R: BufRead>(input: &mut R) -> Result<Option<MotifInfo>, ParserError> {
    // Line buffer
    let mut line = String::new();

    // Get an entire line from the input
    if let 0 = input.read_line(&mut line)? {
        return Ok(None);
    };

    // Split on whitespace
    let mut tokens = line.split_whitespace().skip(1);

    // Get node pair
    let uv = tokens.next().ok_or("node pair")?;

    // Split node pair on ':'
    let (u_node, v_node) = uv.split(':').collect_tuple().ok_or("node pair")?;

    // Parse into prefix and int
    let (u_prefix, u) = parse_node(u_node)?;
    let (v_prefix, v) = parse_node(v_node)?;

    // Skip c, get orbit pair
    let op = tokens.nth(1).ok_or("orbit pair")?;

    // Split into o and p
    let (o_str, p_str) = op.split(':').collect_tuple().ok_or("orbit pair 2")?;
    let (o, p) = (o_str.parse::<u32>()?, p_str.parse::<u32>()?);

    let raw = format!("{}:{}:{}:{}", u_node, v_node, o_str, p_str);

    Ok(Some(MotifInfo {
        raw,
        u_prefix,
        u,
        v_prefix,
        v,
        o,
        p,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    #[test]
    fn it_works() -> Result<(), ParserError> {
        let expected = vec![
            MotifInfo {
                raw: "ENSG00000164164:ENSG00000175376:11:12".to_owned(),
                u_prefix: "ENSG".to_owned(),
                u: 164164,
                v_prefix: "ENSG".to_owned(),
                v: 175376,
                o: 11,
                p: 12,
            },
            MotifInfo {
                raw: "ENSG00000006194:ENSG00000174851:6:6".to_owned(),
                u_prefix: "ENSG".to_owned(),
                u: 6194,
                v_prefix: "ENSG".to_owned(),
                v: 174851,
                o: 6,
                p: 6,
            },
            MotifInfo {
                raw: "ENSG00000205302:ENSG00000175895:11:12".to_owned(),
                u_prefix: "ENSG".to_owned(),
                u: 205302,
                v_prefix: "ENSG".to_owned(),
                v: 175895,
                o: 11,
                p: 12,
            },
            MotifInfo {
                raw: "ENSG00000147041:ENSG00000205302:6:6".to_owned(),
                u_prefix: "ENSG".to_owned(),
                u: 147041,
                v_prefix: "ENSG".to_owned(),
                v: 205302,
                o: 6,
                p: 6,
            },
        ];
        let _input = std::io::stdin();
        let mut actual = Vec::new();
        let file = std::fs::File::open("4_line_motif_test.txt")?;
        let mut file_br = BufReader::new(file);
        while let Some(mi) = parse_motif(&mut file_br)? {
            actual.push(mi);
        }

        assert_eq!(expected, actual);
        Ok(())
    }
}
