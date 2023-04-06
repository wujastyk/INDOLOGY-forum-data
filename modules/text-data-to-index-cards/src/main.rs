use std::io::{self, BufRead};
use std::{env, fs, path};

fn main() {
    // get the base dir path
    let args: Vec<String> = env::args().collect();
    let source_files_glob = &args[1];
    dbg!(&source_files_glob);

    for file_path in glob::glob(source_files_glob)
        .expect("Failed to read glob pattern")
        .filter_map(Result::ok)
    {
        dbg!(&file_path);

        if let Ok(lines) = read_lines(file_path) {
            let mut filtered_lines: Vec<&str> = Vec::new();

            for line in lines.iter() {
                let line = line.trim();
                if line.is_empty() { continue; }
                if line.starts_with(['>', '\n']) { continue; }
                if line.starts_with("--") { continue; }
                if line.starts_with("From ") { continue; }
                if line.starts_with("From: ") { continue; }
                if line.starts_with("To: ") { continue; }
                if line.starts_with("Cc: ") { continue; }
                if line.starts_with("Date: ") { continue; }
                //if line.starts_with("Subject: ") { continue; }
                if line.starts_with("Message-ID: ") { continue; }
                if line.starts_with("Message-Id: ") { continue; }
                if line.starts_with("In-Reply-To: ") { continue; }
                if line.starts_with("Reply-To: ") { continue; }
                if line.starts_with("Received: ") { continue; }
                if line.starts_with("Return-path: ") { continue; }
                if line.starts_with("Phone ") { continue; }
                if line.starts_with("Tel: ") { continue; }
                if line.starts_with("Tel. ") { continue; }

                // filter out the attachments related lines
                if line.contains("was scrubbed...") {continue;}
                if line.starts_with("Name: ") { continue; }
                if line.starts_with("Type: ") { continue; }
                if line.starts_with("Size: ") { continue; }
                if line.starts_with("Desc: ") { continue; }
                if line.starts_with("URL: ") { continue; }

                filtered_lines.push(line);
            }

            println!("{:?}\n", filtered_lines);
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<path::Path>,
{
    let file = fs::File::open(filename)?;
    let mut lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect();
    lines.sort();

    Ok(lines)
}

// time cargo build --bin text-data-to-index-cards --target-dir ./target --release --target x86_64-unknown-linux-musl
// time cargo run --bin text-data-to-index-cards --target-dir ./target -- "/home/claudius/workspace/repositories/git/github.com/wujastyk/INDOLOGY-forum-data/data/**/*.txt"
// time ./text-data-to-index-cards "/home/claudius/workspace/repositories/git/github.com/wujastyk/INDOLOGY-forum-data/data/**/*.txt"
