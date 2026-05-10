// 20 - テスト（Testing）
// このファイルはTODOコメント付きの雛形です。
// guide.mdを参考にして、各TODOの部分を自分で実装してみてください。

// TODO: テスト対象の関数を定義してください
// 例: add, subtract, is_even などのシンプルな関数

fn main() {
    println!("=== テストの実行 ===");
    println!("このファイルのテストを実行するには:");
    println!("  cargo test");
    println!();

    // テスト関数はmainから呼び出す必要はありません
    // cargo test が自動的に #[test] 属性の関数を見つけて実行します

    // TODO: テスト対象の関数をここでデモ実行してください
    // 例:
    // println!("add(1, 2) = {}", add(1, 2));
}

// TODO: #[cfg(test)] モジュールを作成してください
// その中に #[test] 関数を定義してください
#[cfg(test)]
mod tests {
    // TODO: use super::*; を追加してください

    // TODO: assert_eq! を使ったテストを書いてください
    // #[test]
    // fn test_add() {
    //     assert_eq!(add(1, 2), 3);
    // }

    // TODO: assert! を使ったテストを書いてください
    // #[test]
    // fn test_is_even() {
    //     assert!(is_even(4));
    //     assert!(!is_even(3));
    // }

    // TODO: assert_ne! を使ったテストを書いてください

    // TODO: #[should_panic] を使ったテストを書いてください
    // #[test]
    // #[should_panic]
    // fn test_panic() {
    //     panic!("意図的なパニック");
    // }

    // TODO: Resultを返すテストを書いてください
    // #[test]
    // fn test_with_result() -> Result<(), Box<dyn std::error::Error>> {
    //     Ok(())
    // }
}

// TODO: ドックテスト用のドキュメントコメントを書いてください
// /// 2つの数値を加算する。
// ///
// /// # Examples
// ///
// /// ```
// /// assert_eq!(add(1, 2), 3);
// /// ```
// pub fn add(a: i32, b: i32) -> i32 {
//     a + b
// }
