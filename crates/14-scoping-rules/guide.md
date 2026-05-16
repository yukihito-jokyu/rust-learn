# スコープ規則（Scoping Rules）

## 学習内容

- RAII（Resource Acquisition Is Initialization）
- 所有権とムーブ
- 借用（`&T`, `&mut T`）
- ライフタイム（`'a`）

## 参考ページ

https://doc.rust-jp.rs/rust-by-example-ja/scope.html

## 内容

### RAII（Resource Acquisition Is Initialization）

Rustでは、リソース（メモリ、ファイルハンドル、ロックなど）は変数がスコープに入るときに確保され、スコープを出るときに自動的に解放されます。これがRAIIパターンです。

```rust
fn main() {
    {
        let v = vec![1, 2, 3]; // リソース確保
        println!("v: {:?}", v);
    } // v がスコープを抜け、メモリが自動解放される（Drop トレイトが呼ばれる）

    // v はもう使えない
}
```

#### カスタム Drop 実装

```rust
struct Resource {
    name: String,
}

impl Drop for Resource {
    fn drop(&mut self) {
        println!("Dropping resource: {}", self.name);
    }
}

fn main() {
    let r1 = Resource { name: String::from("A") };
    {
        let r2 = Resource { name: String::from("B") };
        println!("Inside inner scope");
    } // r2 がドロップされる
    println!("Outside inner scope");
} // r1 がドロップされる

// 出力:
// Inside inner scope
// Dropping resource: B
// Outside inner scope
// Dropping resource: A
```

- 変数がスコープを抜けると自動的に `Drop::drop()` が呼ばれる
- ドロップ順序は宣言の **逆順**
- ガベージコレクタが不要な仕組み

### 所有権とムーブ（Ownership and Move）

Rustではすべての値に「所有者」がひとつだけいます。代入や関数呼び出しで値がムーブ（移動）されます。

#### 基本的なムーブ

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // s1 の所有権が s2 にムーブされる

    // println!("{}", s1); // エラー！ s1 はもう使えない
    println!("{}", s2);   // OK: s2 が所有権を持っている
}
```

#### 関数でのムーブ

```rust
fn take_ownership(s: String) {
    println!("taken: {}", s);
} // s はここでドロップされる

fn main() {
    let s = String::from("hello");
    take_ownership(s);
    // println!("{}", s); // エラー！ s の所有権は関数に移動した
}
```

#### 戻り値での所有権移動

```rust
fn give_ownership() -> String {
    String::from("yours now")
}

fn take_and_give(s: String) -> String {
    s // 所有権を受け取り、そのまま返す
}

fn main() {
    let s1 = give_ownership();       // 関数から所有権を受け取る
    let s2 = String::from("hello");
    let s3 = take_and_give(s2);      // s2 を渡し、新しい所有権を受け取る
    // s2 は使えない（ムーブ済み）

    println!("s1: {}", s1);
    println!("s3: {}", s3);
}
```

#### コピー型（ムーブされない型）

整数や浮動小数点数などの単純な型は、代入時にコピーされます。

```rust
fn main() {
    let x = 42;
    let y = x; // コピーされる（ムーブではない）
    println!("x: {}, y: {}", x, y); // 両方使える

    // Copy トレイトを実装する型:
    // - すべての整数型（i32, u8, ...）
    // - 浮動小数点数型（f32, f64）
    // - bool, char
    // - タプル（要素がすべて Copy の場合）
}
```

### 借用（Borrowing）

所有権を移動させずに値を参照する仕組みが「借用」です。

#### 不変借用（`&T`）

```rust
fn calculate_length(s: &String) -> usize {
    s.len()
} // s はスコープを抜けるが、所有権がないので何も起きない

fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // s1 を借用（参照を渡す）
    println!("'{}' has length {}", s1, len); // s1 はまだ使える
}
```

- `&T` で不変参照を作成
- 参照元の値は読めるが変更できない
- 複数の不変借用を同時に持てる

#### 可変借用（`&mut T`）

```rust
fn append_world(s: &mut String) {
    s.push_str(", world");
}

