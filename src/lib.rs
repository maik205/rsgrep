pub fn parse_arguments(args: Vec<String>) -> Option<Command> {
    let mut iter = args.iter();
    let mut part: Option<&String>;
    let mut option: Vec<GrepOption> = Vec::new();
    iter.next();
    loop {
        part = iter.next();
        
        // Check for arguments
        if part.expect("Should be able to unwrap").starts_with("-") {
            let opts = part.expect("Invalid options");
            
            let opts_chars: Vec<char> = opts.chars().collect();
            for char in &opts_chars[1..] {
                match char {
                    'c' => {
                        option.push(GrepOption::Count);
                    }
                    'i' => {
                        option.push(GrepOption::IgnoreCase);
                    }
                    'l' => {
                        option.push(GrepOption::OnlyFileName);
                    }
                    'e' => {
                        option.push(GrepOption::Expression(
                            iter.next()
                                .expect("rsgrep: Please specify the correct argument.")
                                .to_owned(),
                        ));
                    }
                    _ => {
                        println!("rsgrep: Unimplemented");
                        return None;
                    }
                }
            }
        }
        //
        else {
            break;
        }
    }
    
    println!("{:?}",part);
    
    return Some(Command {
        default_expression: part.expect("rsgrep: Invalid expression.").to_owned(),
        options: option,
        file_directories: iter.map(|x| x.to_owned()).collect(),
    });
}


#[derive(Debug)]
pub enum GrepOption {
    Count,
    IgnoreCase,
    IgnoreFileName,
    OnlyFileName,
    DirectorySearch,
    Expression(String),
}
impl Command {
    fn new() -> Command {
        Command {
            options: Vec::new(),
            file_directories: Vec::new(),
            default_expression: String::new(),
        }
    }
    
}
#[derive(Debug)]

pub struct Command {
    options: Vec<GrepOption>,
    file_directories: Vec<String>,
    default_expression: String,
}
pub struct Result {
    command: Command,
    results: Vec<Line>,
}
pub struct Line {
    line_number: u32,
    line_content: String,
}

