use std::{
    fs::File,
    io::{self, BufRead, BufReader, Seek},
    process::exit,
    thread, time,
};

const MAX_BUFFER_SIZE: u64 = 2 * 1024 * 1024;

trait Tailer {
    fn tail(file_obj: &mut File, n_lines: u32, read_bytes: u64) -> (String, u64);
    fn poll(filename: &str, n_lines: u32) -> ();
}

struct TailController {}

struct TailCli {
    filename: String,
    n_lines: u32,
}

impl Tailer for TailController {
    fn tail(file_obj: &mut File, n_lines: u32, read_bytes: u64) -> (String, u64) {
        let file_length = file_obj
            .metadata()
            .expect("Cannot get the size of file!")
            .len();
        // Seek to the starting position
        file_obj
            .seek(io::SeekFrom::Start(read_bytes))
            .expect("Cannot seek to the position of read bytes!");
        let buffer = BufReader::new(file_obj);
        let lines: Vec<String> = buffer
            .lines()
            .map(|line| line.expect("Could not read line"))
            .collect();
        (
            lines
                .iter()
                .rev()
                .take(n_lines as usize)
                .map(|x| x.clone())
                .reduce(|acc, e| acc + "\n" + &e)
                .unwrap_or_else(|| String::new()),
            file_length,
        )
    }
    fn poll(filename: &str, n_lines: u32) -> () {
        let mut tail_str: String;
        let mut file_obj = File::open(filename).expect("Cannot open the given file!");
        let file_size = file_obj
            .metadata()
            .expect("Cannot get the size of file!")
            .len();
        let mut read_bytes = if file_size < MAX_BUFFER_SIZE {
            0
        } else {
            file_size - MAX_BUFFER_SIZE
        };
        loop {
            (tail_str, read_bytes) = Self::tail(&mut file_obj, n_lines, read_bytes);
            if tail_str != "" {
                print!("{}\n", tail_str);
            }
            thread::sleep(time::Duration::from_millis(1));
        }
    }
}

fn main() {
    if "--help".to_string() == std::env::args().nth(1).unwrap_or("--help".to_string()) {
        print!("\nusage:\t{}\n", "rust-tail [filename] [number of lines]");
        exit(0)
    }
    let tail_cli = TailCli {
        filename: std::env::args().nth(1).expect("No file provided!"),
        n_lines: std::env::args()
            .nth(2)
            .unwrap_or("10".to_string())
            .parse::<u32>()
            .expect("Cannot parse number of lines variable"),
    };
    TailController::poll(tail_cli.filename.as_str(), tail_cli.n_lines);
}
