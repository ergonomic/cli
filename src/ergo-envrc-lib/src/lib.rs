use pulldown_cmark::{CodeBlockKind, Event, Parser, Tag};
use regex::Regex;
use std::io::Write;
use std::process::{Command, Stdio};
use std::str;

pub fn render(input: String) -> String {
    let result = parse(input);
    let result = resolve(result);
    return result;
}

fn templated(text: &String) -> bool {
    let re = Regex::new(r"op:\/\/").unwrap();
    return re.is_match(&text);
}

fn inject(input: &mut String) {
    let source = input.clone();
    let mut op = Command::new("op")
        .arg("inject")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to spawn op command");

    let mut stdin = op.stdin.take().expect("failed to open stdin");
    std::thread::spawn(move || {
        stdin
            .write_all(source.as_bytes())
            .expect("failed to write to stdin");
    });

    let output = op.wait_with_output().expect("failed to read stdout");
    let result = str::from_utf8(&output.stdout)
        .expect("read result")
        .to_string();

    input.clear();
    input.push_str(&result.to_string());
}

fn resolve(text: String) -> String {
    let mut result = text.clone();

    while templated(&result) {
        inject(&mut result);
    }

    return result.to_string();
}

fn parse(input: String) -> String {
    let parser = Parser::new(&input);
    let mut result = "".to_string();
    let mut active = false;

    for event in parser {
        match event {
            Event::Start(tag) => match tag {
                Tag::CodeBlock(kind) => match kind {
                    CodeBlockKind::Fenced(info) => {
                        let lang = info.split(' ').next().unwrap();
                        if lang == "envrc" {
                            active = true;
                        }
                    }
                    _ => (),
                },
                _ => (),
            },
            Event::End(tag) => match tag {
                Tag::CodeBlock(kind) => match kind {
                    CodeBlockKind::Fenced(info) => {
                        let lang = info.split(' ').next().unwrap();
                        if lang == "envrc" {
                            active = false;
                            result += "\n";
                        }
                    }
                    _ => (),
                },
                _ => (),
            },
            Event::Text(body) => {
                if active {
                    result += &body.to_string();
                }
            }
            _ => (),
        }
    }

    return result;
}
