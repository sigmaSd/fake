type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let n = args.next().ok_or("no file name specified")?;
    let s = args.next().ok_or("no file size specified")?;
    let s = parse(s)?;
    let f = std::io::BufWriter::with_capacity(s, std::fs::File::create(n)?);
    write(f, s)?;
    Ok(())
}

fn write(mut f: impl std::io::Write, s: usize) -> Result<()> {
    let b = if let Some(idx) = std::env::args().position(|a| a == "-b") {
        Some(parse(
            std::env::args()
                .nth(idx + 1)
                .ok_or("no buffer size specified")?,
        )?)
    } else {
        None
    };

    if !std::env::args().any(|s| s == "--zero") {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        for i in 0..s {
            if let Some(b) = b {
                if i != 0 && i % b == 0 {
                    f.flush()?;
                }
            }

            f.write_all(&[rng.gen::<u8>()])?;
        }
        f.flush()?;
    } else {
        for i in 0..s {
            if let Some(b) = b {
                if i != 0 && i % b == 0 {
                    f.flush()?;
                }
            }
            f.write_all(&[0])?;
        }

        f.flush()?;
    }
    Ok(())
}
fn parse(s: String) -> Result<usize> {
    if s.ends_with("MB") {
        Ok(s[..s.len() - 2].parse::<usize>()? * 10_usize.pow(6))
    } else if s.ends_with("GB") {
        Ok(s[..s.len() - 2].parse::<usize>()? * 10_usize.pow(9))
    } else {
        Err("you can only specify MB/GB as size unit".into())
    }
}
