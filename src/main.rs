use std::io::{BufRead, Write};
use chrono;

static FMT: &str = "%b %d %H:%M:%S ";

fn main() {
    let mut buf: Vec<u8> = vec![];

    let mut i_handle = std::io::stdin().lock();
    let mut o_handle = std::io::stdout().lock();

    loop {
        buf.clear();
        let _ = i_handle.read_until(b'\n', &mut buf);
        let _ = o_handle.write_all(
            chrono::offset::Local::now().format(FMT).to_string().as_bytes());
        let _ = o_handle.write_all(&buf);
    }
}
