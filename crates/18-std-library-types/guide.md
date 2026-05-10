# 18 - 標準ライブラリの型（Standard Library Types）

## 学習内容
- `Box<T>` - ヒープ割り当て
- `String` - 文字列操作
- `Option<T>` - 値の有無
- `Result<T, E>` - エラーハンドリング
- `Vec<T>` - 動的配列
- `HashMap<K, V>` / `HashSet<T>` - コレクション
- `Rc<T>` / `Arc<T>` - 参照カウント
- `Cell<T>` / `RefCell<T>` - 内部可変性

## 参考ページ
https://doc.rust-jp.rs/rust-by-example-ja/std.html

## 内容

### Box<T> - ヒープ割り当て

`Box<T>`は値をヒープに配置し、スタックにはそのポインタを保持します。

```rust
fn main() {
    // 基本的なBox
    let boxed_int = Box::new(5);
    println!("Boxの中身: {}", boxed_int); // 5

    // Boxから値を取り出す
    let unboxed = *boxed_int;
    println!("取り出した値: {}", unboxed); // 5

    // 再帰的な型にBoxを使う（コンパイル時にサイズを知る必要があるため）
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    let list = List::Cons(1,
        Box::new(List::Cons(2,
            Box::new(List::Cons(3,
                Box::new(List::Nil)
            ))
        ))
    );
    println!("リスト: {:?}", list);

    // トレイトオブジェクトとしてのBox
    trait Animal {
        fn speak(&self);
    }

    struct Dog;
    struct Cat;

    impl Animal for Dog {
        fn speak(&self) { println!("ワン！"); }
    }

    impl Animal for Cat {
        fn speak(&self) { println!("ニャー！"); }
    }

    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog),
        Box::new(Cat),
    ];

    for animal in animals {
        animal.speak();
    }
}
```

### String - 文字列操作

```rust
fn main() {
    // Stringの作成
    let mut s = String::new();
    s.push_str("こんにちは");
    s.push('！');
    println!("{}", s); // こんにちは！

    // from で作成
    let s2 = String::from("Rust");
    println!("{}", s2);

    // リテラルからの変換
    let s3 = "世界".to_string();

    // 文字列の結合
    let greeting = s2 + " " + &s3;
    println!("{}", greeting); // Rust 世界

    // format!マクロ
    let formatted = format!("{} {}", "Hello", "Rust");
    println!("{}", formatted);

    // 文字列の長さ（バイト数）
    println!("バイト数: {}", "Rust".len());        // 4
    println!("バイト数: {}", "こんにちは".len());   // 15 (UTF-8)

    // 文字数
    println!("文字数: {}", "こんにちは".chars().count()); // 5

    // 文字のイテレーション
    for c in "Rust🦀".chars() {
        println!("文字: {}", c);
    }

    // スライス（バイト単位なので注意）
    let hello = "こんにちは";
    let first_char = &hello[0..3]; // 最初の1文字（UTF-8で3バイト）
    println!("最初の文字: {}", first_char);

    // 文字列の検索
    let sentence = "Rustは素晴らしい言語です";
    println!("含む?: {}", sentence.contains("素晴らしい")); // true
    println!("先頭: {}", sentence.starts_with("Rust"));    // true

    // replace, trim, split
    println!("{}", "  hello  ".trim());           // "hello"
    println!("{}", "a,b,c".split(',').collect::<Vec<_>>().join(" - "));
}
```

### Vec<T> - 動的配列

```rust
fn main() {
    // Vecの作成
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    // マクロで作成
    let vec2 = vec![10, 20, 30, 40, 50];

    // 要素へのアクセス
    println!("最初: {}", vec[0]);          // 1
    println!("get: {:?}", vec.get(1));     // Some(2)

    // イテレーション
    for val in &vec2 {
        println!("値: {}", val);
    }

    // インデックス付きイテレーション
    for (i, val) in vec2.iter().enumerate() {
        println!("[{}] = {}", i, val);
    }

    // 変更可能なイテレーション
    let mut nums = vec![1, 2, 3];
    for val in &mut nums {
        *val *= 2;
    }
    println!("2倍: {:?}", nums); // [2, 4, 6]

    // 便利なメソッド
    println!("長さ: {}", vec2.len());
    println!("空?: {}", vec2.is_empty());
    println!("含む?: {}", vec2.contains(&30));

    // map, filter, collect
    let doubled: Vec<i32> = vec2.iter().map(|x| x * 2).collect();
    println!("2倍: {:?}", doubled);

    let evens: Vec<&i32> = vec2.iter().filter(|x| *x % 2 == 0).collect();
    println!("偶数: {:?}", evens);

    // ソート
    let mut unsorted = vec![3, 1, 4, 1, 5, 9];
    unsorted.sort();
    println!("ソート済み: {:?}", unsorted);

    // reverse, pop, remove
    let mut v = vec![1, 2, 3, 4, 5];
    v.reverse();
    println!("逆順: {:?}", v);
    let last = v.pop();
    println!("pop: {:?}", last); // Some(1)
}
```

