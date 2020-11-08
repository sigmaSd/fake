fn main() {
    let mut args = std::env::args().skip(1);
    let n = args.next().unwrap();
    let s = args.next().unwrap();
    let s = parse(s);
    let mut f = std::io::BufWriter::with_capacity(s, std::fs::File::create(n).unwrap());
    write(&mut f, s);
}

fn write(mut f: impl std::io::Write, s: usize) {
    let b = if let Some(idx) = std::env::args().position(|a| a == "-b") {
        std::env::args().nth(idx + 1)
    } else {
        None
    };

    let b = b.map(parse);

    if std::env::args().any(|s| s == "--rand") {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        for i in 0..s {
            if let Some(b) = b {
                if i != 0 && i % b == 0 {
                    f.flush().unwrap();
                }
            }

            f.write_all(&[rng.gen::<u8>()]).unwrap();
        }
        f.flush().unwrap();
    } else {
        for i in 0..s {
            if let Some(b) = b {
                if i != 0 && i % b == 0 {
                    f.flush().unwrap();
                }
                f.write_all(&[0]).unwrap();
            }
        }

        f.flush().unwrap();
    }
}
fn parse(s: String) -> usize {
    if s.ends_with("MB") {
        s[..s.len() - 2].parse::<usize>().unwrap() * 10_usize.pow(6)
    } else if s.ends_with("GB") {
        s[..s.len() - 2].parse::<usize>().unwrap() * 10_usize.pow(9)
    } else {
        unreachable!()
    }
}
