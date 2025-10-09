use fluent_syntax::parser::parse;
use fluent_syntax::ast;
use anyhow::{Result, anyhow};
use std::fs;
use std::collections::{HashMap, HashSet};
use walkdir::WalkDir;

pub fn parse_ftl_keys(dir: &str) -> Result<HashMap<String, HashSet<String>>> {
    let mut lang_keys = HashMap::new();

    for entry in WalkDir::new(dir).into_iter().filter_map(Result::ok) {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        // 只认 .ftl 文件
        if path.extension().and_then(|s| s.to_str()) == Some("ftl") {
            let stem = path.file_stem().unwrap().to_string_lossy();
            let lang = stem.to_string();
            let content = fs::read_to_string(path)?;
            let resource = parse(content.as_str())
                .map_err(|e| anyhow!("Parse error in {}: {:?}", path.display(), e))?;

            let mut keys = HashSet::new();
            for entry in resource.body.into_iter() {
                if let ast::Entry::Message(msg) = entry {
                    keys.insert(msg.id.name.to_string());
                }
            }
            lang_keys.insert(lang, keys);
        }
    }

    Ok(lang_keys)
}