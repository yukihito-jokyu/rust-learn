# 17 - エラーハンドリング（Error Handling）

## 学習内容
- `panic!`と回復不可能なエラー
- `Option`と`unwrap`
- `Result`型
- `?`演算子
- エラーの伝播
- カスタムエラー型
- `Box<dyn Error>`によるエラーの抽象化

## 参考ページ
https://doc.rust-jp.rs/rust-by-example-ja/error.html

## 内容

### panic! - 回復不可能なエラー

`panic!`は、プログラムが復旧できないエラーに遭遇したときに使用します。スレッドをパニックさせ、エラーメッセージを出力してプログラムを終了します。

```rust
fn give_a(drink: &str) {
    if drink == "レモネード" {
        println!("レモネードください！");
    } else {
        panic!("レモネード以外は飲みません！！");
    }
}

fn main() {
    give_a("レモネード"); // OK
    // give_a("コーヒー"); // panic!が発生: レモネード以外は飲みません！！
}
```

### Option と unwrap

`Option<T>`は値が存在するかもしれないことを表す型です。

```rust
// Option<T> の定義
// enum Option<T> {
//     Some(T),
//     None,
// }

// 配列の安全なアクセス
fn get_item(vec: &Vec<i32>, index: usize) -> Option<&i32> {
    vec.get(index) // 範囲外ならNoneを返す
}

fn main() {
    let numbers = vec![1, 2, 3];

    // unwrap - 値を取り出す。Noneならpanic!
    let first = numbers.get(0).unwrap();
    println!("最初の要素: {}", first); // 1

    // match で安全に処理
    match numbers.get(5) {
        Some(val) => println!("値: {}", val),
        None => println!("インデックスが範囲外です"),
    }

    // if let で簡潔に
    if let Some(val) = numbers.get(2) {
        println!("3番目: {}", val); // 3
    }

    // unwrap_or でデフォルト値を指定
    let val = numbers.get(10).unwrap_or(&0);
    println!("デフォルト付き: {}", val); // 0

    // unwrap_or_else でクロージャでデフォルト値を生成
    let val = numbers.get(10).unwrap_or_else(|| {
        println!("デフォルト値を使用");
        &0
    });
}
```

### Result型

`Result<T, E>`は操作が成功（`Ok(T)`）または失敗（`Err(E)`）する可能性があることを表します。

```rust
// Result<T, E> の定義
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

use std::fs::File;
use std::io::Read;

fn read_file(path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("ゼロ除算エラー"))
    } else {
        Ok(a / b)
    }
}

fn main() {
    // Resultの基本的な使い方
    match divide(10.0, 3.0) {
        Ok(result) => println!("10 / 3 = {:.2}", result),
        Err(e) => println!("エラー: {}", e),
    }

    match divide(10.0, 0.0) {
        Ok(result) => println!("10 / 0 = {}", result),
        Err(e) => println!("エラー: {}", e), // ゼロ除算エラー
    }

    // unwrap - Okなら値を返す、Errならpanic!
    let result = divide(6.0, 2.0).unwrap();
    println!("6 / 2 = {}", result); // 3

    // expect - エラーメッセージをカスタマイズ
    let result = divide(8.0, 2.0).expect("除算に失敗しました");
    println!("8 / 2 = {}", result); // 4

    // unwrap_or, unwrap_or_else, unwrap_or_default
    let result = divide(10.0, 0.0).unwrap_or(f64::INFINITY);
    println!("デフォルト付き: {}", result); // inf

    // is_ok, is_err で判定
    if divide(10.0, 2.0).is_ok() {
        println!("計算成功");
    }
}
```

### ?演算子

`?`演算子は、`Result`や`Option`のエラー伝播を簡潔に書くための構文です。`Err`のときは早期リターンし、`Ok`のときは値を取り出します。

