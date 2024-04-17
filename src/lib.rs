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
use std::{fs, str::Split};
pub fn process_query(command: Command) -> Option<QueryResult> {
    let mut search_tokens: Vec<String> = vec![command.default_expression];
    let mut search_directory: bool = false;
    let mut query_result: QueryResult = QueryResult::new();
    for option in command.options {
        match option {
            GrepOption::Expression(_expression) => {
                search_tokens.push(_expression);
            },
            GrepOption::DirectorySearch => {
                search_directory = true;
            }
            _ => ()
        }
    }
    if (search_directory){
        for file in command.file_directories {
            let mut file_lines: Split<'static, char>;
            {
                let file_content = fs::read_to_string(file).expect("rsgrep: Can't open file.");
                file_lines = file_content.clone().split('\n');

            }
            for (i, line) in file_lines.into_iter().enumerate() {
                query_result.results.push(Line {
                    line_number: i,
                    line_content: line.to_owned()
                })
            }

            
        }
    }
    else {
        panic!("rsgrep: Feature not implemented");
    }   
    None
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

impl QueryResult{
    fn new() -> QueryResult {
        QueryResult {
            results: Vec::new()
        }
    }
}
#[derive(Debug)]

pub struct Command {
    options: Vec<GrepOption>,
    file_directories: Vec<String>,
    default_expression: String,
}
pub struct QueryResult {
    results: Vec<Line>
}
pub struct Line {
    line_number: usize,
    line_content: String,
}

