#[allow(unused_imports)]
use std::io::{self, Write};

mod command;
use command::Command;
mod custom_command;
use custom_command::CustomCommand;
mod built_in_command;
use built_in_command::BuiltInCommand;

fn main() {
    loop {
        process_input();
    }
}

fn process_input() {
    print!("$ ");
    io::stdout().flush().unwrap();

    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();

    let mut arguments = parse_parts(input);
    
    let redirect_index = arguments.iter().position(|x| x.contains(">"));
    let redirect_args = arguments[redirect_index.unwrap_or(arguments.len())..].to_vec();
    if redirect_index.is_some() && redirect_args.len() > 1 {
        arguments = arguments[..redirect_index.unwrap()].to_vec();
    }
    
    let command = arguments.first().unwrap_or(&"".to_string()).clone();
    let arguments = arguments[1..redirect_index.unwrap_or(arguments.len())].to_vec();

    let mut command: Box<dyn Command> =
        BuiltInCommand::try_from_string(command.clone(), arguments.clone())
            .map(|c| Box::new(c) as Box<dyn Command>)
            .unwrap_or_else(|_| Box::new(CustomCommand::new(command, arguments)));

    if redirect_index.is_some() {
        let file = redirect_args[1].clone();
        let redirect_file = if redirect_args[0].contains(">>") {
            std::fs::OpenOptions::new().append(true).create(true).open(file).unwrap()
        } else {
            std::fs::OpenOptions::new().write(true).create(true).truncate(true).open(file).unwrap()
        };
        
        let redirect_descriptor = redirect_args[0].replace(">", "").replace(" ", "");
        
        match redirect_descriptor.as_str() {
            "1" | "" => command.stdout(redirect_file),
            "2" => command.stderr(redirect_file),
            _ => println!("{}: invalid redirect descriptor", redirect_descriptor),
        }
    }
    
    command.execute();
}

fn parse_parts(input: String) -> Vec<String> {
    let mut result = Vec::new();
    let argument_string = input.trim().to_string();

    let mut quote_stack: Vec<char> = Vec::new();

    let mut argument = String::new();
    let mut escape = false;
    for c in argument_string.chars() {
        if (c == '\'' || c == '"') && !escape {
            if quote_stack.is_empty() {
                quote_stack.push(c);
            } else if *quote_stack.last().unwrap() == c {
                quote_stack.pop();
            } else {
                argument.push(c);
            }
        } else if c == '\\' && (quote_stack.is_empty() || *quote_stack.first().unwrap() == '"') {
            if escape {
                argument.push(c);
                escape = false;
            } else {
                escape = true;
            }
        } else if c == ' ' && quote_stack.is_empty() {
            if escape {
                argument.push(c);
                escape = false;
            } else if !argument.is_empty() {
                result.push(argument.clone());
                argument.clear();
            }
        } else {
            if escape
                && !quote_stack.is_empty()
                && *quote_stack.first().unwrap() == '"'
                && c != '"'
                && c != '\\'
                && c != '$'
            {
                argument.push('\\');
            }
            argument.push(c);
            escape = false;
        }
    }

    if !argument.is_empty() {
        result.push(argument.clone());
    }

    result
}

fn locate_in_path(_command: &str) -> Option<String> {
    let path = std::env::var("PATH").unwrap();
    let paths = path.split(":").collect::<Vec<&str>>();

    for p in paths {
        let full_path = format!("{}/{}", p, _command);
        if std::path::Path::new(&full_path).exists() {
            return Some(full_path);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "echo 'Hello, World!' \"This 'is'   a test\"";
        let expected = vec!["echo", "Hello, World!", "This 'is'   a test"];
        let result = parse_parts(input.to_string());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_backslash_escaping() {
        let input = r#"echo before\ \ after"#;
        let expected = vec!["echo", r#"before  after"#];
        let result = parse_parts(input.to_string());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_backslash_escape_quotes() {
        let input = "echo \\\'\\\"test shell\\\"\\\'";
        let expected = vec!["echo", r#"'"test"#, r#"shell"'"#];
        let result = parse_parts(input.to_string());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_backslash_within_double_quotes() {
        let input = r#"echo "hello\"insidequotes"script\""#;
        let expected = vec!["echo", r#"hello"insidequotesscript""#];
        let result = parse_parts(input.to_string());
        assert_eq!(result, expected);
    }
}
