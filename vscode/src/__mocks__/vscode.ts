import { vi } from 'vitest';

export const window = {
    showInformationMessage: vi.fn(),
    showErrorMessage: vi.fn(),
    createOutputChannel: vi.fn(() => ({
        appendLine: vi.fn(),
        show: vi.fn(),
        dispose: vi.fn(),
    })),
};

export const workspace = {
    getConfiguration: vi.fn(() => ({
        get: vi.fn(),
        update: vi.fn(),
    })),
    onDidChangeConfiguration: vi.fn(),
};

export const commands = {
    registerCommand: vi.fn(),
    executeCommand: vi.fn(),
};

export const languages = {
    registerDocumentFormattingEditProvider: vi.fn(),
};

export const ExtensionContext = vi.fn();

export const Uri = {
    file: vi.fn((path) => ({ fsPath: path })),
    parse: vi.fn(),
};

export const Range = vi.fn();
export const Position = vi.fn();
export const TextEdit = {
    replace: vi.fn(),
};

export enum DiagnosticSeverity {
    Error = 0,
    Warning = 1,
    Information = 2,
    Hint = 3,
}
