# 変数バインディング（Variable Bindings）

## 学習内容
- 可変性（`mut`）
- スコープとシャドウイング
- 宣言
- 凍結（freezing）

## 参考ページ
https://doc.rust-jp.rs/rust-by-example-ja/variable_bindings.html

## 内容

### 可変性（Mutability）

Rustの変数はデフォルトで不変（immutable）です。値を変更するには `mut` キーワードを使います。

```rust
fn main() {
    // 不変変数（デフォルト）
    let x = 5;
    // x = 6; // エラー！不変変数に再代入はできない

    // 可変変数
    let mut y = 5;
    println!("y = {}", y);
    y = 6; // OK！
    println!("y = {}", y);
}
```

- `let` --- 不変変数の宣言（デフォルト）
- `let mut` --- 可変変数の宣言
- 不変性は安全な並行プログラミングを助ける

### スコープとシャドウイング

#### スコープ

変数は宣言されたブロック（`{}`）内でのみ有効です。

```rust
fn main() {
    let outer = 1;

    { // 内側のブロック開始
        let inner = 2;
        println!("outer = {}, inner = {}", outer, inner); // 両方アクセス可能
    } // 内側のブロック終了

    // println!("{}", inner); // エラー！innerはスコープ外
    println!("outer = {}", outer); // outerはアクセス可能
}
```

#### シャドウイング（Shadowing）

同じ名前の変数を再宣言できます。前の変数は「隠れる」状態になります。

```rust
fn main() {
    let x = 5;
    println!("x = {}", x); // 5

    // シャドウイング：同じ名前で新しい変数を宣言
    let x = x + 1;
    println!("x = {}", x); // 6

    // 型も変えられる！
    let x = "文字列"; // i32 から &str に変更
    println!("x = {}", x); // 文字列

    // シャドウイングと mut の違い：
    // - シャドウイングは新しい変数を作る（型も変えられる）
    // - mut は同じ変数の値を変える（型は変えられない）
}
```

シャドウイングのユースケース：

```rust
fn main() {
    // 文字列を数値に変換して同じ名前で使う
    let input = "42";
    let input: i32 = input.parse().unwrap();
    println!("数値: {}", input); // 42
}
```

### 宣言

変数の宣言にはいくつかの方法があります。

```rust
fn main() {
    // 基本的な宣言
    let a = 1;

    // 型注釈付きの宣言
    let b: i32 = 2;

    // 初期値なしの宣言（後で初期化）
    let c;
    c = 3; // 最初の代入が初期化
    // c = 4; // エラー！不変変数に再代入不可

    // 可変変数の宣言
    let mut d = 4;
    d = 5; // OK

    // 宣言と同時にパターンマッチング
    let (e, f) = (5, 6);
    println!("e = {}, f = {}", e, f);
}
```

### 凍結（Freezing）

可変変数を不変変数でシャドウイングすると、そのスコープ内では「凍結」されます。

```rust
fn main() {
    let mut x = 10;

    {
        // 可変変数xを不変変数でシャドウイング
        let x = x; // このスコープ内ではxは不変

        println!("凍結された x = {}", x); // 10

        // x = 20; // エラー！凍結されている
    }

    // 外側のスコープでは x は再び可変
    x = 20;
    println!("可変な x = {}", x); // 20
}
```

### まとめ

| 機能 | キーワード | 説明 |
|------|-----------|------|
| 不変変数 | `let` | 再代入不可（デフォルト） |
| 可変変数 | `let mut` | 再代入可能 |
| シャドウイング | `let`同名 | 新しい変数で隠す（型変更可能） |
| スコープ | `{}` | ブロック内でのみ有効 |
| 凍結 | `let x = x` | 可変変数を不変でシャドウ |

#### `mut` と シャドウイングの使い分け

```rust
fn main() {
    // mut: 同じ型で値を繰り返し変更したい場合
    let mut count = 0;
    for _ in 0..5 {
        count += 1;
    }
    println!("count = {}", count);

    // シャドウイング: 型を変えたい場合や、変換後の値を同じ名前で使いたい場合
    let value = "123";
    let value: i32 = value.parse().unwrap();
    println!("value = {}", value);
}
```
