// 17 - エラーハンドリング（Error Handling）
// このファイルはTODOコメント付きの雛形です。
// guide.mdを参考にして、各TODOの部分を自分で実装してみてください。

use std::error::Error;
use std::fmt;

fn main() {
    println!("=== panic! の動作確認 ===");

    // TODO: panic!を発生させるコードを書いてみてください
    // 注意: panic!が発生するとプログラムが終了するため、
    // テスト時はコメントアウトしてください

    println!("\n=== Option の使用 ===");

    // TODO: Option値を作成し、match / if let / unwrap で処理してください
    let some_value: Option<i32> = Some(42);
    let none_value: Option<i32> = None;

    // TODO: unwrap_orを使ってデフォルト値を設定してください

    println!("\n=== Result の処理 ===");

    // TODO: Result値を作成し、matchでOk/Errを処理してください
    // TODO: 除算関数を作成しゼロ除算をResultで処理してください

    println!("\n=== ?演算子の使用 ===");

    // TODO: ?演算子を使った関数を定義し、呼び出してください
    // ヒント: 戻り値がResultの関数内で?を使う

    println!("\n=== カスタムエラーの定義 ===");

    // TODO: カスタムエラー型を定義してください
    // - enumでエラーの種類を定義
    // - Displayトレイトを実装
    // - Errorトレイトを実装

    // TODO: カスタムエラーを使う関数を作成してください
}

// TODO: カスタムエラー型をここに定義してください
// #[derive(Debug)]
// enum MyError {
//     ...
// }
//
// impl fmt::Display for MyError { ... }
// impl Error for MyError {}

// TODO: ?演算子を使う関数をここに定義してください
// fn some_function() -> Result<_, _> {
//     ...
// }
