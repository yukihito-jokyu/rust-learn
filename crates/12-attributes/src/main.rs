// TODO: #[allow(dead_code)] を使って未使用関数の警告を抑制してください
// ヒント: #[allow(dead_code)] fn unused() { ... }

// TODO: #[derive] を使って構造体にトレイトを自動実装してください
// - Debug, Clone, PartialEq, Default を derive する Point 構造体
// ヒント: #[derive(Debug, Clone, PartialEq, Default)]

// TODO: マクロを使ってみてください
// - println!, vec!, assert!, assert_eq!, assert_ne!
// ヒント: let v = vec![1, 2, 3];

// TODO: cfg アトリビュートで条件コンパイルを試してください
// - target_os ごとに異なる関数を定義
// - cfg! マクロでコンパイル時条件を確認
// ヒント: #[cfg(target_os = "macos")] fn macos_only() { ... }
// ヒント: if cfg!(target_os = "macos") { ... }

#[allow(dead_code)]
fn unused_function() {
    println!("This is unused.");
}

#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    // ここにコードを書いてください
    let p1 = Point { x: 1, y: 2 };
    let p2 = p1.clone();

    // Debug トレイトの自動実装により {:?} で表示可能
    println!("{:?}", p1); // Point { x: 1, y: 2 }

    // PartialEq トレイトの自動実装
    println!("equal: {}", p1 == p2); // equal: true

    // Clone トレイトの自動実装
    let p3 = p1.clone();
    println!("cloned: {:?}", p3);

    // vec! マクロ
    let v = vec![1, 2, 3, 4, 5];
    println!("vec: {:?}", v);
}
