use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "ftl-codegen")]
#[command(about = "FTL i18n consistency checker + AppCode generator", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// 校验 ftl 文件 key 一致性
    Check {
        /// 翻译文件目录
        #[arg(short, long, default_value = "./configs/locales")]
        dir: String,
    },
    /// 从 ftl 文件生成 AppCode
    GenCode {
        /// 翻译文件目录
        #[arg(short, long, default_value = "./configs/codes.yaml")]
        codes_path: String,
        #[arg(short, long, default_value = "./configs/locales")]
        locales_dir: String,
        /// 输出 Rust 代码路径
        #[arg(short, long, default_value = "./src/error/code.rs")]
        out: String,
    },
}
