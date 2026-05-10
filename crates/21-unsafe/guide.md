# 21 - Unsafe Rust

## 学習内容
- `unsafe`ブロック
- unsafe関数
- 生ポインタ（Raw Pointers）
- 可変静的変数（Mutable Statics）
- unsafe impl
- FFI（Foreign Function Interface）
- unsafeの安全性ガイドライン

## 参考ページ
https://doc.rust-jp.rs/rust-by-example-ja/unsafe.html

## 内容

### unsafeブロックの基本

Rustの安全性保証は、5つの主要なチェックによって守られています。`unsafe`ブロックはこれらのチェックの一部を無効にし、プログラマが手動で安全性を保証する責任を負います。

unsafeブロックで可能になる操作（「unsafeスーパーパワー」）：
1. **生ポインタのデリファレンス**
2. **unsafe関数やメソッドの呼び出し**
3. **可変静的変数へのアクセスや変更**
4. **unsafe implの実装**
5. **`union`のフィールドへのアクセス**

```rust
fn main() {
    // unsafeブロックの基本的な使い方
    unsafe {
        // ここでunsafe操作が可能
        println!("unsafeブロック内で実行");
    }

    println!("unsafeブロックの外は安全");
}
```

### 生ポインタ（Raw Pointers）

生ポインタには`*const T`（不変）と`*mut T`（可変）の2種類があります。参照とは異なり、借用チェッカーの監視を受けません。

```rust
fn main() {
    // 参照から生ポインタを作成
    let mut num = 5;

    let r1 = &num as *const i32;  // 不変生ポインタ
    let r2 = &mut num as *mut i32; // 可変生ポインタ

    // 生ポインタのデリファレンスはunsafeブロック内でのみ可能
    unsafe {
        println!("r1: {}", *r1); // 5
        println!("r2: {}", *r2); // 5
    }

    // メモリアドレスを直接指定して生ポインタを作成（危険！）
    let address = 0x012345usize;
    let _raw = address as *const i32;

    // 注意: 上記のような直接アドレス指定は未定義動作を引き起こす可能性が高い
    // 実際には使用を避けるべき

    // Boxから生ポインタを取得
    let mut boxed = Box::new(42);
    let raw_ptr: *mut i32 = &mut *boxed;
    unsafe {
        println!("Box内の値: {}", *raw_ptr);
    }

    // ポインタ演算
    let arr: [i32; 5] = [10, 20, 30, 40, 50];
    let ptr = arr.as_ptr();

    unsafe {
        println!("arr[0] = {}", *ptr);
        println!("arr[2] = {}", *ptr.add(2)); // ポインタのオフセット
        println!("arr[4] = {}", *ptr.add(4));
    }
}
```

### unsafe関数

`unsafe fn`で定義された関数は、unsafeブロック内でのみ呼び出せます。

```rust
// unsafe関数の定義
unsafe fn dangerous() {
    println!("危険な操作を実行中...");
}

// unsafeブロック内でのみ呼び出し可能
fn main() {
    // dangerous(); // エラー! unsafeブロックが必要
    unsafe {
        dangerous();
    }
}

// unsafe関数は安全なAPIを構築するためにも使われる
fn safe_wrapper() {
    // 外部からは安全に見えるが、内部でunsafeを使用
    unsafe {
        dangerous();
    }
}
```

### 可変静的変数（Mutable Statics）

静的変数はプログラムの全期間で存在し、グローバルにアクセス可能です。可変静的変数へのアクセスはデータ競合のリスクがあるため、unsafeが必要です。

```rust
// 不変静的変数（安全）
static LANGUAGE: &str = "Rust";

// 可変静的変数（unsafeが必要）
static mut COUNTER: u32 = 0;

// 定数（インライン展開される）
const MAX_POINTS: u32 = 100_000;

fn add_to_counter(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    println!("言語: {}", LANGUAGE);
    println!("最大ポイント: {}", MAX_POINTS);

    add_to_counter(10);
    add_to_counter(5);

    unsafe {
        println!("カウンター: {}", COUNTER); // 15
    }

    // atomic型を使えばunsafeなしで安全に変更可能
    use std::sync::atomic::{AtomicU32, Ordering};
    static ATOMIC_COUNTER: AtomicU32 = AtomicU32::new(0);

    ATOMIC_COUNTER.fetch_add(1, Ordering::SeqCst);
    ATOMIC_COUNTER.fetch_add(1, Ordering::SeqCst);
    println!("アトミックカウンター: {}", ATOMIC_COUNTER.load(Ordering::SeqCst)); // 2
}
```

### unsafe impl

