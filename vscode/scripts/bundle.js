#!/usr/bin/env node
"use strict";

const esbuild = require("esbuild");
const fs = require("node:fs");
const path = require("node:path");

const vscodeDir = path.resolve(__dirname, "..");
const entry = path.join(vscodeDir, "src", "extension.ts");
const outFile = path.join(vscodeDir, "dist", "extension.js");
const watch = process.argv.includes("--watch");

fs.mkdirSync(path.dirname(outFile), { recursive: true });

const buildOptions = {
  entryPoints: [entry],
  outfile: outFile,
  bundle: true,
  platform: "node",
  format: "cjs",
  target: "node18",
  sourcemap: true,
  external: ["vscode"],
  logLevel: "info",
};

async function run() {
  if (watch) {
    const ctx = await esbuild.context(buildOptions);
    await ctx.watch();
    process.stdout.write("Watching for changes...\n");
    return;
  }

  await esbuild.build(buildOptions);
}

run().catch((err) => {
  process.stderr.write(`${err instanceof Error ? err.message : String(err)}\n`);
  process.exitCode = 1;
});
