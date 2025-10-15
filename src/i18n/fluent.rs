use std::{borrow::Cow, collections::HashMap, fs, sync::{Arc, RwLock}};
use fluent_bundle::{concurrent::FluentBundle, FluentResource, FluentArgs, FluentValue};
use unic_langid::LanguageIdentifier;

use super::{I18nBackend, TranslateArgs, ArgValue, I18nResult, Locale};

/// Fluent-based I18n backend
pub struct FluentBackend {
    bundles: HashMap<LanguageIdentifier, Arc<FluentBundle<FluentResource>>>,
    current_locale: RwLock<LanguageIdentifier>,
    supported: Vec<LanguageIdentifier>,
    default_locale: LanguageIdentifier,
}

impl FluentBackend {
    /// 初始化：从 `locales/` 目录加载所有 `.ftl` 文件
    pub fn new(locales_dir: &str, default_locale: &str) -> I18nResult<Self> {
        let mut bundles = HashMap::new();
        let mut supported = vec![];

        for entry in fs::read_dir(locales_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) != Some("ftl") {
                continue;
            }

            let stem = path.file_stem()
                .and_then(|s| s.to_str())
                .ok_or_else(|| anyhow::anyhow!("Invalid file name"))?;

            let lang: LanguageIdentifier = stem.parse()
                .map_err(|_| anyhow::anyhow!("Invalid language identifier: {}", stem))?;

            let ftl_str = fs::read_to_string(&path)?;
            let resource = FluentResource::try_new(ftl_str)
                .map_err(|(_, errs)| anyhow::anyhow!("FTL parse errors: {:?}", errs))?;

            let mut bundle = FluentBundle::new_concurrent(vec![lang.clone()]);
            bundle.add_resource(resource)
                .map_err(|errs| anyhow::anyhow!("Fluent resource errors: {:?}", errs))?;

            bundles.insert(lang.clone(), Arc::new(bundle));
            supported.push(lang);
        }

        let default_locale: LanguageIdentifier = default_locale.parse()?;
        let current_locale = RwLock::new(default_locale.clone());

        Ok(Self {
            bundles,
            supported,
            current_locale,
            default_locale,
        })
    }

    /// 内部方法：获取 FluentBundle
    fn get_bundle(&self, locale: &LanguageIdentifier) -> Option<Arc<FluentBundle<FluentResource>>> {
        self.bundles.get(locale).cloned()
            .or_else(|| self.bundles.get(&self.default_locale).cloned())
    }

    /// 将 TranslateArgs 转换为 FluentArgs
    fn build_args(args: &TranslateArgs) -> FluentArgs<'static> {
        let mut fargs = FluentArgs::new();
        for (k, v) in args.iter() {
            let fv = match v {
                ArgValue::String(s) => FluentValue::from(s.clone()),
                ArgValue::Number(n) => FluentValue::from(*n),
                ArgValue::Boolean(b) => FluentValue::from(*b as i32),
                ArgValue::List(list) => {
                    let joined = list.iter()
                        .filter_map(|i| i.as_str())
                        .collect::<Vec<_>>()
                        .join(", ");
                    FluentValue::from(joined)
                }
            };
            fargs.set(Cow::Owned(k.clone()), fv);
        }
        fargs
    }

    /// 实际翻译逻辑
    fn do_translate(
        &self,
        key: &str,
        locale: &LanguageIdentifier,
        args: Option<&FluentArgs>,
    ) -> I18nResult<String> {
        let bundle = self
            .get_bundle(locale)
            .ok_or_else(|| anyhow::anyhow!("Locale not loaded: {}", locale))?;

        let msg = bundle
            .get_message(key)
            .ok_or_else(|| anyhow::anyhow!("Missing translation key: {}", key))?;

        let pattern = msg.value().ok_or_else(|| anyhow::anyhow!("Message has no value: {}", key))?;
        let mut errors = vec![];
        let value = bundle.format_pattern(pattern, args, &mut errors);
        if !errors.is_empty() {
            return Err(anyhow::anyhow!("Formatting errors: {:?}", errors));
        }
        Ok(value.to_string())
    }
}

impl I18nBackend for FluentBackend {
    fn translate(
        &self,
        key: &str,
        locale: &Locale,
        args: &TranslateArgs,
    ) -> I18nResult<String> {
        let fargs = Self::build_args(args);
        self.do_translate(key, locale, Some(&fargs))
    }

    fn current_locale(&self) -> Locale {
        self.current_locale.read().unwrap().clone()
    }

    fn set_locale(&self, locale: Locale) -> I18nResult<()> {
        if !self.supported_locales().contains(&locale) {
            return Err(anyhow::anyhow!("Unsupported locale: {}", locale));
        }
        *self.current_locale.write().unwrap() = locale;
        Ok(())
    }

    fn supported_locales(&self) -> Vec<Locale> {
        self.supported.clone()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    fn setup_backend() -> FluentBackend {
        let locales_dir = Path::new("/root/opensources/uniquant/configs/locales");
        FluentBackend::new(locales_dir.to_str().unwrap(), "en").unwrap()
    }

    #[test]
    fn test_translate_with_args() {
        let backend = setup_backend();
        let args = TranslateArgs::new().add("message", "Alice");

        let result_en = backend.translate("error-bad_request", &"en".parse().unwrap(), &args).unwrap();
        let result_zh = backend.translate("error-bad_request", &"zh-CN".parse().unwrap(), &args).unwrap();
        println!("{}", backend.default_locale.to_string());
        println!("{:?}", backend.supported);
        assert_eq!(result_en, "Bad request: \u{2068}Alice\u{2069}");
        assert_eq!(result_zh, "请求错误: \u{2068}Alice\u{2069}");
    }
}