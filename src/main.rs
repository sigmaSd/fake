use std::io::Write;
fn main() {
    let mut args = std::env::args().skip(1);
    let n = args.next().unwrap();
    let s = args.next().unwrap();
    let s = parse(s);
    let mut f = std::fs::File::create(n).unwrap();
    write(&mut f, s);
}

fn write(f: &mut std::fs::File, s: usize) {
    if std::env::args().any(|s| s == "--rand") {
        use rand::Rng;
        let mut v: Vec<u8> = vec![0; s];
        let mut rng = rand::thread_rng();

        for x in v.iter_mut() {
            *x = rng.gen();
        }
        f.write_all(&v).unwrap();
    } else {
        f.write_all(&vec![0_u8; s]).unwrap();
    }
}
fn parse(s: String) -> usize {
    if s.ends_with("MB") {
        s[..s.len() - 2].parse::<usize>().unwrap() * 10_usize.pow(6)
    } else {
        unreachable!()
    }
}
