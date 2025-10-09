use std::collections::{HashMap, HashSet};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct DiffResult {
    /// 多语言文件中所有 key 的联合集合
    pub all_keys: HashSet<String>,
    /// 每个语言缺失的 key 列表
    pub missing: HashMap<String, Vec<String>>,
    /// 每个语言多余的 key 列表
    pub extra: HashMap<String, Vec<String>>,
}

/// 比较各语言 key 集合，生成一个 DiffResult
pub fn diff_keys(lang_keys: &HashMap<String, HashSet<String>>) -> DiffResult {
    let mut all_keys = HashSet::new();
    for keys in lang_keys.values() {
        for k in keys {
            all_keys.insert(k.clone());
        }
    }

    let mut missing = HashMap::new();
    let mut extra = HashMap::new();

    for (lang, keys) in lang_keys {
        // 计算缺失 = all_keys - keys
        let miss: Vec<String> = all_keys
            .difference(keys)
            .cloned()
            .collect();
        if !miss.is_empty() {
            missing.insert(lang.clone(), miss);
        }

        // 计算多余 = keys - all_keys, 实际上一般空
        let ext: Vec<String> = keys
            .difference(&all_keys)
            .cloned()
            .collect();
        if !ext.is_empty() {
            extra.insert(lang.clone(), ext);
        }
    }

    DiffResult {
        all_keys,
        missing,
        extra,
    }
}
