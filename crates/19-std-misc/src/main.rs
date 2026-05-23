// 19 - 標準ライブラリ雑多（Standard Library Misc）
// このファイルはTODOコメント付きの雛形です。
// guide.mdを参考にして、各TODOの部分を自分で実装してみてください。

use std::thread;
use std::time::Duration;

use std::process::Command;

fn main() {
    println!("=== スレッドの作成 ===");

    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("スレッド内: カウント {}", i);
            thread::sleep(Duration::from_millis(100));
        }
    });

    // メインスレッドでの処理
    for i in 1..3 {
        println!("メイン: カウント {}", i);
        thread::sleep(Duration::from_millis(100));
    }

    // スレッドの終了を待機
    handle.join().unwrap();
    println!("スレッド終了");

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

    let output = Command::new("echo")
        .arg("こんにちは、プロセス！")
        .output()
        .expect("コマンドの実行に失敗しました");

    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    println!("終了コード: {}", output.status);

    let status = Command::new("ls").arg("-la").status().expect("コマンドの実行に失敗しました");
    println!("終了コード: {}", status);
}
