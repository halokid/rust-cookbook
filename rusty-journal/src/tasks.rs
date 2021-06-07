
use chrono::{serde::ts_seconds, DateTime, Local, Utc};
use serde::Deserialize;
use serde::Serialize;

use std::fs::{OpenOptions, File};
use std::io::{BufReader, Result, Seek, SeekFrom, Error, ErrorKind};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]    // todo: 后面两个驱动让struct可以支持序列化、反序列化
pub struct Task {
  pub text:  String,

  #[serde(with = "ts_seconds")]
  pub created_at:  DateTime<Utc>,
}

impl Task {
  pub fn new(text: String) -> Task {
    let created_at: DateTime<Utc> = Utc::now();
    Task { text, created_at }
  }
}

impl std::fmt::Display for Task {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let created_at = self.created_at.with_timezone(&Local).format("%F %H:%M");
    // todo: 使用 write! 宏将 Task 表示形式写入 Formatter 值 f。 我们按如下所示表示 Task 类型：, {:<50}：填充了 50 个空格的左对齐字符串。后跟 [{}]：创建任务的日期和时间，并用括号括起。
    write!(f, "{:<50} [{}]", self.text, created_at)
  }
}


// todo: 当我们需要保存结构和枚举实例时，需要考虑序列化。 当我们需要让该数据回到程序时，我们讨论的是反序列化。
// todo: 所有函数都具有相同的返回类型：std::io::Result<()>。 此格式表示返回类型为 I/O 结果。 这种返回类型表明，当我们处理真实世界中的数据时，可能会得到一系列不需要的结果。 Ok 变体只是一个空元组 (())，这是通常不与任何数据关联的类型。 其唯一目的是表明函数返回了 Ok 且没有发生错误。

/// 1. 生成读取器并使用其内容作为任务向量
/// 2. 读取文件后倒回
/// 3. 将修改后的任务列表写回文件
pub fn add_task(journal_path: PathBuf, task: Task) -> Result<()> {
  // open the file
  let mut file = OpenOptions::new()
    .read(true)
    .write(true)
    .create(true)
    .open(journal_path)?;     // todo: ?号表示假如程序发生了错误，就会返回错误，?的写法是省略了用match去匹配的语法糖

  // consume the file's contents as a vector of tasks
  /*
  let mut tasks:  Vec<Task> = match serde_json::from_reader(&file) {
    Ok(tasks) => tasks,
    Err(e) if e.is_eof() => Vec::new(),
    Err(e) => Err(e)?,      // todo: ?号表示假如程序发生了错误，就会返回错误，?的写法是省略了用match去匹配的语法糖
  };

  // rewind the file after reading from it
  // todo: 重新装文件的句柄游标设置到0的位置，那么write的时候， 就会从0开始write
  file.seek(SeekFrom::Start(0))?;
   */
  let mut tasks = collect_tasks(&file)?;

  // write the modified task list back into the file
  tasks.push(task);
  serde_json::to_writer(file, &tasks)?;

  Ok(())
}


// todo: ?的意义在于省略match去匹配， 该语句后面的问号 (?) 用于传播错误，而无需编写太多样板代码。 如果返回的错误与其所在函数的返回类型匹配，那么它是用于提前返回错误的语法糖。 所以这两个代码片段是等效的：
/*
fn function_1() -> Result(Success, Failure) {
    match operation_that_might_fail() {
        Ok(success) => success,
        Err(failure) => return Err(failure),
    }
}

fn function_2() -> Result(Success, Failure) {
    operation_that_might_fail()?
}
 */

fn collect_tasks(mut file: &File) -> Result<Vec<Task>> {
  file.seek(SeekFrom::Start(0))?; // Rewind the file before.
  let tasks = match serde_json::from_reader(file) {
    Ok(tasks) => tasks,
    Err(e) if e.is_eof() => Vec::new(),
    Err(e) => Err(e)?,
  };
  file.seek(SeekFrom::Start(0))?; // Rewind the file after.

  Ok(tasks)
}

/// 1. 读取文件。
/// 2. 收集现有任务（如果有）。
/// 3. 删除位于指定位置的任务（如果有）。
/// 4. 更新的任务向量写回文件中。
///
/// todo: 我们没有创建日志文件。 它不存在。我们在写入文件之前对其进行截断，因为我们要执行删除操作。 所以文件会比原始文件小。 如果忽略此步骤，则倒回后的光标将停在先前写入的文件字节之后，导致 JSON 文件格式不正确。 当我们使用 file.set_len(0) 操作截断文件时，我们应确保将字节写入空白页
pub fn complete_task(journal_path: PathBuf, task_position: usize) -> Result<()> {
  // open the file
  let file = OpenOptions::new()
    .read(true)
    .write(true)
    .open(journal_path)?;

  // consume file's contents as a vector of tasks
  let mut tasks = collect_tasks(&file)?;

  // try to remove the task
  if task_position == 0 || task_position > tasks.len() {
    return Err(Error::new(ErrorKind::InvalidInput, "非法的task ID"));
  }
  tasks.remove(task_position - 1);

  // write the modified task list back into the file
  file.set_len(0)?;
  serde_json::to_writer(file, &tasks)?;

  Ok(())
}

pub fn list_tasks(journal_file: PathBuf) -> Result<()> {
  // open the file
  let file = OpenOptions::new().read(true).open(journal_file)?;

  // parse the file and collect the tasks
  let tasks = collect_tasks(&file)?;

  // enumerate and display tasks, if any
  if tasks.is_empty() {
    println!("任务列表为空");
  } else {
    let mut order:  u32 = 1;
    for task in tasks {
      println!("任务ID{}: {}", order, task);
      order += 1
    }
  }

  Ok(())
}





