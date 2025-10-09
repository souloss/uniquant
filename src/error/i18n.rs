use fluent_bundle::{bundle, FluentResource, FluentArgs};
use unic_langid::LanguageIdentifier;
use std::collections::HashMap;
use std::sync::Arc;
use once_cell::sync::Lazy;
// Import the concurrent memoizer for thread-safety
use intl_memoizer::concurrent::IntlLangMemoizer;

// --- MODIFICATION ---
// The generic parameters for FluentBundle have been updated.
// We are now explicitly using the thread-safe `IntlLangMemoizer`.
// The original `fluent_bundle::FluentBundle` is a type alias and does not accept the memoizer generic,
// so we use the full path `bundle::FluentBundle`.
static LOCALIZATION_BUNDLES: Lazy<HashMap<String, Arc<bundle::FluentBundle<FluentResource, IntlLangMemoizer>>>> = Lazy::new(|| {
    let mut bundles = HashMap::new();
    let supported = vec!["en", "zh-CN"];

    for lang in supported {
        let ftl_path = format!("locales/{}.ftl", lang);
        let ftl_content = std::fs::read_to_string(&ftl_path)
            .unwrap_or_else(|_| panic!("Missing translation file: {}", ftl_path));

        let res = FluentResource::try_new(ftl_content)
            .expect("Failed to parse FTL");

        let langid: LanguageIdentifier = lang.parse().unwrap();
        
        // --- MODIFICATION ---
        // This call to `new_concurrent` now works because we are using the correct
        // FluentBundle struct type that has this method.
        let mut bundle = bundle::FluentBundle::new_concurrent(vec![langid]);
        bundle.add_resource(res).expect("Failed to add FTL resource");

        bundles.insert(lang.to_string(), Arc::new(bundle));
    }

    bundles
});

/// Gets a thread-safe FluentBundle for the specified language.
// --- MODIFICATION ---
// The return type is updated to match the type stored in the static HashMap.
pub fn get_bundle(lang: &str) -> Arc<bundle::FluentBundle<FluentResource, IntlLangMemoizer>> {
    LOCALIZATION_BUNDLES
        .get(lang)
        .cloned()
        .or_else(|| LOCALIZATION_BUNDLES.get("en").cloned())
        .expect("Default language bundle not found")
}

/// Translates a key without arguments.
pub fn tr(lang: &str, key: &str) -> String {
    let bundle = get_bundle(lang);
    let msg = bundle.get_message(key).unwrap_or_else(|| {
        panic!("Missing translation key `{}` in {}", key, lang)
    });
    let pattern = msg.value().unwrap();
    let mut errors = vec![];
    bundle.format_pattern(pattern, None, &mut errors).to_string()
}

/// Translates a key with arguments.
pub fn tr_with_args(lang: &str, key: &str, args: &FluentArgs) -> String {
    let bundle = get_bundle(lang);
    let msg = bundle.get_message(key).unwrap_or_else(|| {
        panic!("Missing translation key `{}` in {}", key, lang)
    });
    let pattern = msg.value().unwrap();
    let mut errors = vec![];
    bundle.format_pattern(pattern, Some(args), &mut errors).to_string()
}

