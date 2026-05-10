# 関数（Functions）

## 学習内容
- 関数の定義と呼び出し
- 引数と戻り値
- クロージャ（無名関数）
- 高階関数（関数を引数や戻り値にする）
- 発散する関数（diverging functions）

## 参考ページ
https://doc.rust-jp.rs/rust-by-example-ja/fn.html

## 内容

### 関数の定義

Rustでは `fn` キーワードを使って関数を定義します。関数の引数には型注釈が必須です。

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let result = add(3, 4);
    println!("3 + 4 = {}", result); // 3 + 4 = 7
}
```

- `fn` キーワードで関数を宣言
- 引数には型注釈が **必須**（`a: i32`）
- `-> i32` で戻り値の型を指定
- 最後の式が戻り値になる（セミコロンなし）

### 戻り値

Rustの関数は最後の式を戻り値として返します。`return` キーワードも使えますが、通常は最後の式をそのまま返します。

```rust
// 最後の式が戻り値
fn square(x: i32) -> i32 {
    x * x // セミコロンなし = 戻り値
}

// return キーワードを使った早期リターン
fn absolute(x: i32) -> i32 {
    if x < 0 {
        return -x; // 早期リターン
    }
    x // 最後の式
}

// タプルで複数の値を返す
fn swap(a: i32, b: i32) -> (i32, i32) {
    (b, a)
}
```

- セミコロンなしの式が戻り値
- `return` は早期リターンに使う
- タプルを使って複数値を返せる

### クロージャ

クロージャは無名関数で、周囲の環境の変数をキャプチャできます。

```rust
fn main() {
    // 基本的なクロージャ
    let add = |a: i32, b: i32| a + b;
    println!("add: {}", add(3, 4));

    // 型推論が働く（2回目の呼び出しで型が決まる）
    let multiply = |a, b| a * b;
    println!("multiply: {}", multiply(3, 4));

    // 環境の変数をキャプチャ
    let x = 10;
    let add_x = |a| a + x; // x をキャプチャ
    println!("add_x: {}", add_x(5)); // 15

    // 複数行のクロージャ
    let compute = |a: i32| {
        let doubled = a * 2;
        let squared = doubled * doubled;
        squared
    };
    println!("compute: {}", compute(3)); // 36
}
```

- `|引数| 式` でクロージャを定義
- 周囲の変数をキャプチャできる（これが「クロージャ」の名前の由来）
- 型注釈は省略可能（コンパイラが推論）
- 複数行の場合は `{}` で囲む

#### クロージャのキャプチャ方式

クロージャは変数を3つの方法でキャプチャします：

```rust
fn main() {
    let mut color = String::from("red");

    // 1. 参照でキャプチャ（&T）: 読み取りのみ
    let print_color = || println!("color: {}", color);
    print_color();

    // 2. 可変参照でキャプチャ（&mut T）: 変更可能
    let mut change_color = || {
        color.push_str("-blue");
    };
    change_color();
    println!("changed: {}", color);

    // 3. ムーブでキャプチャ（T）: 所有権を取得
    let consume_color = || {
        let _c = color; // 所有権をムーブ
        println!("consumed");
    };
    // この時点では color はまだ使える（遅延評価のため実際のムーブは呼び出し時）
}
```

- `Fn` トレイト：参照でキャプチャ（複数回呼び出し可能）
- `FnMut` トレイト：可変参照でキャプチャ（複数回呼び出し可能）
- `FnOnce` トレイト：ムーブでキャプチャ（1回だけ呼び出し可能）

#### クロージャを引数に取る関数

```rust
fn apply<F>(f: F, x: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    f(x)
}

fn main() {
    let double = |x| x * 2;
    println!("apply: {}", apply(double, 5)); // 10
}
```

### 高階関数

高階関数は、関数を引数に取るか、関数を返す関数です。Rustのイテレータと組み合わせてよく使われます。

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // map: 各要素に関数を適用
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("doubled: {:?}", doubled); // [2, 4, 6, 8, 10]

    // filter: 条件で絞り込み
    let evens: Vec<&i32> = numbers.iter().filter(|x| *x % 2 == 0).collect();
    println!("evens: {:?}", evens); // [2, 4]

    // fold: 畳み込み（reduce）
    let sum: i32 = numbers.iter().fold(0, |acc, x| acc + x);
    println!("sum: {}", sum); // 15

    // 関数ポインタを渡すことも可能
    fn is_odd(n: &i32) -> bool {
        n % 2 != 0
    }
    let odds: Vec<&i32> = numbers.iter().filter(is_odd).collect();
    println!("odds: {:?}", odds); // [1, 3, 5]
}
```

- `map`：各要素を変換
- `filter`：条件で絞り込み
- `fold`：値を一つにまとめる
- 関数ポインタ（`fn` 型）も高階関数の引数に渡せる

### 発散する関数（Diverging Functions）

`!` 型（never型）を返す関数は「決して戻らない」ことを表します。パニックや無限ループが該当します。

```rust
// パニックで終了（戻らない）
fn die(message: &str) -> ! {
    panic!("Fatal error: {}", message);
}

// 無限ループ（戻らない）
fn forever() -> ! {
    loop {
        // 永遠に続く
    }
}

fn main() {
    let result = match Some(42) {
        Some(value) => value,
        None => die("no value"), // ! 型は任意の型に強制変換可能
    };
    println!("result: {}", result);

    // panic! マクロも ! 型を返す
    let _x: i32 = if true { 42 } else { panic!("never") };
}
```

- `-> !` は「決して戻らない」ことを示す
- `panic!()`, `todo!()`, `unimplemented!()` などが該当
- `!` 型は任意の型に強制変換可能（coerce）
- `match` 式などで、あり得ないブランチに使える
