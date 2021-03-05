//! A parser that takes output files from [BLANT] and
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
//!
//! [BLANT]: (https://github.com/waynebhayes/BLANT)

use crate::parser::ParseError::InvalidFormat;
use itertools::Itertools;
use std::io::BufRead;
use std::result::Result;
use thiserror::Error;

/// Consolidates the various errors that can occur while parsing into one enum so as to unify
/// into one variant high up this projects "error tree".
#[derive(Error, Debug)]
pub enum ParseError {
    /// Error in an underlying IO syscall.
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    /// Error in interpreting text that should be a number as a number.
    #[error(transparent)]
    ParseIntErr(#[from] std::num::ParseIntError),
    // C was not a bool
    #[error(transparent)]
    ParseBoolErrors(#[from] std::str::ParseBoolError),
    /// The formatting of the input file is to be wrong.
    #[error("Invalid format: `{0}`")]
    InvalidFormat(String),
}

impl From<&str> for ParseError {
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

/// The information about a motif edge BLANT condensed into a useful format for the CMS.
#[derive(Debug, Eq, PartialEq)]
pub struct CMSInfo {
    pub uv: String,
    pub op: String,
    /// Whether or not u:v is connected?
    pub c: u8,
}

pub struct Parser {
    line: String,
}

impl Default for Parser {
    fn default() -> Self {
        Self {
            line: String::new(),
        }
    }
}

impl Parser {
    pub fn new() -> Self {
        Self {
            line: String::new(),
        }
    }

    /// Given any buffered reader, this function will extract one line of text and attempt to parse
    /// it expecting the following format (explained in more detail in module docs):
    /// `P u:v e o:p q:r x:y`
    /// where: P is a flag meaning prediction mode, u, v, x and y are nodes of the format
    /// [a-zA-z]*[0-9]+, and o, p, q and r are positive integers.
    ///
    /// # Arguments
    /// * `input` - any type that implements [BufRead] that can output the above format.
    ///
    /// # Returns
    /// While there is still a line left in `input`, returns an `Ok(Some(parser::MotifInfo))`. On
    /// error returns a `parser::ParseError` and on EOF returns `Ok(None)`
    ///
    /// [BufRead]: (https://doc.rust-lang.org/std/io/trait.BufRead.html)
    pub fn parse_motif<R: BufRead>(
        &mut self,
        input: &mut R,
    ) -> Result<Option<MotifInfo>, ParseError> {
        self.line.clear();
        // Get an entire line from the input, return None on EOF
        if let 0 = input.read_line(&mut self.line)? {
            return Ok(None);
        };

        // Split on whitespace, skip P
        let mut tokens = self.line.split_whitespace().skip(1);

        // Get node pair
        let uv = tokens.next().ok_or("node pair")?;

        // Split node pair on ':'
        let (u_node, v_node) = uv.split(':').collect_tuple().ok_or("node pair")?;

        // Parse into prefix and int
        let (u_prefix, u) = Parser::parse_node(u_node)?;
        let (v_prefix, v) = Parser::parse_node(v_node)?;

        // Skip c, get orbit pair
        let op = tokens.nth(1).ok_or("orbit pair")?;

        // Split into o and p
        let (o_str, p_str) = op.split(':').collect_tuple().ok_or("orbit pair 2")?;
        let (o, p) = (o_str.parse::<u32>()?, p_str.parse::<u32>()?);

        // Concat into hashable representation
        let raw = format!("{}:{}:{}:{}", u_node, v_node, o_str, p_str);

        // Pack into struct
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

    /// Given any buffered reader, this function will extract one line of text and attempt to parse
    /// it expecting the following format (explained in more detail in module docs):
    /// `P u:v e o:p q:r x:y`
    /// where: P is a flag meaning prediction mode, u, v, x and y are nodes of the format
    /// [a-zA-z]*[0-9]+, and o, p, q and r are positive integers.
    ///
    /// # Arguments
    /// * `input` - any type that implements [BufRead] that can output the above format.
    ///
    /// # Returns
    /// While there is still a line left in `input`, returns an `Ok(Some(String))`, where the String
    /// is a condensed, hashable representation. On error returns a `parser::ParseError` and on EOF
    /// returns `Ok(None)`
    ///
    /// [BufRead]: (https://doc.rust-lang.org/std/io/trait.BufRead.html)
    pub fn parse_raw<R: BufRead>(&mut self, input: &mut R) -> Result<Option<String>, ParseError> {
        self.line.clear();
        // Get an entire line from the input, return None on EOF
        if let 0 = input.read_line(&mut self.line)? {
            return Ok(None);
        };

        // Split on whitespace, skip P
        let mut tokens = self.line.split_whitespace().skip(1);

        // Get node pair
        let uv = tokens.next().ok_or("node pair")?;

        // Skip c, get orbit pair
        let op = tokens.nth(1).ok_or("orbit pair")?;

        // Concat into raw hashable representation
        Ok(Some(format!("{}:{}", uv, op)))
    }

    /// Given any buffered reader, this function will extract one line of text and attempt to parse
    /// it expecting the following format (explained in more detail in module docs):
    /// `P u:v e o:p q:r x:y`
    /// where: P is a flag meaning prediction mode, u, v, x and y are nodes of the format
    /// [a-zA-z]*[0-9]+, and o, p, q and r are positive integers.
    ///
    /// # Arguments
    /// * `input` - any type that implements [BufRead] that can output the above format.
    ///
    /// # Returns
    /// While there is still a line left in `input`, returns an `Ok(Some((String, String)))`, where the
    /// String is a condensed, hashable representation. On error returns a `parser::ParseError` and
    /// on EOF returns `Ok(None)`
    ///
    /// [BufRead]: (https://doc.rust-lang.org/std/io/trait.BufRead.html)
    pub fn parse_cms<R: BufRead>(&mut self, input: &mut R) -> Result<Option<CMSInfo>, ParseError> {
        self.line.clear();
        // Get an entire line from the input, return None on EOF
        if let 0 = input.read_line(&mut self.line)? {
            return Ok(None);
        };

        // Split on whitespace, skip P
        let mut tokens = self.line.split_whitespace().skip(1);

        // Get node pair
        let uv = tokens.next().ok_or("node pair")?;

        // Get connected bit
        let c = tokens.next().ok_or("connected")?;
        let c_num = match c {
            "0" => 0,
            "1" => 1,
            _ => return Err(InvalidFormat("C must be 0 or 1".into())),
        };

        // Get orbit pair
        let op = tokens.next().ok_or("orbit pair")?;

        // Concat into raw hashable representation
        Ok(Some(CMSInfo {
            uv: uv.to_owned(),
            op: op.to_owned(),
            c: c_num,
        }))
    }

    fn parse_node(tok: &str) -> Result<(String, u32), ParseError> {
        if let Some(idx) = tok.find(&['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'][..]) {
            Ok((tok[..idx].to_owned(), tok[(idx + 1)..].parse::<u32>()?))
        } else {
            Err("no numbers in node".into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    #[test]
    fn parse_motif_test() -> Result<(), ParseError> {
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

        let mut actual = Vec::new();
        let mut file_br = BufReader::new(std::fs::File::open("tests/4_line_motif_test.txt")?);

        let mut parser = Parser::new();

        while let Some(mi) = parser.parse_motif(&mut file_br)? {
            actual.push(mi);
        }

        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn parse_raw_test() -> Result<(), ParseError> {
        let expected = vec![
            "ENSG00000164164:ENSG00000175376:11:12".to_owned(),
            "ENSG00000006194:ENSG00000174851:6:6".to_owned(),
            "ENSG00000205302:ENSG00000175895:11:12".to_owned(),
            "ENSG00000147041:ENSG00000205302:6:6".to_owned(),
        ];

        let mut actual = Vec::new();
        let mut file_br = BufReader::new(std::fs::File::open("tests/4_line_motif_test.txt")?);

        let mut parser = Parser::new();

        while let Some(raw) = parser.parse_raw(&mut file_br)? {
            actual.push(raw);
        }

        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn parse_split_test() -> Result<(), ParseError> {
        let expected = vec![
            CMSInfo {
                uv: "ENSG00000164164:ENSG00000175376".to_owned(),
                op: "11:12".to_owned(),
                c: 0,
            },
            CMSInfo {
                uv: "ENSG00000006194:ENSG00000174851".to_owned(),
                op: "6:6".to_owned(),
                c: 0,
            },
            CMSInfo {
                uv: "ENSG00000205302:ENSG00000175895".to_owned(),
                op: "11:12".to_owned(),
                c: 0,
            },
            CMSInfo {
                uv: "ENSG00000147041:ENSG00000205302".to_owned(),
                op: "6:6".to_owned(),
                c: 0,
            },
        ];

        let mut actual = Vec::new();
        let mut file_br = BufReader::new(std::fs::File::open("tests/4_line_motif_test.txt")?);

        let mut parser = Parser::new();

        while let Some(raw) = parser.parse_cms(&mut file_br)? {
            actual.push(raw);
        }

        assert_eq!(expected, actual);
        Ok(())
    }
}
