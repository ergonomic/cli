use pulldown_cmark::{CodeBlockKind, Event, Parser, Tag};

pub fn parse(input: String) -> String {
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