```rust
use std::fs::File;
use std::io::{self, Read};

// ?を使わない書き方
fn read_username_from_file_manual(path: &str) -> Result<String, io::Error> {
    let file = File::open(path);
    let mut file = match file {
        Ok(f) => f,
        Err(e) => return Err(e),
    };
    let mut username = String::new();
    match file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// ?を使った書き方（同じ意味）
fn read_username_from_file(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut username = String::new();
    file.read_to_string(&mut username)?;
    Ok(username)
}

// チェーンも可能
fn read_username_short(path: &str) -> Result<String, io::Error> {
    let mut username = String::new();
    File::open(path)?.read_to_string(&mut username)?;
    Ok(username)
}

// Option でも ? が使える
fn get_last_char(s: &str) -> Option<char> {
    let last = s.chars().last()?;  // None なら早期リターン
    Some(last)
}

fn main() {
    // Resultでの?の使用
    match read_username_from_file("存在しないファイル.txt") {
        Ok(content) => println!("内容: {}", content),
        Err(e) => println!("ファイル読み込みエラー: {}", e),
    }

    // Optionでの?の使用
    if let Some(c) = get_last_char("Rust") {
        println!("最後の文字: {}", c); // t
    }
}
```

### エイリアス（type alias）

`Result`型を頻繁に使う場合、型エイリアスで簡略化できます。

```rust
use std::num::ParseIntError;

// 型エイリアスの定義
type ParseResultI32 = Result<i32, ParseIntError>;

fn parse_number(s: &str) -> ParseResultI32 {
    s.parse::<i32>()
}

fn main() {
    match parse_number("42") {
        Ok(n) => println!("パース結果: {}", n),
        Err(e) => println!("エラー: {}", e),
    }
}
```

### カスタムエラー型

独自のエラー型を定義することで、アプリケーション固有のエラーを表現できます。

```rust
use std::fmt;
use std::error::Error;

// カスタムエラー型の定義
#[derive(Debug)]
enum AppError {
    IoError(String),
    ParseError(String),
    NotFound(String),
    InvalidInput { field: String, message: String },
}

// Displayトレイトの実装（ユーザー向けメッセージ）
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::IoError(msg) => write!(f, "IO エラー: {}", msg),
            AppError::ParseError(msg) => write!(f, "パース エラー: {}", msg),
            AppError::NotFound(msg) => write!(f, "見つかりません: {}", msg),
            AppError::InvalidInput { field, message } => {
                write!(f, "不正な入力 ({}) : {}", field, message)
            }
        }
    }
}

// Errorトレイトの実装
impl Error for AppError {}

// Fromトレイトの実装で他のエラー型からの変換を可能にする
impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::IoError(err.to_string())
    }
}

impl From<std::num::ParseIntError> for AppError {
    fn from(err: std::num::ParseIntError) -> Self {
        AppError::ParseError(err.to_string())
    }
}

// カスタムエラーを使う関数
fn parse_age(s: &str) -> Result<u32, AppError> {
    let age: u32 = s.parse().map_err(|e| AppError::ParseError(e.to_string()))?;

    if age > 150 {
        return Err(AppError::InvalidInput {
            field: "age".to_string(),
            message: "年齢は150以下でなければなりません".to_string(),
        });
    }

    Ok(age)
}

fn main() {
    // 正常系
    match parse_age("25") {
        Ok(age) => println!("年齢: {}歳", age),
        Err(e) => println!("エラー: {}", e),
    }

    // パースエラー
    match parse_age("abc") {
        Ok(age) => println!("年齢: {}歳", age),
        Err(e) => println!("エラー: {}", e), // パース エラー
    }

    // バリデーションエラー
    match parse_age("200") {
        Ok(age) => println!("年齢: {}歳", age),
        Err(e) => println!("エラー: {}", e), // 不正な入力
    }
}
```

### Box<dyn Error>によるエラーの抽象化

複数の異なるエラー型を扱う場合、`Box<dyn Error>`を使って統一的に処理できます。

```rust
use std::error::Error;
use std::fs;

// Box<dyn Error> を使えば、?演算子で異なるエラー型を統一的に扱える
fn read_and_parse(path: &str) -> Result<i32, Box<dyn Error>> {
    let content = fs::read_to_string(path)?;  // io::Error -> Box<dyn Error>
    let number = content.trim().parse::<i32>()?;  // ParseIntError -> Box<dyn Error>
    Ok(number)
}

fn main() {
    match read_and_parse("number.txt") {
        Ok(n) => println!("数値: {}", n),
        Err(e) => println!("エラー: {}", e),
    }
}
```
