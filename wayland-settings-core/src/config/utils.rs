use std::path::PathBuf;
use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Write},
};

pub fn theme_file(path: PathBuf, line_starts_with: &str, insert_string: String) {
    let mut conf = OpenOptions::new()
        .read(true)
        .open(
            path.to_str()
                .expect("Failed: to convert path to str (VSCode)"),
        )
        .expect("file error");
    let reader = BufReader::new(&mut conf);

    let mut lines: Vec<String> = reader
        .lines()
        .map(|l| l.expect("Couldn't read a line"))
        .collect();

    for i in 0..lines.len() {
        if lines[i].starts_with(line_starts_with) {
            if let Some(elem) = lines.get_mut(i) {
                *elem = insert_string.clone();
            }
        }
    }

    let data = lines.join("\n");
    let mut f = File::create(path).expect("Unable to create file");
    f.write_all(data.as_bytes()).expect("Unable to write data");
    f.flush().expect("Error: Flushing  VSCodes settings.json");
}
