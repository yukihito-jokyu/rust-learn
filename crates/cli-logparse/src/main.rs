// TODO: CLI引数のパーサーを定義する
// TODO: ログファイルのパーサーを実装する
// TODO: フィルタリング処理を実装する
// TODO: 統計情報の集計を実装する
// TODO: 並行処理で高速化する

use chrono::NaiveDateTime;
use clap::Parser;
use core::time;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Parser)]
#[command(name = "logparser")]
struct Cli {
    /// ファイルパス
    #[arg(short, long)]
    logfile: String,

    /// フィルタリング
    #[arg(short, long)]
    filter: Option<String>,

    /// ステータス
    #[arg(short, long)]
    status: bool,
}

#[derive(Debug)]
enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

#[derive(Debug)]
struct LogEntries {
    level: LogLevel,
    messages: Vec<String>,
}

fn main() {
    let cli = Cli::parse();
    let logfile = cli.logfile;
    let filter = cli.filter;
    let status = cli.status;

    println!("ファイル: {}", logfile);

    if let Some(filter) = filter {
        println!("フィルター: {}", filter);
    } else {
        println!("フィルター: 指定なし");
    }

    println!("ステータス: {}", status);

    // ファイル読み込み
    let file = File::open(logfile).expect("ファイルが開けません");
    let reader = BufReader::new(file);

    let re: Regex = Regex::new(r"^(\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}) (\[\w+\]) (.+)$").unwrap();

    let text = "2024-01-15 10:30:45 [ERROR] Database connection failed";
    let caps = re.captures(text).unwrap();

    let timestamp = &caps[1]; // "2024-01-15 10:30:45"
    let level = &caps[2]; // "ERROR"
    let message = &caps[3]; // "Database connection failed"

    println!("timestamp: {}", timestamp);
    println!("level: {}", level);
    println!("message: {}", message);

    // LogEntriesの使い方
    let mut info_logs = LogEntries { level: LogLevel::Info, messages: Vec::new() };
    let mut debug_logs = LogEntries { level: LogLevel::Debug, messages: Vec::new() };
    let mut warn_logs = LogEntries { level: LogLevel::Warn, messages: Vec::new() };
    let mut error_logs = LogEntries { level: LogLevel::Error, messages: Vec::new() };

    for line in reader.lines() {
        let line = line.expect("読み込みエラー");

        if let Some(caps) = re.captures(&line) {
            let level_str = &caps[2];
            let message = &caps[3];
            let timestamp = &caps[1];

            let formatted_massage = format!("{} {} {}", level_str, timestamp, message);

            match level_str {
                "[INFO]" => info_logs.messages.push(formatted_massage),
                "[DEBUG]" => debug_logs.messages.push(formatted_massage),
                "[WARN]" => warn_logs.messages.push(formatted_massage),
                "[ERROR]" => error_logs.messages.push(formatted_massage),
                _ => println!("未知のログレベル: {}", level_str),
            }
        }
    }

    // 結果の確認
    println!("{:#?}", info_logs.messages);
    println!("{:#?}", error_logs.messages);
}
