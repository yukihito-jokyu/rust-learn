# 式（Expressions）

## 学習内容
- ブロックと式
- if式

## 参考ページ
https://doc.rust-jp.rs/rust-by-example-ja/expression.html

## 内容

### Rustにおける文と式

Rustは「式指向」の言語です。ほとんどの構文が値を返す「式（expression）」として扱われます。

- **文（statement）**: 値を返さない。`;` で終わる。
- **式（expression）**: 値を返す。`;` をつけないと値になる。

```rust
fn main() {
    // 文（値を返さない）
    let x = 5; // let は文

    // 式（値を返す）
    let y = 5 + 3; // 5 + 3 は式（値8を返す）
}
```

### ブロックと式

ブロック `{}` も式です。最後の式（`;` なし）がブロック全体の値になります。

```rust
fn main() {
    // ブロック式の基本
    let x = {
        let a = 1;
        let b = 2;
        a + b  // セミコロンなし → このブロックの値
    };
    println!("x = {}", x); // 3

    // セミコロンをつけると文になり、ユニット () になる
    let y = {
        let a = 1;
        let b = 2;
        a + b;  // セミコロンあり → ユニット ()
    };
    println!("y = {:?}", y); // ()
}
```

#### ブロック式の活用例

```rust
fn main() {
    // 初期化時に複雑な計算を行う
    let result = {
        let base = 100;
        let tax = 10;
        let total = base + tax;
        format!("合計: {}円（税込み）", total)
    };
    println!("{}", result);

    // 中間変数をスコープに閉じ込める
    let area = {
        let width = 10;
        let height = 20;
        width * height
    };
    println!("面積: {}", area);
    // width, height はここでは使えない
}
```

### if式

Rustの `if` は文ではなく式です。値を返すことができます。

```rust
fn main() {
    // 基本的なif式
    let number = 5;
    if number > 3 {
        println!("{} は3より大きい", number);
    }

    // if-else式
    let x = 10;
    if x % 2 == 0 {
        println!("{} は偶数", x);
    } else {
        println!("{} は奇数", x);
    }

    // if式を変数に代入
    let condition = true;
    let result = if condition {
        "条件は真です"
    } else {
        "条件は偽です"
    };
    println!("結果: {}", result);

    // else ifチェーン
    let score = 85;
    let grade = if score >= 90 {
        "A"
    } else if score >= 80 {
        "B"
    } else if score >= 70 {
        "C"
    } else if score >= 60 {
        "D"
    } else {
        "F"
    };
    println!("成績: {}", grade);
}
```

#### if式の注意点

```rust
fn main() {
    // 両方のブランチの型が一致している必要がある
    let x = 5;

    // OK: 両方とも &str
    let result = if x > 3 {
        "大きい"
    } else {
        "小さい"
    };

    // NG: 型が異なるとコンパイルエラー
    // let error = if x > 3 {
    //     42
    // } else {
    //     "小さい"  // エラー！i32 と &str は型が違う
    // };

    // if式で値を返さないブランチは () を返す
    if x > 3 {
        println!("大きい");
        // 値を返さない → ()
    }
    // 上のif式全体の値は ()
}
```

### 式として使える構文のまとめ

| 構文 | 値を返すか | 例 |
|------|-----------|-----|
| ブロック `{}` | 最後の式 | `{ 1 + 2 }` → `3` |
| `if` / `else` | 選ばれたブランチ | `if true { 1 } else { 2 }` → `1` |
| `match` | マッチしたアーム | `match x { 1 => "one", _ => "other" }` |
| `loop` | `break`の値 | `loop { break 42; }` → `42` |
| 安全な演算子 | 演算結果 | `a + b`, `a * b` など |
