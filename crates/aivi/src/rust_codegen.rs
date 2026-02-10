use crate::hir::HirProgram;
use crate::AiviError;

pub fn compile_rust(program: HirProgram) -> Result<String, AiviError> {
    let json = serde_json::to_vec(&program)
        .map_err(|err| AiviError::Codegen(format!("failed to serialize program: {err}")))?;
    let escaped = escape_bytes(&json);

    let mut output = String::new();
    output.push_str("use aivi::HirProgram;\n\n");
    output.push_str("const PROGRAM_JSON: &[u8] = b\"");
    output.push_str(&escaped);
    output.push_str("\";\n\n");
    output.push_str("fn build_program() -> HirProgram {\n");
    output.push_str("    serde_json::from_slice(PROGRAM_JSON)\n");
    output.push_str("        .expect(\"deserialize embedded AIVI program\")\n");
    output.push_str("}\n\n");
    output.push_str("fn main() {\n");
    output.push_str("    let program = build_program();\n");
    output.push_str("    if let Err(err) = aivi::run_native(program) {\n");
    output.push_str("        eprintln!(\"{err}\");\n");
    output.push_str("        std::process::exit(1);\n");
    output.push_str("    }\n");
    output.push_str("}\n");

    Ok(output)
}

pub fn compile_rust_lib(program: HirProgram) -> Result<String, AiviError> {
    let json = serde_json::to_vec(&program)
        .map_err(|err| AiviError::Codegen(format!("failed to serialize program: {err}")))?;
    let escaped = escape_bytes(&json);

    let mut output = String::new();
    output.push_str("use aivi::HirProgram;\n\n");
    output.push_str("const PROGRAM_JSON: &[u8] = b\"");
    output.push_str(&escaped);
    output.push_str("\";\n\n");
    output.push_str("pub fn build_program() -> HirProgram {\n");
    output.push_str("    serde_json::from_slice(PROGRAM_JSON)\n");
    output.push_str("        .expect(\"deserialize embedded AIVI program\")\n");
    output.push_str("}\n\n");
    output.push_str("pub fn run() -> Result<(), aivi::AiviError> {\n");
    output.push_str("    aivi::run_native(build_program())\n");
    output.push_str("}\n");

    Ok(output)
}

fn escape_bytes(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len());
    for byte in bytes {
        for escaped in std::ascii::escape_default(*byte) {
            out.push(escaped as char);
        }
    }
    out
}
