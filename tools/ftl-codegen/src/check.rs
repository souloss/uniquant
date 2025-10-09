use crate::{parser, diff};
use colored::*;
use anyhow::Result;

pub fn run(dir: &str) -> Result<()> {
    let lang_keys = parser::parse_ftl_keys(dir)?;

    let dr = diff::diff_keys(&lang_keys);

    for (lang, miss) in &dr.missing {
        println!(
            "{} missing keys: {:?}",
            lang.red(),
            miss
        );
    }
    for (lang, ext) in &dr.extra {
        println!(
            "{} extra keys: {:?}",
            lang.yellow(),
            ext
        );
    }

    if dr.missing.is_empty() && dr.extra.is_empty() {
        println!("{}", "✅ All translations are consistent".green());
    } else {
        println!("{}", "⚠️ Inconsistencies found".red());
    }

    Ok(())
}
