# 制御フロー（Flow of Control）

## 学習内容
- if / else
- loop
- while
- for とイテレータ
- match とパターンマッチング
- if let
- let else

## 参考ページ
https://doc.rust-jp.rs/rust-by-example-ja/flow_control.html

## 内容

### if / else

条件分岐の基本構文です。

```rust
fn main() {
    let n = 5;

    // 基本的な if / else
    if n > 0 {
        println!("正の数です");
    } else if n < 0 {
        println!("負の数です");
    } else {
        println!("ゼロです");
    }

    // if は式なので値を返せる
    let result = if n % 2 == 0 { "偶数" } else { "奇数" };
    println!("{}は{}", n, result);
}
```

### loop

`loop` は無限ループを作ります。`break`で脱出、`continue`で次のイテレーションへ進みます。

```rust
fn main() {
    let mut count = 0;

    // 基本的なloop
    loop {
        count += 1;
        if count == 5 {
            println!("countが5になったので終了");
            break;
        }
    }

    // loop は値を返せる（breakで値を指定）
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // 20を返す
        }
    };
    println!("result = {}", result); // 20

    // ネストしたloopとラベル
    'outer: for x in 0..5 {
        for y in 0..5 {
            if x == 2 && y == 2 {
                println!("外側のループを終了 ({}, {})", x, y);
                break 'outer; // 外側のループをbreak
            }
        }
    }

    // continue で次のイテレーションへ
    for i in 0..10 {
        if i % 2 == 0 {
            continue; // 偶数はスキップ
        }
        println!("奇数: {}", i);
    }
}
```

### while

`while` は条件が真の間ループします。

```rust
fn main() {
    let mut n = 1;

    // 基本的なwhile
    while n < 100 {
        n *= 2;
    }
    println!("n = {}", n); // 128

    // whileでコレクションを走査（非推奨、forを使うべき）
    let arr = [1, 2, 3, 4, 5];
    let mut index = 0;
    while index < arr.len() {
        println!("arr[{}] = {}", index, arr[index]);
        index += 1;
    }
}
```

### for とイテレータ

`for` はイテレータを使ってコレクションを走査する最も一般的な方法です。

```rust
fn main() {
    // 配列のイテレート
    let arr = [1, 2, 3, 4, 5];
    for element in arr.iter() {
        println!("要素: {}", element);
    }

    // Rangeを使ったループ
    for i in 0..5 {
        println!("i = {}", i); // 0, 1, 2, 3, 4
    }

    // 逆向きのRange
    for i in (0..5).rev() {
        println!("i = {}", i); // 4, 3, 2, 1, 0
    }

    // enumerateでインデックス付き
    for (index, value) in arr.iter().enumerate() {
        println!("arr[{}] = {}", index, value);
    }

    // _で値を無視（N回繰り返す）
    for _ in 0..3 {
        println!("3回繰り返す");
    }

    // 可変参照で値を変更
    let mut nums = [1, 2, 3];
    for n in nums.iter_mut() {
        *n *= 2;
    }
    println!("{:?}", nums); // [2, 4, 6]
}
```

#### イテレータメソッド

```rust
fn main() {
    let arr = [1, 2, 3, 4, 5];

    // for と同じことが iter() のメソッドでできる
    let sum: i32 = arr.iter().sum();
    println!("合計: {}", sum); // 15

    let doubled: Vec<i32> = arr.iter().map(|x| x * 2).collect();
    println!("2倍: {:?}", doubled); // [2, 4, 6, 8, 10]

    let evens: Vec<&i32> = arr.iter().filter(|x| *x % 2 == 0).collect();
    println!("偶数: {:?}", evens); // [2, 4]
}
```

### match とパターンマッチング

`match` はRustの強力なパターンマッチング機能です。

```rust
fn main() {
    let number = 13;

    // 基本的なmatch
    match number {
        1 => println!("いち"),
        2 | 3 | 5 | 7 | 11 | 13 => println!("素数"),
        13..=19 => println!("10代"),
        _ => println!("その他"), // _ はワイルドカード
    }

    // タプルのマッチング
    let pair = (2, -2);
    match pair {
        (0, y) => println!("xは0、yは{}", y),
        (x, 0) => println!("xは{}、yは0", x),
        (x, y) if x == -y => println!("{}と{}は反転", x, y), // ガード
        _ => println!("マッチなし"),
    }

    // Enumのマッチング
    enum Color {
        Red,
        Green,
        Blue,
        RGB(u8, u8, u8),
    }
    let color = Color::RGB(122, 17, 40);
    match color {
        Color::Red => println!("赤"),
        Color::Green => println!("緑"),
        Color::Blue => println!("青"),
        Color::RGB(r, g, b) => println!("RGB({}, {}, {})", r, g, b),
    }

    // 参照のマッチング（refとref mut）
    let mut value = 10;
    match value {
        ref r => println!("参照: {}", r), // r は &i32
    }
    match value {
        ref mut m => {
            *m += 1;
            println!("可変参照: {}", m);
        }
    }

    // 構造体のマッチング
    struct Point { x: i32, y: i32 }
    let point = Point { x: 0, y: 7 };
    match point {
        Point { x: 0, y } => println!("y軸上: y = {}", y),
        Point { x, y: 0 } => println!("x軸上: x = {}", x),
        Point { x, y } => println!("({}, {})", x, y),
    }
}
```

#### マッチングの網羅性

```rust
fn main() {
    let number = 4;

    // matchは全パターンを網羅する必要がある
    match number {
        1 => println!("1"),
        2..=10 => println!("2〜10"),
        _ => println!("その他"), // これがないとコンパイルエラー
    }
}
```

### if let

`if let` は `match` の簡略版で、一つのパターンだけを処理したい場合に使います。

```rust
fn main() {
    let option = Some(7);

    // matchで書く場合
    match option {
        Some(i) => println!("値: {}", i),
        _ => {} // その他は無視
    }

    // if letで書く場合（より簡潔）
    if let Some(i) = option {
        println!("値: {}", i);
    }

    // else付き
    let option: Option<i32> = None;
    if let Some(i) = option {
        println!("値: {}", i);
    } else {
        println!("値はありません");
    }
}
```

### let else

`let else` はパターンがマッチしなかった場合に早期リターンやパニックができます（Rust 1.65以降）。

```rust
fn main() {
    let option: Option<i32> = Some(42);

    // let elseパターン
    let Some(value) = option else {
        println!("値がありません");
        return; // 早期リターン
    };

    println!("値: {}", value); // valueはここで i32 として使える
}
```

```rust
use std::env;

fn main() {
    // let elseの実用的な例
    let args: Vec<String> = env::args().collect();

    let Some(first_arg) = args.get(1) else {
        println!("引数を指定してください");
        return;
    };

    println!("最初の引数: {}", first_arg);
}
```

### 制御フロー構文のまとめ

| 構文 | 用途 | 値を返すか |
|------|------|-----------|
| `if` / `else` | 条件分岐 | はい |
| `loop` | 無限ループ | はい（breakで） |
| `while` | 条件付きループ | いいえ（常に()） |
| `for` | イテレーション | いいえ（常に()） |
| `match` | パターンマッチング | はい |
| `if let` | 単一パターンマッチ | いいえ |
| `let else` | マッチしない場合の早期リターン | いいえ |