fn main() {
    let mut s = String::from("hello");
    append_world(&mut s); // 可変借用
    println!("{}", s); // "hello, world"
}
```

- `&mut T` で可変参照を作成
- 参照元の値を読み書きできる
- **同時には1つだけ** 可変借用を持てる

#### 借用のルール

```rust
fn main() {
    let mut s = String::from("hello");

    // ルール1: 複数の不変借用は OK
    let r1 = &s;
    let r2 = &s;
    println!("r1: {}, r2: {}", r1, r2); // OK

    // ルール2: 可変借用は1つだけ
    let r3 = &mut s;
    r3.push_str(" world");
    println!("r3: {}", r3); // OK

    // ルール3: 不変借用と可変借用は同時に存在できない
    // let r4 = &s;     // エラー！ r3 が可変借用中
    // let r5 = &mut s; // エラー！ r3 が可変借用中
}
```

借用の3つのルール：

1. 任意の数の不変参照（`&T`）を持てる
2. 正確に1つの可変参照（`&mut T`）だけを持てる
3. 不変参照と可変参照は同時に存在できない

#### Non-Lexical Lifetimes（NLL）

Rust 2018以降では、参照が最後に使われた時点で借用が終了します。

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("r1: {}, r2: {}", r1, r2);
    // r1, r2 はもう使われないので借用はここで終了

    let r3 = &mut s; // OK！ r1, r2 の借用は終了している
    r3.push_str(" world");
    println!("r3: {}", r3);
}
```

### ライフタイム（Lifetimes）

ライフタイムは参照が有効な期間をコンパイラに伝える仕組みです。

#### ライフタイム注釈の基本

```rust
// 'a はライフタイムパラメータ
// 戻り値の参照は x と y の短い方のライフタイムを持つ
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let s1 = String::from("long string");
    let result;

    let s2 = String::from("xyz");
    result = longest(s1.as_str(), s2.as_str());
    println!("longest: {}", result); // "long string"
}
```

- `'a` はライフタイムパラメータ（アポストロフィで始まる）
- コンパイラに「どの参照がどのくらい生きているか」を伝える
- 参照を返す関数ではライフタイム注釈が必要なことがある

#### ライフタイムがエラーを防ぐ例

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let s1 = String::from("long string");
    let result;

    {
        let s2 = String::from("xyz");
        // result = longest(s1.as_str(), s2.as_str());
        // println!("longest: {}", result);
        // s2 がスコープを抜けた後に result を使うとエラー
    }
    // println!("result: {}", result); // エラー！ s2 はもう存在しない
}
```

#### 構造体のライフタイム

構造体が参照を保持する場合、ライフタイム注釈が必要です。

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();

    let excerpt = ImportantExcerpt {
        part: first_sentence,
    };
    println!("excerpt: {}", excerpt.part);
}
```

#### ライフタイムの省略規則（Elision Rules）

すべての関数にライフタイム注釈を書く必要はありません。コンパイラが推論できる場合があります。

ルール：

1. 各参照パラメータは独自のライフタイムを取得する
2. 参照パラメータが1つだけの場合、戻り値のライフタイムはそれと同じ
3. 複数の参照パラメータがあり、そのうちの1つが `&self` または `&mut self` の場合、戻り値のライフタイムは `self` と同じ

```rust
// 省略形（コンパイラが推論）
fn first_word(s: &str) -> &str {
    // コンパイラが以下のように推論:
    // fn first_word<'a>(s: &'a str) -> &'a str
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
```

#### `'static` ライフタイム

`'static` はプログラム全体の生存期間を示します。

```rust
// 文字列リテラルは 'static ライフタイムを持つ
let s: &'static str = "I have a static lifetime.";

// プログラム全体で有効
```
