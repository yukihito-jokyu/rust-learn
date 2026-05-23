# 19 - 標準ライブラリ雑多（Standard Library Misc）

## 学習内容

- スレッド（Threads）
- チャネル（Channels）
- パニック処理（panic handling）
- ファイルI/O
- プロセス（Process）
- FFI（Foreign Function Interface）

## 参考ページ

https://doc.rust-jp.rs/rust-by-example-ja/std_misc.html

## 内容

### スレッド（Threads）

Rustの標準ライブラリはOSのネイティブスレッド（1:1モデル）を提供します。

```rust
use std::thread;
use std::time::Duration;

fn main() {
    // 基本的なスレッドの作成
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

    // moveクロージャで所有権を移動
    let important_data = String::from("重要なデータ");
    let handle = thread::spawn(move || {
        println!("スレッドで受信: {}", important_data);
    });
    handle.join().unwrap();

    // スレッドの返り値を取得
    let handle = thread::spawn(|| {
        let mut sum = 0;
        for i in 1..=100 {
            sum += i;
        }
        sum
    });
    let result = handle.join().unwrap();
    println!("スレッドの計算結果: {}", result); // 5050

    // Builderでカスタム設定
    let handle = thread::Builder::new()
        .name("my-worker".to_string())
        .stack_size(32 * 1024)
        .spawn(|| {
            println!("スレッド名: {:?}",
                thread::current().name().unwrap_or("名前なし"));
        })
        .unwrap();
    handle.join().unwrap();
}
```

### チャネル（Channels）

チャネルを使ってスレッド間でメッセージを受け渡せます。

```rust
use std::sync::mpsc;  // multi-producer, single-consumer
use std::thread;
use std::time::Duration;

fn main() {
    // 基本的なチャネル
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let messages = vec![
            String::from("こんにちは"),
            String::from("Rustの"),
            String::from("チャネル！"),
        ];
        for msg in messages {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    // 受信（ブロッキング）
    for received in rx {
        println!("受信: {}", received);
    }

    // 複数のプロデューサー
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    let tx2 = tx.clone();

    // 元のtxはもう使えない（moveしている）のでdrop
    drop(tx);

    thread::spawn(move || {
        let msgs = vec![
            String::from("スレッド1: メッセージA"),
            String::from("スレッド1: メッセージB"),
        ];
        for msg in msgs {
            tx1.send(msg).unwrap();
        }
    });

    thread::spawn(move || {
        let msgs = vec![
            String::from("スレッド2: メッセージA"),
            String::from("スレッド2: メッセージB"),
        ];
        for msg in msgs {
            tx2.send(msg).unwrap();
        }
    });

    // 全てのメッセージを受信
    for received in rx {
        println!("受信: {}", received);
    }

    // try_recv - ノンブロッキング受信
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(500));
        tx.send(String::from("遅延メッセージ")).unwrap();
    });

    loop {
        match rx.try_recv() {
            Ok(msg) => {
                println!("受信成功: {}", msg);
                break;
            }
            Err(mpsc::TryRecvError::Empty) => {
                println!("メッセージ待機中...");
                thread::sleep(Duration::from_millis(200));
            }
            Err(mpsc::TryRecvError::Disconnected) => {
                println!("送信側が切断されました");
                break;
            }
        }
    }
}
```

### パニック処理

スレッドのパニックをキャッチできます。

```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        panic!("おっと、パニックしました！");
    });

    // スレッドのパニックを検知
    match handle.join() {
        Ok(_) => println!("スレッド正常終了"),
        Err(e) => println!("スレッドがパニックしました: {:?}", e),
    }
}
```

### ファイルI/O

```rust
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};

fn main() -> io::Result<()> {
    // ファイルの書き込み
    let mut file = File::create("example.txt")?;
    writeln!(file, "こんにちは、Rust！")?;
    writeln!(file, "ファイルI/Oのテスト")?;

    // ファイルの読み込み（全体）
    let content = fs::read_to_string("example.txt")?;
    println!("ファイル内容:\n{}", content);

    // ファイルの読み込み（行ごと）
    let file = File::open("example.txt")?;
    let reader = BufReader::new(file);
    for (i, line) in reader.lines().enumerate() {
        println!("行 {}: {}", i + 1, line?);
    }

    // ファイルのメタデータ
    let metadata = fs::metadata("example.txt")?;
    println!("ファイルサイズ: {} バイト", metadata.len());

    // ファイルの追記
    let mut file = File::options()
        .append(true)
        .open("example.txt")?;
    writeln!(file, "追記された行")?;

    // ファイルの削除
    fs::remove_file("example.txt")?;
    println!("ファイルを削除しました");

    // ディレクトリの作成と削除
    fs::create_dir("test_dir")?;
    println!("ディレクトリを作成しました");
    fs::remove_dir("test_dir")?;
    println!("ディレクトリを削除しました");

    Ok(())
}
```

### プロセス（Process）

```rust
use std::process::Command;

fn main() {
    // コマンドの実行と出力の取得
    let output = Command::new("echo")
        .arg("こんにちは、プロセス！")
        .output()
        .expect("コマンドの実行に失敗しました");

    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    println!("終了コード: {}", output.status);

    // コマンドのステータスだけを取得
    let status = Command::new("ls")
        .arg("-la")
        .status()
        .expect("コマンドの実行に失敗しました");
    println!("終了コード: {}", status);

    // パイプで入力を渡す
    let mut child = Command::new("cat")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("コマンドの起動に失敗しました");

    use std::io::Write;
    if let Some(mut stdin) = child.stdin.take() {
        writeln!(stdin, "パイプからの入力").unwrap();
    }

    let output = child.wait_with_output().unwrap();
    println!("出力: {}", String::from_utf8_lossy(&output.stdout));
}
```

### FFI（Foreign Function Interface）

RustからC言語の関数を呼び出せます。

```rust
// Cargo.tomlに以下を追加:
// [dependencies]
// libc = "0.2"

// externブロックで外部関数を宣言
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    // unsafeブロック内で外部関数を呼び出す
    let x = -5;
    let y = unsafe { abs(x) };
    println!("abs({}) = {}", x, y); // abs(-5) = 5

    // libcクレートを使う場合
    // let y = unsafe { libc::abs(x) };
}
```
