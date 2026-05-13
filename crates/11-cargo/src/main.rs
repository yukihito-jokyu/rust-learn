// TODO: Cargo.toml を確認・設定してください
// - [package] の name, version, edition を確認
// - [dependencies] に外部クレート（例: rand = "0.8"）を追加

// TODO: 追加した外部クレートをインポートして使用してください
// ヒント: use rand::Rng;

// TODO: 単体テストを書いてみてください
// ヒント: #[cfg(test)] mod tests { use super::*; #[test] fn test_...() { ... } }

// TODO: ドキュメンテーションコメントを書いてみてください
// ヒント: /// ドキュメント
// ヒント: /// # Examples
// ヒント: /// ``` ... ```

/// 2つの数値を足し算します。
///
/// # Examples
///
/// ```
/// use my_project::add;
/// assert_eq!(add(2, 3), 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// # 見出し1
///
/// ## 見出し2
///
/// - 箇条書き
/// - **太字**
/// - *斜体*
/// - `インラインコード`
///
/// | 左 | 右 |
/// |---|---|
/// | a | b |
///
/// [リンク](https://example.com)
pub fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 { None } else { Some(a / b) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
    }

    #[test]
    fn test_add_not_equal() {
        assert_ne!(add(2, 3), 6);
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide(10, 2), Some(5));
        assert_eq!(divide(10, 0), None);
    }

    #[test]
    #[should_panic(expected = "overflow")]
    fn test_overflow() {
        panic!("overflow");
    }

    #[test]
    #[ignore] // cargo test -- --ignored で実行
    fn expensive_test() {
        // 時間のかかるテスト
    }
}

fn main() {
    // ここにコードを書いてください
}
