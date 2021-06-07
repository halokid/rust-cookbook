mod cli;
mod tasks;
use structopt::StructOpt;

use cli::{ Action::*, CommandLineArgs };
use tasks::Task;
use crate::cli::Action;
use std::path::PathBuf;
use anyhow::anyhow;


fn find_default_journal_file() -> Option<PathBuf> {
  home::home_dir().map(| mut path | {
    path.push("test-journal.json");
    path
  })
}

/// todo: 首先调用 cargo run --，以确保将 -- 后传递的所有参数发送到程序，而不是发送到 cargo 本身。
/// add task: cargo run -- -j test-journal.json add "water the plants"
/// todo: 实际在生产环境执行 ./target/release/rusty-journal list
///
// fn main() {
fn main() -> anyhow::Result<()> {
  println!("{:#?}", cli::CommandLineArgs::from_args());

  // get the cmmand-line argument
  let CommandLineArgs {
    action,
    journal_file
  } = CommandLineArgs::from_args();

  // unpack the journal file
  // let journal_file = journal_file.expect("查找文件失败");
  let journal_file = journal_file
    .or_else(find_default_journal_file)
    .ok_or(anyhow!("查找文件失败"))?;
    // .expect("查找文件失败");

  // perform the action
  match action {
    Add { text } => tasks::add_task(journal_file, Task::new(text)),
    Done { position } => tasks::complete_task(journal_file, position),
    List => tasks::list_tasks(journal_file),
  }?;
  // .expect("执行action失败")     // todo: 我们在 match 块的末尾调用 .expect，因为所有函数都返回 Result 类型，这可能导致失败， 而main函数是无返回的，所以要定义expect

  Ok(())
}





