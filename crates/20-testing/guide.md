# 20 - テスト（Testing）

## 学習内容
- ユニットテスト
- `#[test]`属性
- `assert!`、`assert_eq!`、`assert_ne!`マクロ
- テスト用モジュール（`#[cfg(test)]`）
- ドックテスト
- 統合テスト
- テストの並列実行
- テストの無視とフィルタリング

## 参考ページ
https://doc.rust-jp.rs/rust-by-example-ja/testing.html

## 内容

### ユニットテストの基本

Rustでは`#[test]`属性を関数に付けることでテスト関数を定義できます。

```rust
// テスト対象の関数
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
        assert_eq!(add(-1, 1), 0);
        assert_eq!(add(0, 0), 0);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(subtract(5, 3), 2);
        assert_ne!(subtract(5, 3), 0);
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide(10.0, 2.0), Some(5.0));
        assert_eq!(divide(10.0, 0.0), None);
    }

    #[test]
    fn test_assert() {
        let value = true;
        assert!(value, "値はtrueであるべきです");
    }
}
```

### assert!マクロ

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_assert_basic() {
        // assert! - ブール値がtrueであることを確認
        assert!(true);
        assert!(1 + 1 == 2);

        // カスタムメッセージ付き
        assert!(true, "このテストは成功するはずです");
    }

    #[test]
    fn test_assert_eq() {
        // assert_eq! - 2つの値が等しいことを確認
        assert_eq!(1 + 1, 2);
        assert_eq!("hello".to_uppercase(), "HELLO");

        // カスタムメッセージ付き
        assert_eq!(2 * 3, 6, "2×3は6であるべきです");
    }

    #[test]
    fn test_assert_ne() {
        // assert_ne! - 2つの値が等しくないことを確認
        assert_ne!(1, 2);
        assert_ne!("abc", "def");
    }

    #[test]
    #[should_panic]
    fn test_panic() {
        // should_panic - パニックが発生することを確認
        panic!("意図的なパニック");
    }

    #[test]
    #[should_panic(expected = "オーバーフロー")]
    fn test_panic_with_message() {
        // expected - パニックメッセージの部分一致を確認
        panic!("オーバーフローが発生しました！");
    }
}
```

### Resultを使ったテスト

テスト関数が`Result<T, E>`を返すようにすることで、`?`演算子を使えます。

```rust
fn parse_number(s: &str) -> Result<i32, std::num::ParseIntError> {
    s.parse::<i32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_number() -> Result<(), std::num::ParseIntError> {
        // ?演算子が使える
        assert_eq!(parse_number("42")?, 42);
        assert_eq!(parse_number("-5")?, -5);
        Ok(())
    }

    // Resultを返すテストでエラーを返すとテスト失敗
    #[test]
    fn test_parse_invalid() -> Result<(), Box<dyn std::error::Error>> {
        // parse_number("abc")はErrを返すので、?でテストが失敗する
        // parse_number("abc")?;
        // 代わりに明示的に確認する
        assert!(parse_number("abc").is_err());
        Ok(())
    }
}
```

### テスト用モジュール（#[cfg(test)]）

`#[cfg(test)]`を付けたモジュールは、`cargo test`のときだけコンパイルされます。

```rust
// プロダクションコード
pub struct Calculator {
    history: Vec<String>,
}

impl Calculator {
    pub fn new() -> Self {
        Calculator { history: Vec::new() }
    }

    pub fn add(&mut self, a: i32, b: i32) -> i32 {
        let result = a + b;
        self.history.push(format!("{} + {} = {}", a, b, result));
        result
    }

    pub fn multiply(&mut self, a: i32, b: i32) -> i32 {
        let result = a * b;
        self.history.push(format!("{} * {} = {}", a, b, result));
        result
    }

    pub fn history(&self) -> &[String] {
        &self.history
    }
}

// テストモジュール（cargo test時のみコンパイル）
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculator_add() {
        let mut calc = Calculator::new();
        assert_eq!(calc.add(2, 3), 5);
        assert_eq!(calc.history().len(), 1);
    }

    #[test]
    fn test_calculator_multiply() {
        let mut calc = Calculator::new();
        assert_eq!(calc.multiply(4, 5), 20);
        assert_eq!(calc.history()[0], "4 * 5 = 20");
    }

    // setupヘルパー関数
    fn create_calc_with_history() -> Calculator {
        let mut calc = Calculator::new();
        calc.add(1, 2);
        calc.multiply(3, 4);
        calc
    }

    #[test]
    fn test_with_setup() {
        let calc = create_calc_with_history();
        assert_eq!(calc.history().len(), 2);
    }
}
```

### ドックテスト（Doc Tests）

ドキュメンテーションコメント内のコード例はテストとして実行されます。

```rust
/// 2つの数値を加算する。
///
/// # Examples
///
/// ```
/// use s20_testing::add;
/// assert_eq!(add(1, 2), 3);
/// assert_eq!(add(-1, 1), 0);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// 指定された値が正の数かどうかを判定する。
///
/// # Examples
///
/// ```
/// use s20_testing::is_positive;
/// assert!(is_positive(1));
/// assert!(!is_positive(-1));
/// assert!(!is_positive(0));
/// ```
pub fn is_positive(n: i32) -> bool {
    n > 0
}
```

### テストの無視とフィルタリング

```rust
#[test]
fn test_normal() {
    assert_eq!(1 + 1, 2);
}

#[test]
#[ignore]
fn test_expensive() {
    // 時間のかかるテスト
    // cargo test -- --ignored で実行可能
    println!("重いテストを実行中...");
}

#[test]
#[ignore = "環境変数CIが設定されているときのみ実行"]
fn test_ci_only() {
    // CI環境でのみ実行したいテスト
}
```

### テストの実行コマンド

```bash
# 全テストを実行
cargo test

# テスト名でフィルタリング
cargo test test_add

# 標準出力を表示
cargo test -- --nocapture

# 無視されたテストも実行
cargo test -- --ignored

# ドックテストのみ実行
cargo test --doc

# ユニットテストのみ実行
cargo test --lib

# 統合テストのみ実行
cargo test --test integration_test

# スレッド数を指定（並列実行の制御）
cargo test -- --test-threads=1
```
