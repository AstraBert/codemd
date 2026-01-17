use std::{fs, io::Read};

use clap::Parser;
use regex::bytes::Regex;
use richrs::prelude::*;

fn extract_code_from_markdown(
    language_code: &str,
    input_file_path: &str,
    output_file_path: Option<String>,
) -> anyhow::Result<()> {
    let file_content = fs::read_to_string(input_file_path)?;
    let regex_pattern = format!(r"```{}\s*([^```]+)\s*```", language_code);
    let re = Regex::new(&regex_pattern)?;
    let mut results = vec![];
    for (_, [code]) in re
        .captures_iter(file_content.as_bytes())
        .map(|c| c.extract())
    {
        results.push(code);
    }
    let mut code_content = String::new();
    for mut result in results {
        let mut code = String::new();
        result.read_to_string(&mut code)?;
        code_content += &code;
    }
    let mut console = Console::new();
    match output_file_path {
        Some(s) => {
            if !code_content.is_empty() {
                fs::write(s, code_content)?;
            } else {
                console.print(
                    "[red bold]Error: no code was found matching the requested language tag[/]",
                )?;
            }
        }
        None => {
            if !code_content.is_empty() {
                let syntax = Syntax::new(code_content, language_code)
                    .line_numbers(true)
                    .theme("base16-ocean.dark");
                console.write_segments(&syntax.render(80))?;
            } else {
                console.print(
                    "[red bold]Error: no code was found matching the requested language tag[/]",
                )?;
            }
        }
    }
    Ok(())
}

fn execute(command: &str) -> anyhow::Result<()> {
    let command_exec: Vec<&str> = command.split(" ").collect();
    let main_command = command_exec[0];
    let mut cmd = if command_exec.len() == 1 {
        std::process::Command::new(main_command)
            .stdin(std::process::Stdio::inherit())
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .spawn()?
    } else {
        let cmd_slice = &command_exec[1..];
        std::process::Command::new(main_command)
            .args(cmd_slice)
            .stdin(std::process::Stdio::inherit())
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .spawn()?
    };
    cmd.wait()?;
    Ok(())
}

/// Extract code from markdown based on specific language tags
#[derive(Parser, Debug)]
#[command(version = "0.2.0")]
#[command(name = "codemd")]
#[command(about, long_about = None)]
struct Args {
    /// Language tag to search for (e.g. `python` or `py` for Python, `typescript` or `ts` for TypeScript..)
    #[arg(short, long)]
    language: String,

    /// Path to the input file (must be markdown)
    #[arg(short, long)]
    input: String,

    /// Path to the output file. If not provided, code will be printed to the console.
    #[arg(short, long, default_value = None)]
    output: Option<String>,

    /// Command use to execute the code (e.g. 'python3 output.py'). Can be used only if an output file is provided
    #[arg(short, long, default_value = None)]
    command: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let mut console = Console::new();
    if args.output.is_none() && args.command.is_some() {
        console.print("[red bold]Error: cannot execute code with the provided command if an output file is not provided[/]")?;
    }
    if args.input.ends_with(".md") {
        extract_code_from_markdown(&args.language, &args.input, args.output)?;
        if args.command.is_some() {
            if let Some(command) = args.command {
                execute(&command)?;
            }
        }
    } else {
        console.print("[red bold]Error: input file should be markdown (.md) extension[/]")?;
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_extract_code_from_markdown_python() {
        let result = extract_code_from_markdown(
            "python",
            "testfiles/test.md",
            Some("testfiles/output.py".to_string()),
        );
        match result {
            Ok(_) => {}
            Err(e) => {
                eprintln!(
                    "An error occurred while running the test: {}",
                    e.to_string()
                );
                // exit the test
                assert!(false);
            }
        }
        let file_content =
            fs::read_to_string("testfiles/output.py").expect("Should be able to read file");
        assert!(file_content.contains("print('hello world!')"));
        assert!(file_content.contains("print()"));
    }

    #[test]
    fn test_extract_code_from_markdown_typescript() {
        let result = extract_code_from_markdown(
            "typescript",
            "testfiles/test.md",
            Some("testfiles/output.ts".to_string()),
        );
        match result {
            Ok(_) => {}
            Err(e) => {
                eprintln!(
                    "An error occurred while running the test: {}",
                    e.to_string()
                );
                // exit the test
                assert!(false);
            }
        }
        let file_content =
            fs::read_to_string("testfiles/output.ts").expect("Should be able to read file");
        assert!(file_content.contains("console.log('Hello world!');"));
        // make sure that the `ts` language tag was not counted, as it is not the target language tag
        assert!(!file_content.contains("console.error('this is an error');"));
    }

    #[test]
    fn test_extract_code_from_markdown_empty() {
        let result = extract_code_from_markdown(
            "bash",
            "testfiles/test.md",
            Some("testfiles/output.sh".to_string()),
        );
        match result {
            Ok(_) => {}
            Err(e) => {
                eprintln!(
                    "An error occurred while running the test: {}",
                    e.to_string()
                );
                // exit the test
                assert!(false);
            }
        }
        let file_exists =
            fs::exists("testfiles/output.sh").expect("Should be able to check file existence");
        assert!(!file_exists);
    }

    #[test]
    fn test_execute_no_error() {
        assert!(execute("echo 'Hello world!'").is_ok());
        // cat tries to access a non-existing file
        // -> error is notified to the user throw inherited stderr
        // -> no error in executing the function
        assert!(execute("cat does_not_exist.txt").is_ok());
    }

    #[test]
    fn test_execute_throws_error() {
        // cat spelled wrong (car) -> command does not exist -> error
        assert!(execute("car README.md").is_err());
    }
}
