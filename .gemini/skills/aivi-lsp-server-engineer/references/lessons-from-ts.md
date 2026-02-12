Lessons for building an AIVI language server

The AIVI language (see the uploaded specification) is a statically typed, purely functional language with a WASM/WASI target and effect‑tracking. To provide a seamless development experience comparable to TypeScript in VS Code, you need a Language Server and a VS Code extension that acts as the client. From the TypeScript example we can extract the following principles:

Separate the entry point from the core session implementation. The server.ts file doesn’t implement language features; it sets up logging and environment handling and delegates to a session. For AIVI you can provide a small Node (or Rust) wrapper that instantiates the compiler and starts the server. This design makes it easy to run the same language server in multiple environments (Node for VS Code, WASI for other editors) and to test the server independently of VS Code.

Expose a StartSessionOptions‑like configuration interface. TypeScript passes options such as plugin lists, inferred‑project behaviour and whether to suppress diagnostics. AIVI could accept similar options: list of domains/effects to load, strictness flags, concurrency options, etc. Define a plain TypeScript interface for these options and read them from command‑line arguments or environment variables.

Redirect side‑effects through a logger. The TypeScript server overrides console.log/warn/error so that third‑party code cannot corrupt the protocol. Your AIVI server should provide a logger that writes to a file or the VS Code output channel and patch global logging functions. Avoid printing directly to stdout/stderr; reserve those streams for the JSON‑RPC protocol.

Structure your server in a functional, composable way. According to the user’s coding preferences, avoid classes; instead, use plain objects and functions. Define a createLogger function returning an object with info, warn, error methods. Define pure functions for request handlers (e.g. handleDefinition, handleCompletion) that accept the current compiler state and a request and return a response without side‑effects. Effects such as reading files or spawning worker threads should be isolated behind thin wrappers.

Use a unified compiler API for both CLI and LSP. TypeScript’s language service uses the same parser, type‑checker and module‑resolver as the compiler. For AIVI, expose your core compiler (parser, desugaring, type inference, domain resolution) as pure functions in a shared package (e.g. @aivi/compiler). Both aivi check and the language server should call these functions. This ensures consistent behaviour and reduces duplication.

Provide incremental project management. TypeScript maintains a project graph and updates it on file changes. AIVI should maintain a module graph with symbol tables and type information. On textDocument/didChange, re‑parse only the affected files and propagate type‑checking to dependent modules. To enable fast feedback, perform these computations in worker threads or the Rust core and cache intermediate results.

Integrate with VS Code via the Language Server Protocol (LSP). The Language Server Extension Guide
 shows how to use the vscode‑languageserver Node library to implement an LSP server. The server must respond to initialize, textDocument/didOpen, didChange, hover, definition, completion, references, rename, codeAction and other requests. The example server uses a TextDocuments manager to track document contents and incremental edits. Use similar utilities or write your own to manage AIVI source files.

Agent instructions for creating the AIVI VS Code extension and language server

Below is a high‑level plan for building the AIVI tooling, distilled into a set of actionable steps. Each step is phrased as an instruction for an automated agent or developer.

Define shared types and compiler API.

Start by designing TypeScript interfaces for the AIVI compiler API: ParseResult, TypeCheckResult, SymbolInfo, Diagnostic, etc. These should live in a package such as @aivi/compiler. Implement pure functions parseModule, typeCheck, resolveSymbol, provideCompletions. Avoid classes and global mutable state; all functions should be pure and referentially transparent. Use curried functions and remeda for functional utilities where appropriate.

Create the language‑server core.

Implement a function createAiviLanguageServer(options: ServerOptions): LanguageServer. The server should return an object exposing initialize, shutdown and handlers for each LSP request. Use the vscode‑languageserver library’s createConnection and TextDocuments utilities as shown in the official sample.

In initialize, detect client capabilities (configuration support, workspace folders) and return your server’s capabilities (incremental text sync, completion, hover, definition, references, rename, formatting, diagnostics, code actions).

On didOpen/didChange, call your compiler’s parseModule and typeCheck functions and publish diagnostics. Cache module graphs in memory so that later requests (hover, definition) can reuse the AST and type information.

For definition, resolve the symbol at the given position and return its declaration location; for hover, return the type and documentation. For completion, provide a list of possible identifiers, keywords, domains and effects appropriate to the current context. Always return immutable arrays; do not mutate state in place.

Wrap the server in a small CLI.

Create a small Node entry file packages/aivi-lsp/src/server.ts analogous to TypeScript’s tsserver/server.ts. It should parse command‑line arguments (e.g. --logFile, --domains) and environment variables. Use these to build a ServerOptions object and call createAiviLanguageServer. Set up logging: redirect console.log/warn/error to your logger so that any plugin or domain code writes through the log. Call server.start() to begin listening on stdin/stdout or a socket. Keep this file minimal; put all heavy logic in the shared server core.

Implement a VS Code client extension.

In the VS Code extension (e.g. packages/aivi-vscode-extension), declare a contributes.languages section for AIVI with file extensions .aivi and .av. Use the vscode-languageclient/node package to spawn your server from the compiled server.js. In the activate function, create a LanguageClient with an appropriate document selector and start it. Stop the client in deactivate.

Define language configuration (comments, brackets, auto‑closing pairs) and basic syntax highlighting (using TextMate or tree‑sitter). Provide snippets for common AIVI constructs (module definitions, ADT declarations, effect scopes).

Support workspace settings and CLI integration.

Expose configuration options (e.g. aivi.trace.server, aivi.domainPaths) via VS Code settings. In the server, respond to workspace/configuration requests to update behaviour at runtime. Use the AIVI CLI (aivi fmt, aivi check, aivi desugar) behind the scenes for tasks such as formatting or building.

Provide commands in the extension for running unit tests, compiling to WASM, and generating MCP schemas.

Plan for plugin and domain extensibility.

Design your server’s ServerOptions to accept a list of domain/plugin packages to load at startup (similar to TypeScript’s --globalPlugins and --pluginProbeLocations). Each plugin should conform to a simple interface (e.g. activate(server: LanguageServer): void) that can register additional completion items, code actions or diagnostics.

When creating your logger and server host, ensure that domain plugins cannot perform uncontrolled I/O; provide APIs that enforce effect tracking and security.

By following the above steps you can build a modular, functional AIVI language server that leverages the lessons from TypeScript’s tsserver. The separation of concerns (thin entry point, shared compiler API, pure request handlers) will make the tooling robust and easy to maintain, and the VS Code extension will provide a first‑class developer experience.