`unsafe`トレイトの実装や、コンパイラが自動的に検証できないトレイトの実装に使います。

```rust
use std::marker::Send;
use std::ptr::NonNull;

// Sendトレイトは通常、コンパイラが自動的に実装を判定する
// しかし、生ポインタを含む型などは自動実装されない
// unsafe implで安全性を手動で宣言できる

struct MyBox<T> {
    ptr: NonNull<T>,
}

// NonNullはnullになり得ないことが保証されているため、
// Sendの実装は安全と判断できる（TがSendの場合のみ）
unsafe impl<T: Send> Send for MyBox<T> {}

// Syncも同様
unsafe impl<T: Sync> Sync for MyBox<T> {}

// 4つの自動トレイト（auto traits）
// Send - 別スレッドに移動可能
// Sync - 複数スレッドから参照可能
// Unpin - ピン留め解除可能
// Sized - コンパイル時にサイズが既知

fn main() {
    println!("unsafe implの例");
}
```

### Union（共用体）

共用体はC言語との相互運用で使われます。フィールドへのアクセスはunsafeが必要です。

```rust
// 共用体の定義
#[repr(C)]
union IntOrFloat {
    i: i32,
    f: f32,
}

fn main() {
    let mut u = IntOrFloat { i: 42 };

    // アクセスにはunsafeが必要
    unsafe {
        println!("整数として: {}", u.i); // 42
    }

    u.f = 3.14;

    // 最後に書き込んだフィールド以外へのアクセスは未定義動作の可能性
    unsafe {
        println!("浮動小数点として: {}", u.f); // 3.14
        // println!("整数として: {}", u.i); // 未定義動作の可能性
    }
}
```

### 外部関数の呼び出し（FFI）

```rust
// 外部C関数の宣言
extern "C" {
    fn abs(input: i32) -> i32;
    fn strlen(s: *const i8) -> usize;
}

// Rustの関数をCから呼び出せるようにエクスポート
#[no_mangle]
pub extern "C" fn call_from_c() -> i32 {
    42
}

fn main() {
    // 外部関数の呼び出し
    let x = -10;
    let y = unsafe { abs(x) };
    println!("abs({}) = {}", x, y); // abs(-10) = 10

    // C文字列の長さ
    use std::ffi::CString;
    let c_str = CString::new("hello").unwrap();
    let len = unsafe { strlen(c_str.as_ptr()) };
    println!("文字列の長さ: {}", len); // 5
}
```

### unsafeの安全性ガイドライン

unsafeを使用する際の重要なルール：

1. **データ競合を防ぐ**: 複数スレッドからの同時アクセス・変更を避ける
2. **メモリ安全性を保証**: 解放後使用、二重解放、ヌルポインタ参照を防ぐ
3. **正しいライフタイムを維持**: ダングリングポインタを防ぐ
4. **不変条件を維持**: 型が期待する不変条件が守られることを保証する
5. **unsafeの範囲を最小化**: 必要最小限のコードだけをunsafeブロックに入れる
6. **安全なAPIを提供**: ユーザーがunsafeを使わずに利用できるようにする

```rust
// 良い例: unsafeな内部実装を安全なAPIで包む
struct Vec2D {
    data: *mut f64,
    len: usize,
    capacity: usize,
}

impl Vec2D {
    fn new() -> Self {
        Vec2D {
            data: std::ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }

    fn push(&mut self, val: f64) {
        // 内部でunsafeを使用するが、外部からは安全に使える
        if self.len == self.capacity {
            self.grow();
        }
        unsafe {
            std::ptr::write(self.data.add(self.len), val);
        }
        self.len += 1;
    }

    fn get(&self, index: usize) -> Option<f64> {
        if index < self.len {
            unsafe { Some(std::ptr::read(self.data.add(index))) }
        } else {
            None
        }
    }

    fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
        let new_layout = std::alloc::Layout::array::<f64>(new_capacity).unwrap();
        let new_data = unsafe { std::alloc::alloc(new_layout) as *mut f64 };

        if !self.data.is_null() {
            unsafe {
                std::ptr::copy_nonoverlapping(self.data, new_data, self.len);
                let old_layout = std::alloc::Layout::array::<f64>(self.capacity).unwrap();
                std::alloc::dealloc(self.data as *mut u8, old_layout);
            }
        }

        self.data = new_data;
        self.capacity = new_capacity;
    }
}

impl Drop for Vec2D {
    fn drop(&mut self) {
        if !self.data.is_null() {
            unsafe {
                let layout = std::alloc::Layout::array::<f64>(self.capacity).unwrap();
                std::alloc::dealloc(self.data as *mut u8, layout);
            }
        }
    }
}
```