### HashMap<K, V> / HashSet<T>

```rust
use std::collections::{HashMap, HashSet};

fn main() {
    // HashMap
    let mut scores = HashMap::new();
    scores.insert("Alice", 10);
    scores.insert("Bob", 20);

    // アクセス
    println!("Alice: {:?}", scores.get("Alice")); // Some(10)
    println!("Charlie: {:?}", scores.get("Charlie")); // None

    // エントリAPI
    scores.entry("Alice").or_insert(0);   // 既に存在するので何もしない
    scores.entry("Charlie").or_insert(30); // 存在しないので挿入

    // イテレーション
    for (name, score) in &scores {
        println!("{}: {}", name, score);
    }

    // 値の更新
    let text = "hello world wonderful world";
    let mut word_count = HashMap::new();
    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }
    println!("単語数: {:?}", word_count);

    // HashSet
    let mut set = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    set.insert(1); // 重複は無視される

    println!("セット: {:?}", set);
    println!("含む?: {}", set.contains(&2)); // true

    // 集合演算
    let a: HashSet<_> = [1, 2, 3, 4].iter().cloned().collect();
    let b: HashSet<_> = [3, 4, 5, 6].iter().cloned().collect();

    println!("積集合: {:?}", a.intersection(&b).collect::<Vec<_>>()); // [3, 4]
    println!("和集合: {:?}", a.union(&b).collect::<Vec<_>>());       // [1, 2, 3, 4, 5, 6]
    println!("差集合: {:?}", a.difference(&b).collect::<Vec<_>>());   // [1, 2]
}
```

### Rc<T> / Arc<T> - 参照カウント

`Rc<T>`（Reference Counted）は、単一スレッド内で複数の所有権を実現します。`Arc<T>`（Atomic Reference Counted）は、マルチスレッド対応版です。

```rust
use std::rc::Rc;
use std::sync::Arc;
use std::thread;

fn main() {
    // Rc<T> - 単一スレッド用の参照カウント
    let data = Rc::new(vec![1, 2, 3, 4, 5]);

    // cloneすると参照カウントが増える（データの複製ではない）
    let data2 = Rc::clone(&data);
    let data3 = Rc::clone(&data);

    println!("参照カウント: {}", Rc::strong_count(&data)); // 3

    // 全ての参照が同じデータを指している
    println!("data: {:?}", *data);
    println!("data2: {:?}", *data2);

    // グラフ構造の例
    #[derive(Debug)]
    struct Node {
        value: i32,
        next: Option<Rc<Node>>,
    }

    let shared = Rc::new(Node { value: 1, next: None });
    let node_a = Node { value: 2, next: Some(Rc::clone(&shared)) };
    let node_b = Node { value: 3, next: Some(Rc::clone(&shared)) };
    // shared は node_a と node_b の両方から参照される

    // Arc<T> - マルチスレッド用の参照カウント
    let data = Arc::new(vec![1, 2, 3]);

    let mut handles = vec![];
    for _ in 0..3 {
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            println!("スレッドでデータ: {:?}", *data_clone);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
```

### Cell<T> / RefCell<T> - 内部可変性

不変参照からでも値を変更できるようにするラッパーです。

```rust
use std::cell::{Cell, RefCell};

fn main() {
    // Cell - Copy型向け
    let cell = Cell::new(10);
    println!("Cell値: {}", cell.get());
    cell.set(20);
    println!("Cell値: {}", cell.get());

    // RefCell - 参照を返す型向け
    let ref_cell = RefCell::new(vec![1, 2, 3]);

    // 借用のルールは実行時にチェックされる
    {
        let mut borrowed = ref_cell.borrow_mut();
        borrowed.push(4);
    }

    println!("RefCell値: {:?}", ref_cell.borrow());
}
```
