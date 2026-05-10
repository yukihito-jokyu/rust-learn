// 19 - 標準ライブラリ雑多（Standard Library Misc）
// このファイルはTODOコメント付きの雛形です。
// guide.mdを参考にして、各TODOの部分を自分で実装してみてください。

use std::thread;
use std::time::Duration;

fn main() {
    println!("=== スレッドの作成 ===");

    // TODO: thread::spawnで新しいスレッドを作成してください
    // メインスレッドとは別のカウントを出力するスレッド

    // TODO: moveクロージャを使ってスレッドにデータを渡してください

    // TODO: join()でスレッドの終了を待機してください

    println!("\n=== チャネル通信 ===");

    // TODO: mpsc::channelでチャネルを作成してください

    // TODO: 送信側(tx)をスレッドに渡してメッセージを送信してください

    // TODO: 受信側(rx)でメッセージを受信して表示してください

    println!("\n=== ファイルの読み書き ===");

    // TODO: ファイルを作成して書き込んでください
    // ヒント: std::fs::File, std::io::Write

    // TODO: ファイルを読み込んで内容を表示してください
    // ヒント: std::fs::read_to_string

    // TODO: ファイルを削除してください
    // ヒント: std::fs::remove_file
}
