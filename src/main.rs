use std::fmt::Debug;

use press_batch::{Press, Row, a};
use thiserror::Error;

fn main() -> anyhow::Result<()> {
    // get csv file path from argument
    let arg = std::env::args();

    if arg.len() != 2 {
        return Err(Error::InvalidArgument.into());
    }

    let path: Vec<String> = arg
        .into_iter()
        .enumerate()
        .filter(|v| v.0 > 0)
        .map(|v| v.1)
        .collect();

    // construct CSV reader from given csv file
    let mut reader = csv::Reader::from_path(path.first().unwrap())?;

    let mut p1 = Press::new();
    let mut p2 = Press::new();

    for res in reader.deserialize::<Row>() {
        let row = res?;
        p1.push(row.press_1);
        p2.push(row.press_2);
    }

    let res = a(p1, p2);
    println!("{:?}", res);

    Ok(())
}

/// Error type application returns.
#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid argument: need csv file path")]
    InvalidArgument,
}
