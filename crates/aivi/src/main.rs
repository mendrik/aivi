use aivi::{parse_target, AiviError};
use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), AiviError> {
    let mut args = env::args().skip(1);
    let Some(command) = args.next() else {
        print_help();
        return Ok(());
    };

    match command.as_str() {
        "-h" | "--help" => {
            print_help();
            Ok(())
        }
        "parse" => {
            let Some(target) = args.next() else {
                print_help();
                return Ok(());
            };
            let bundle = parse_target(&target)?;
            let output = serde_json::to_string_pretty(&bundle)
                .map_err(|err| AiviError::Io(std::io::Error::new(std::io::ErrorKind::Other, err)))?;
            println!("{output}");
            Ok(())
        }
        "check" | "fmt" | "lsp" | "build" | "run" => {
            println!("{command} is a stub for now.");
            Ok(())
        }
        _ => {
            print_help();
            Ok(())
        }
    }
}

fn print_help() {
    println!(
        "aivi\n\nUSAGE:\n  aivi <COMMAND>\n\nCOMMANDS:\n  parse <path|dir/...>\n  check\n  fmt\n  lsp\n  build\n  run\n  -h, --help"
    );
}
