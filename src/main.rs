use std::io::BufRead;
use std::process::{Command, Stdio};

// Returns a line having removed every substring starting
// with '\x1b' and ending with 'm'.
fn trim_line(line: &str) -> String {
    line.chars()
        .scan(false, |in_tag, ch| {
            if *in_tag {
                if ch == 'm' {
                    *in_tag = false;
                }
                Some('\0')
            } else if ch == '\x1b' {
                *in_tag = true;
                Some('\0')
            } else {
                Some(ch)
            }
        })
        .filter(|&ch| ch != '\0')
        .collect()
}

#[test]
fn trim_line_empty() {
    assert_eq!(trim_line(""), "");
}

#[test]
fn trim_line_trimmed() {
    assert_eq!(trim_line("abc"), "abc");
}

#[test]
fn trim_line_begin() {
    assert_eq!(trim_line("\x1babcmmdef"), "mdef");
}

#[test]
fn trim_line_end() {
    assert_eq!(trim_line("abcm\x1bdefm"), "abcm");
}

fn reverse_errors(stderr: std::io::BufReader<std::process::ChildStderr>) {
    let mut errors = Vec::<Vec<String>>::new();
    let mut current_error = Vec::<String>::new();
    for line in stderr.lines() {
        let line = line.unwrap();
        let trimmed = trim_line(&line);

        // If the line is empty, starts an error record.
        // If the line starts with a letter or there is not yet a current error,
        // print this line.
        // If the line starts with another character or there is already a current error,
        // add the line to the current error.
        match trimmed.chars().next() {
            None => {
                if !current_error.is_empty() {
                    errors.push(current_error);
                    current_error = Vec::<String>::new();
                }
            }
            Some(ch) => {
                let is_title = ch.is_ascii_alphabetic();
                let is_empty = current_error.is_empty();
                if is_title || is_empty {
                    println!("{}", line);
                }
                if is_title || !is_empty {
                    current_error.push(line.clone());
                }
            }
        }
    }
    if !errors.is_empty() {
        println!("{}", "-".repeat(60));
        for error in errors.iter().rev() {
            for line in error {
                println!("{}", line);
            }
            println!();
        }
        if !current_error.is_empty() {
            for line in current_error {
                println!("{}", line);
            }
        }
    }
}

fn main() {
    // Launch "cargo", passing to it all the arguments of the current command line
    let mut command = Command::new("cargo");
    let mut in_cargo_arguments = true;
    let mut for_testing = false;
    let mut has_color_option = false;
    for arg in std::env::args().skip(1) {
        if arg.starts_with("--color") {
            has_color_option = true;
        }
        if in_cargo_arguments {
            if arg == "t" || arg == "test" {
                // It is a "cargo test" command.
                for_testing = true;
            }
            if arg == "--" {
                if has_color_option {
                    has_color_option = false;
                } else {
                    command.arg("--color=always");
                }
                in_cargo_arguments = false;
            }
        }
        command.arg(&arg);
    }
    if in_cargo_arguments && !has_color_option {
        command.arg("--color=always");
    }
    if for_testing {
        if in_cargo_arguments {
            command.arg("--");
        }
        if in_cargo_arguments || !has_color_option {
            command.arg("--color=always");
        }
    }

    let mut child = command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to launch the program.");
    let stdout = child.stdout.take().expect("no stdout");
    let stderr = child.stderr.take().expect("no stderr");

    let read_stderr = std::thread::spawn(|| {
        reverse_errors(std::io::BufReader::new(stderr));
    });

    let mut out_reader = std::io::BufReader::new(stdout);
    loop {
        let mut line = String::new();
        if out_reader.read_line(&mut line).unwrap() == 0 {
            break;
        }
        print!("{}", line);
    }

    read_stderr.join().unwrap();
}

#[test]
fn dummy() {
    assert_eq!(2 + 3, 5);
}

#[test]
fn dummy2() {
    assert_eq!(2 + 3, 6);
}
