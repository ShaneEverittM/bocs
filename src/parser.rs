//! A parser that takes output files from [BLANT](https://github.com/waynebhayes/BLANT) and
//! extracts information about Motifs.
//!
//! # Input Format
//! TODO: Update this to be clearer and more general.
//! The leading P is just a flag, meaning "Prediction mode"; next we have
//! "ENSG00000114125:ENSG00000137266", which is the pair of nodes under consideration for
//! prediction, which I always call u and v; then there's a 0 or 1 (called e), indicating whether
//! these two nodes have an edge (1) or not (0) in the input file ( HI-union.el ); next we have
//! "11:11", which is the orbit pair o:p but without the leading k, which is on the command
//! line--in this case, k=4.
//!
//! The next two columns require explanation and a diagram:
//! Above is an L3; nodes u and v are the endpoints, occupying orbit pair o:p=11:11, and nodes
//! x and y are the "interior" nodes occupying orbit pair q:r=12:12. Orbit IDs are labeled in red.
//! The u and v nodes (at their orbit positions 11:11) are explained above, where u is
//! ENSG00000114125  and v is ENSG00000137266.
//!
//! The next column is q:r 11:12, and the last column is "ENSG00000114125:ENSG00000135916".
//! These two columns represent the edge (u,x) connecting orbits 11 and 12 in the diagram
//! (and from the diagram we see that x must be node ENSG00000135916).

use itertools::Itertools;
use std::io::BufRead;
use std::result::Result;
use thiserror::Error;

/// Consolidates the various errors that can occur while parsing into one enum so as to unify
/// into one variant high up this projects "error tree".
#[derive(Error, Debug)]
pub enum ParserError {
    /// Error in an underlying IO syscall.
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    /// Error in interpreting text that should be a number as a number.
    #[error(transparent)]
    ParseIntErr(#[from] std::num::ParseIntError),
    /// The formatting of the input file is to be wrong.
    #[error("Invalid format: `{0}`")]
    InvalidFormat(String),
}

impl From<&str> for ParserError {
    fn from(s: &str) -> Self {
        Self::InvalidFormat(String::from(s))
    }
}

/// A parsed Motif from BLANT.
#[derive(Debug, Eq, PartialEq)]
pub struct MotifInfo {
    /// A condensed representation for hashing.
    pub raw: String,
    /// The alphanumeric prefix denoting the dataset origin of u.
    pub u_prefix: String,
    /// The node number of u.
    pub u: u32,
    /// The alphanumeric prefix denoting the dataset origin of u.
    pub v_prefix: String,
    /// The node number of v.
    pub v: u32,
    /// The orbit number of o.
    pub o: u32,
    /// The orbit number of p.
    pub p: u32,
}

fn parse_node(tok: &str) -> Result<(String, u32), ParserError> {
    if let Some(idx) = tok.find(&['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'][..]) {
        Ok((tok[..idx].to_owned(), tok[(idx + 1)..].parse::<u32>()?))
    } else {
        Err("no numbers in node".into())
    }
}

/// Given any buffered reader, this function will extract one line of text and attempt to parse
/// it expecting the following format (explained in more detail in module docs):
/// `P u:v e o:p q:r x:y`
/// where: P is a flag meaning prediction mode, u, v, x and y are nodes of the format
/// [a-zA-z]*[0-9]+, and o, p, q and r are positive integers.
///
/// # Arguments
/// * `input` - any type that implements
/// [BufRead](https://doc.rust-lang.org/std/io/trait.BufRead.html) that can output the above format.
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
