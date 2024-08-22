use std::{
    self, env, fs,
    io::{stdout, Write},
    path::Path,
};

fn main() {
    let mut writer = std::io::BufWriter::new(stdout());
    let args: Vec<String> = env::args().collect();
    args[1..]
        .iter()
        .map(Path::new)
        .map(|fpath| {
            fs::read_to_string(fpath)
                .expect(&format!("Cannot open path: {}", fpath.to_str().unwrap()))
        })
        .for_each(|out| {
            if let Err(e) = writeln!(writer, "{}", out) {
                eprintln!("Error writing to output: {}", e);
            }
        });
    // writer.flush().expect_err("Error flushing output!");
}
