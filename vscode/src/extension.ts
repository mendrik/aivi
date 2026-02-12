import * as vscode from "vscode";
import * as fs from "node:fs";
import { spawnSync } from "node:child_process";
import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
} from "vscode-languageclient/node";

let client: LanguageClient | undefined;

export function activate(context: vscode.ExtensionContext) {
  const outputChannel = vscode.window.createOutputChannel("AIVI Language Server");

  const isWindows = process.platform === "win32";
  const serverExe = isWindows ? "aivi-lsp.exe" : "aivi-lsp";
  const bundledServerPath = context.asAbsolutePath(`bin/${serverExe}`);

  const config = vscode.workspace.getConfiguration("aivi");
  const configuredCommand = config.get<string>("server.command");
  const configuredArgs = config.get<string[]>("server.args") ?? [];

  const hasCommand = (cmd: string): boolean => {
    const res = spawnSync(cmd, ["--version"], { stdio: "ignore" });
    return !res.error;
  };

  // Preferred order: user config -> `aivi lsp` -> bundled `aivi-lsp` -> `aivi-lsp` on PATH.
  let serverCommand: string;
  let serverArgs: string[];
  if (configuredCommand && configuredCommand.trim().length > 0) {
    serverCommand = configuredCommand;
    serverArgs = configuredArgs;
  } else if (hasCommand("aivi")) {
    serverCommand = "aivi";
    serverArgs = ["lsp"];
  } else {
    serverCommand = fs.existsSync(bundledServerPath) ? bundledServerPath : "aivi-lsp";
    serverArgs = [];
  }

  if (!isWindows && serverCommand === bundledServerPath && fs.existsSync(bundledServerPath)) {
    try {
      fs.chmodSync(bundledServerPath, 0o755);
    } catch (err) {
      outputChannel.appendLine(`Failed to chmod aivi-lsp: ${String(err)}`);
    }
  }

  const serverOptions: ServerOptions = {
    command: serverCommand,
    args: serverArgs,
  };

  const fileWatchers = [
    vscode.workspace.createFileSystemWatcher("**/*.aivi"),
    vscode.workspace.createFileSystemWatcher("**/aivi.toml"),
    vscode.workspace.createFileSystemWatcher("**/Cargo.toml"),
    vscode.workspace.createFileSystemWatcher("**/specs/**/*"),
    vscode.workspace.createFileSystemWatcher("**/.gemini/skills/**/*"),
  ];
  for (const watcher of fileWatchers) {
    context.subscriptions.push(watcher);
  }

  const clientOptions: LanguageClientOptions = {
    documentSelector: [{ language: "aivi" }],
    synchronize: {
      fileEvents: fileWatchers,
      configurationSection: "aivi",
    },
    outputChannel,
    middleware: {
      provideDocumentFormattingEdits: (document, options, token, next) =>
        next(document, options, token),
      provideDocumentRangeFormattingEdits: (document, range, options, token, next) =>
        next(document, range, options, token),
    },
  };

  client = new LanguageClient("aivi", "Aivi Language Server", serverOptions, clientOptions);
  client.start();

  context.subscriptions.push(
    vscode.commands.registerCommand("aivi.restartServer", async () => {
      outputChannel.appendLine("Restarting AIVI Language Server...");
      const prev = client;
      client = undefined;
      await prev?.stop();
      client = new LanguageClient("aivi", "Aivi Language Server", serverOptions, clientOptions);
      client.start();
    })
  );

  context.subscriptions.push(
    new vscode.Disposable(() => {
      void client?.stop();
    })
  );

}

export function deactivate(): Thenable<void> | undefined {
  if (!client) {
    return undefined;
  }
  return client.stop();
}
