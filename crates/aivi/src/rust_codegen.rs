use crate::hir::HirProgram;
use crate::AiviError;
use crate::{emit_native_rust_source, emit_native_rust_source_lib, kernel, rust_ir};

/// Experimental backend: lower to Kernel -> Rust IR and emit standalone Rust.
///
/// Limitations are those of `rust_ir` + `rustc_backend` (e.g. `match` not supported yet).
pub fn compile_rust_native(program: HirProgram) -> Result<String, AiviError> {
    let kernel = kernel::lower_hir(program);
    let rust_ir = rust_ir::lower_kernel(kernel)?;
    emit_native_rust_source(rust_ir)
}

/// Experimental backend: emit a Rust library with exported definitions.
pub fn compile_rust_native_lib(program: HirProgram) -> Result<String, AiviError> {
    let kernel = kernel::lower_hir(program);
    let rust_ir = rust_ir::lower_kernel(kernel)?;
    emit_native_rust_source_lib(rust_ir)
}
