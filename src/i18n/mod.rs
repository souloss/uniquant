pub mod fluent;

use std::{collections::HashMap, sync::OnceLock};
use anyhow::Result;
use unic_langid::LanguageIdentifier;

/// 使用 unic-langid 的语言标识符
pub type Locale = LanguageIdentifier;

/// 简化的错误类型 - 使用 anyhow
pub type I18nResult<T> = Result<T>;

/// 类型化的翻译参数值
#[derive(Debug, Clone)]
pub enum ArgValue {
    String(String),
    Number(f64),
    Boolean(bool),
    List(Vec<ArgValue>),
}

impl ArgValue {
    pub fn as_str(&self) -> Option<&str> {
        match self {
            ArgValue::String(s) => Some(s),
            _ => None,
        }
    }
    
    pub fn as_number(&self) -> Option<f64> {
        match self {
            ArgValue::Number(n) => Some(*n),
            _ => None,
        }
    }
    
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            ArgValue::Boolean(b) => Some(*b),
            _ => None,
        }
    }
    
    pub fn as_list(&self) -> Option<&Vec<ArgValue>> {
        match self {
            ArgValue::List(l) => Some(l),
            _ => None,
        }
    }
}

// 从基本类型创建 ArgValue
impl From<String> for ArgValue {
    fn from(s: String) -> Self {
        ArgValue::String(s)
    }
}

impl From<&str> for ArgValue {
    fn from(s: &str) -> Self {
        ArgValue::String(s.to_string())
    }
}

impl From<i32> for ArgValue {
    fn from(n: i32) -> Self {
        ArgValue::Number(n as f64)
    }
}

impl From<i64> for ArgValue {
    fn from(n: i64) -> Self {
        ArgValue::Number(n as f64)
    }
}

impl From<f64> for ArgValue {
    fn from(n: f64) -> Self {
        ArgValue::Number(n)
    }
}

impl From<bool> for ArgValue {
    fn from(b: bool) -> Self {
        ArgValue::Boolean(b)
    }
}

impl From<Vec<ArgValue>> for ArgValue {
    fn from(v: Vec<ArgValue>) -> Self {
        ArgValue::List(v)
    }
}

/// 翻译参数集合
#[derive(Debug, Clone, Default)]
pub struct TranslateArgs {
    args: HashMap<String, ArgValue>,
}

impl From<HashMap<String, String>> for TranslateArgs {
    fn from(v: HashMap<String, String>) -> Self {
        let args = v.into_iter()
            .map(|(k, s)| (k, ArgValue::String(s)))
            .collect();
        TranslateArgs { args }
    }
}

impl TranslateArgs {
    pub fn new() -> Self {
        Self {
            args: HashMap::new(),
        }
    }
    
    pub fn add<T: Into<ArgValue>>(mut self, key: &str, value: T) -> Self {
        self.args.insert(key.to_string(), value.into());
        self
    }
    
    pub fn get(&self, key: &str) -> Option<&ArgValue> {
        self.args.get(key)
    }
    
    pub fn contains_key(&self, key: &str) -> bool {
        self.args.contains_key(key)
    }
    
    pub fn iter(&self) -> impl Iterator<Item = (&String, &ArgValue)> {
        self.args.iter()
    }
    
    pub fn len(&self) -> usize {
        self.args.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.args.is_empty()
    }
}

/// 简化的国际化核心 trait
pub trait I18nBackend: Send + Sync {
    /// 获取带参数的翻译文本
    fn translate(
        &self, 
        key: &str, 
        locale: &Locale, 
        args: &TranslateArgs
    ) -> I18nResult<String>;
    
    /// 获取当前区域设置
    fn current_locale(&self) -> Locale;
    
    /// 设置当前区域设置
    fn set_locale(&self, locale: Locale) -> I18nResult<()>;
    
    /// 获取支持的区域设置列表
    fn supported_locales(&self) -> Vec<Locale>;
}

/// 全局国际化管理器
pub struct GlobalI18n {
    backend: OnceLock<Box<dyn I18nBackend>>,
}

impl GlobalI18n {
    /// 获取全局实例
    pub fn get() -> &'static Self {
        static INSTANCE: GlobalI18n = GlobalI18n {
            backend: OnceLock::new(),
        };
        &INSTANCE
    }
    
    /// 初始化全局实例（使用自定义后端）
    pub fn init_with_backend<B: I18nBackend + 'static>(&self, backend: B) -> I18nResult<()> {
        self.backend.set(Box::new(backend))
            .map_err(|_| anyhow::anyhow!("GlobalI18n already initialized"))
    }
    
    /// 获取后端引用
    pub fn backend(&self) -> I18nResult<&dyn I18nBackend> {
        self.backend.get()
            .ok_or_else(|| anyhow::anyhow!("GlobalI18n not initialized. Call init_with_backend() first."))
            .map(|b| b.as_ref())
    }
    
    /// 翻译文本
    pub fn t(&self, key: &str, args: TranslateArgs) -> I18nResult<String> {
        let backend = self.backend()?;
        let locale = backend.current_locale();
        backend.translate(key, &locale, &args)
    }
    
    /// 指定语言翻译文本
    pub fn t_locale(&self, key: &str, locale: &Locale, args: TranslateArgs) -> I18nResult<String> {
        let backend = self.backend()?;
        backend.translate(key, locale, &args)
    }

    /// 设置当前语言
    pub fn set_locale(&self, locale: Locale) -> I18nResult<()> {
        let backend = self.backend()?;
        backend.set_locale(locale)
    }
    
    /// 获取当前语言
    pub fn current_locale(&self) -> I18nResult<Locale> {
        let backend = self.backend()?;
        Ok(backend.current_locale())
    }
    
    /// 获取支持的语言列表
    pub fn supported_locales(&self) -> I18nResult<Vec<Locale>> {
        let backend = self.backend()?;
        Ok(backend.supported_locales())
    }
}

/// 全局函数
pub fn t(key: &str, args: TranslateArgs) -> I18nResult<String>  {
    GlobalI18n::get().t(key, args)
}

pub fn t_locale(key: &str, locale: &Locale, args: TranslateArgs) -> I18nResult<String>  {
    GlobalI18n::get().t_locale(key, locale, args)
}

pub fn supported_locales() -> I18nResult<Vec<Locale>> {
    GlobalI18n::get().supported_locales()
}

pub fn set_locale(locale: Locale) -> I18nResult<()> {
    GlobalI18n::get().set_locale(locale)
}

pub fn current_locale() -> I18nResult<Locale> {
    GlobalI18n::get().current_locale()
}