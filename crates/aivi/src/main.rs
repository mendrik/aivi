use aivi::{
    check_modules, check_types, desugar_target, format_target, load_module_diagnostics,
    load_modules, parse_target, render_diagnostics, AiviError,
};
use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(AiviError::Diagnostics) => ExitCode::FAILURE,
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
            let mut had_errors = false;
            for file in &bundle.files {
                if !file.diagnostics.is_empty() {
                    let rendered = render_diagnostics(&file.path, &file.diagnostics);
                    if !rendered.is_empty() {
                        eprintln!("{rendered}");
                    }
                    had_errors = true;
                }
            }
            if had_errors {
                return Err(AiviError::Diagnostics);
            }
            Ok(())
        }
        "check" => {
            let Some(target) = args.next() else {
                print_help();
                return Ok(());
            };
            let mut diagnostics = load_module_diagnostics(&target)?;
            let modules = load_modules(&target)?;
            diagnostics.extend(check_modules(&modules));
            if diagnostics.is_empty() {
                diagnostics.extend(check_types(&modules));
            }
            if diagnostics.is_empty() {
                return Ok(());
            }
            for diag in diagnostics {
                let rendered = render_diagnostics(&diag.path, std::slice::from_ref(&diag.diagnostic));
                if !rendered.is_empty() {
                    eprintln!("{rendered}");
                }
            }
            Err(AiviError::Diagnostics)
        }
        "fmt" => {
            let Some(target) = args.next() else {
                print_help();
                return Ok(());
            };
            let formatted = format_target(&target)?;
            print!("{formatted}");
            Ok(())
        }
        "desugar" => {
            let Some(target) = args.next() else {
                print_help();
                return Ok(());
            };
            let diagnostics = load_module_diagnostics(&target)?;
            if !diagnostics.is_empty() {
                for diag in diagnostics {
                    let rendered =
                        render_diagnostics(&diag.path, std::slice::from_ref(&diag.diagnostic));
                    if !rendered.is_empty() {
                        eprintln!("{rendered}");
                    }
                }
                return Err(AiviError::Diagnostics);
            }
            let program = desugar_target(&target)?;
            let output = serde_json::to_string_pretty(&program)
                .map_err(|err| AiviError::Io(std::io::Error::new(std::io::ErrorKind::Other, err)))?;
            println!("{output}");
            Ok(())
        }
        "lsp" | "build" | "run" => {
            println!("{command} is a stub for now.");
            Ok(())
        }
        _ => {
            print_help();
            Err(AiviError::InvalidCommand(command))
        }
    }
}

fn print_help() {
    println!(
        "aivi\n\nUSAGE:\n  aivi <COMMAND>\n\nCOMMANDS:\n  parse <path|dir/...>\n  check <path|dir/...>\n  fmt <path>\n  desugar <path|dir/...>\n  lsp\n  build\n  run\n  -h, --help"
    );
}
