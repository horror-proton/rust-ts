use std::io::{BufRead, Write};

const DEFAULT_FMT: &str = "%b %d %H:%M:%S ";

fn work_default(fmt: &str) -> ! {
    let mut buf: Vec<u8> = vec![];

    let mut i_handle = std::io::stdin().lock();
    let mut o_handle = std::io::stdout().lock();

    loop {
        buf.clear();
        let _ = i_handle.read_until(b'\n', &mut buf);
        let _ = o_handle.write_all(
            chrono::offset::Local::now()
                .format(fmt)
                .to_string()
                .as_bytes(),
        ); // TODO: chrono::format::strftime is slow use libc instead?
        let _ = o_handle.write_all(&buf);
    }
}

fn work_increase() -> ! {
    let mut buf: Vec<u8> = vec![];

    let mut i_handle = std::io::stdin().lock();
    let mut o_handle = std::io::stdout().lock();

    let mut t0 = std::time::Instant::now();
    loop {
        buf.clear();
        let _ = i_handle.read_until(b'\n', &mut buf);
        let t1 = std::time::Instant::now();
        // TODO: better way to format
        let _ = o_handle.write_all(format!("{:?} ", (t1 - t0)).as_bytes());
        let _ = o_handle.write_all(&buf);
        t0 = t1;
    }
}

fn work_since_start() -> ! {
    let mut buf: Vec<u8> = vec![];
    let mut i_handle = std::io::stdin().lock();
    let mut o_handle = std::io::stdout().lock();

    let t0 = std::time::Instant::now();
    loop {
        buf.clear();
        let _ = i_handle.read_until(b'\n', &mut buf);
        let t1 = std::time::Instant::now();
        // TODO: better way to format
        let _ = o_handle.write_all(format!("{:?} ", (t1 - t0)).as_bytes());
        let _ = o_handle.write_all(&buf);
    }
}

fn main() {
    enum TimeMode {
        Default,
        Increase,
        SinceStart,
    }

    let mut time_mode = TimeMode::Default;
    let mut fmt: Option<String> = None;

    for argument in std::env::args().skip(1) {
        match argument.as_str() {
            "-i" if fmt.is_none() => {
                time_mode = TimeMode::Increase;
            }
            "-s" if fmt.is_none() => {
                time_mode = TimeMode::SinceStart;
            }
            other => {
                fmt = Some(fmt.get_or_insert(String::new()).to_owned() + other + " ");
            }
        }
    }

    match time_mode {
        TimeMode::Default => {
            work_default(fmt.unwrap_or_else(|| DEFAULT_FMT.to_string()).as_str());
        }
        TimeMode::Increase => {
            work_increase();
        }
        TimeMode::SinceStart => {
            work_since_start();
        }
    }
}
