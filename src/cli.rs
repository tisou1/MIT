use clap::{Parser, Subcommand};
use mit::commands::add::add;
use mit::commands::commit::commit;
use mit::commands::init::init;

/// Rust实现的简易版本的Git，用于学习Rust语言
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The subcommand to run.
    #[clap(subcommand)]
    command: Command,
}
/// @see <a href="https://juejin.cn/post/7242623208825110586">Rust Clap库学习 - 掘金</a>
#[derive(Subcommand)]
enum Command {
    /// 初始化仓库
    Init,
    /// 添加文件到暂存区
    /// @see <a href="https://juejin.cn/post/7053831273277554696">git add .，git add -A，git add -u，git add * 的区别与联系</a>
    Add {
        /// 要添加的文件
        files: Vec<String>,

        /// 将工作区中所有的文件改动提交至暂存区（包括新增、修改和删除）
        #[clap(short = 'A', long)]
        all: bool,

        /// 将工作区中已跟踪的文件(tracked)更新到暂存区（修改 & 删除）；But不包含新增
        #[clap(short, long)]
        update: bool,
    },
    /// 删除文件
    Rm {
        /// 要删除的文件
        files: Vec<String>,
        /// flag 删除暂存区的文件
        #[clap(long, action)]
        cached: bool,
    },
    /// 提交暂存区的文件
    Commit {
        #[clap(short, long)]
        message: String,

        #[clap(long, action)]
        allow_empty: bool,
    },
}
pub fn handle_command() {
    let cli = Cli::parse();
    match cli.command {
        Command::Init => {
            let _ = init();
        }
        Command::Add { files, all, update } => {
            add(files, all, update);
        }
        Command::Rm { files, cached } => {
            println!("rm: {:?}, cached= {}", files, cached);
        }
        Command::Commit {
            message,
            allow_empty,
        } => {
            commit(message, allow_empty);
        }
    }
}
