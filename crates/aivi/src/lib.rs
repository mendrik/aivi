use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize)]
pub struct CstFile {
    pub path: String,
    pub byte_count: usize,
    pub line_count: usize,
    pub lines: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct CstBundle {
    pub files: Vec<CstFile>,
}

#[derive(Debug)]
pub enum AiviError {
    Io(std::io::Error),
    InvalidPath(String),
}

impl std::fmt::Display for AiviError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AiviError::Io(err) => write!(f, "IO error: {err}"),
            AiviError::InvalidPath(path) => write!(f, "Invalid path: {path}"),
        }
    }
}

impl std::error::Error for AiviError {}

impl From<std::io::Error> for AiviError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

pub fn parse_target(target: &str) -> Result<CstBundle, AiviError> {
    let mut files = Vec::new();
    let paths = expand_target(target)?;
    for path in paths {
        files.push(parse_file(&path)?);
    }
    Ok(CstBundle { files })
}

pub fn parse_file(path: &Path) -> Result<CstFile, AiviError> {
    let content = fs::read_to_string(path)?;
    let lines: Vec<String> = content.lines().map(|line| line.to_string()).collect();
    let byte_count = content.as_bytes().len();
    let line_count = lines.len();
    Ok(CstFile {
        path: path.display().to_string(),
        byte_count,
        line_count,
        lines,
    })
}

fn expand_target(target: &str) -> Result<Vec<PathBuf>, AiviError> {
    let mut paths = Vec::new();
    let (base, recursive) = match target.strip_suffix("/...") {
        Some(base) => (if base.is_empty() { "." } else { base }, true),
        None => (target, false),
    };

    let Some(path) = resolve_target_path(base) else {
        return Err(AiviError::InvalidPath(target.to_string()));
    };

    if path.is_file() {
        return Ok(vec![path.to_path_buf()]);
    }

    if path.is_dir() {
        if recursive {
            collect_files(&path, &mut paths)?;
        } else {
            for entry in fs::read_dir(path)? {
                let entry = entry?;
                let entry_path = entry.path();
                if entry_path.is_file() {
                    paths.push(entry_path);
                }
            }
        }
    }

    paths.sort();
    if paths.is_empty() {
        return Err(AiviError::InvalidPath(target.to_string()));
    }

    Ok(paths)
}

fn resolve_target_path(target: &str) -> Option<PathBuf> {
    let target_path = Path::new(target);
    if target_path.is_absolute() {
        return target_path.exists().then(|| target_path.to_path_buf());
    }

    if target_path.exists() {
        return Some(target_path.to_path_buf());
    }

    let Ok(mut dir) = std::env::current_dir() else {
        return None;
    };

    loop {
        if dir.join("Cargo.toml").exists() {
            let candidate = dir.join(target);
            if candidate.exists() {
                return Some(candidate);
            }
        }

        let Some(parent) = dir.parent() else {
            break;
        };
        dir = parent.to_path_buf();
    }

    None
}

fn collect_files(dir: &Path, paths: &mut Vec<PathBuf>) -> Result<(), AiviError> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let entry_path = entry.path();
        if entry_path.is_dir() {
            collect_files(&entry_path, paths)?;
            continue;
        }

        if entry_path.extension().and_then(|ext| ext.to_str()) == Some("aivi") {
            paths.push(entry_path);
        }
    }
    Ok(())
}
