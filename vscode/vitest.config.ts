import { defineConfig } from 'vitest/config';
import path from 'path';

export default defineConfig({
    test: {
        globals: true,
        environment: 'jsdom',
        include: ['src/test/unit/**/*.test.ts'],
        exclude: ['node_modules', 'dist'],
        alias: {
            vscode: path.resolve(__dirname, 'src/__mocks__/vscode.ts'),
        },
    },
});
