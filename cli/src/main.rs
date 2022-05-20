//https://docs.microsoft.com/zh-cn/learn/modules/rust-create-command-line-program/3-cli-module?source=learn

mod cli;
use structopt::StructOpt;

fn main() {
    println!("{:#?}", cli::CommandlineArgs::from_args());
}


 
