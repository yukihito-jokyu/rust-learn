// TODO: モジュール `math` を定義してください
// - 公開関数 `add(a: i32, b: i32) -> i32`
// - 公開関数 `multiply(a: i32, b: i32) -> i32`
// - 非公開関数 `internal_helper()`
// ヒント: mod math { pub fn add(...) { ... } }

// TODO: ネストしたモジュール `network` を定義してください
// - network::connect() （公開）
// - network::server::start() （公開）
// - server::start() から super::connect() を呼び出す
// ヒント: mod network { pub mod server { ... } pub fn connect() { ... } }

// TODO: 公開構造体 `User` を定義するモジュール `models` を作成してください
// - 公開フィールド: name: String
// - 非公開フィールド: password: String
// - 公開コンストラクタ: new(name: &str, password: &str) -> Self
// ヒント: pub struct User { pub name: String, password: String }

// TODO: use 宣言を使ってモジュールの関数をインポートしてください
// ヒント: use math::add; のように記述します

// モジュールの定義
mod english {
    pub fn greet() {
        println!("Hello!");
    }

    fn secret() {
        println!("This is private");
    }
}

// 構造体の可視化
mod shapes {
    pub struct Point {
        pub x: i32,
        pub y: i32,
        _z: i32, // 非公開フィールド（先頭に _ をつけて未使用警告を抑制）
    }

    impl Point {
        // 公開コンストラクタ（非公開フィールドも初期化できる）
        pub fn new(x: i32, y: i32) -> Self {
            Self { x, y, _z: 0 }
        }
    }
}

// モジュールの可視性
mod outer {
    pub mod inner {
        pub fn public_function() {
            println!("公開関数");
        }

        pub(crate) fn crate_visible() {
            println!("クレート内公開");
        }

        pub(super) fn super_visible() {
            println!("親モジュールの未公開")
        }

        fn private_function() {
            println!("非公開関数");
        }
    }

    pub fn try_access() {
        inner::public_function(); // OK: 公開
        inner::crate_visible(); // OK: 同じクレート内
        inner::super_visible(); // OK: 親モジュール
        // inner::private_function(); // NG: 親モジュールからは非公開関数にアクセス不可
    }
}

// useでモジュールのパスを短く記述
mod geometry {
    pub mod circle {
        pub fn area(radius: f64) -> f64 {
            std::f64::consts::PI * radius * radius
        }

        pub fn circumference(radius: f64) -> f64 {
            2.0 * std::f64::consts::PI * radius
        }
    }
}

use geometry::circle::area;
use geometry::circle::circumference;

// superとself
mod network {
    pub fn connect() {
        println!("Connecting...");
    }

    pub mod server {
        pub fn start() {
            // super で親モジュールの関数を呼ぶ
            super::connect();
            println!("Server started");
        }

        pub fn status() {
            // self で同じモジュール内の関数を呼ぶ
            self::start();
        }
    }
}

fn main() {
    // ここにコードを書いてください
    english::greet();

    outer::inner::public_function();
    outer::inner::crate_visible();
    outer::try_access();

    let p = shapes::Point::new(1, 2);
    println!("x: {}, y: {}", p.x, p.y);

    println!("area: {}", area(5.0));
    println!("circumference: {}", circumference(5.0));

    network::server::status();
}
