# Rust (Core) の使い方

全てのバインディングの基礎となる `wareki-core` クレートを直接 Rust プロジェクトから利用する方法です。

## インストール

`Cargo.toml` の依存関係に追加します。
(*※現在は未公開のため、ローカルパスまたはGitリポジトリのパスを指定します。*)

```toml
[dependencies]
wareki-core = { git = "https://github.com/namineco144/maturin-build.git" }
```

## 使用例

```rust
use wareki_core::{to_wareki, from_wareki, Era};
use chrono::NaiveDate;

fn main() {
    // 西暦から和暦への変換
    let w = to_wareki(2026, 2, 23).unwrap();
    println!("{}{}年", w.era_name(), w.year); // "令和8年"

    // 和暦から西暦への変換 (chrono::NaiveDate が返ります)
    let date = from_wareki("令和", 8, 2, 23).unwrap();
    println!("{}", date); // "2026-02-23"

    // 略称やローマ字の入力も可能です
    assert_eq!(from_wareki("R", 8, 2, 23).unwrap(), date);
}
```

## 返り値と型

- `to_wareki` は成功時に `Wareki` 構造体を返します。これには `era` (enum)、`era_name` (文字列スライス)、`year` (u32) が含まれます。
- `from_wareki` は成功時に `chrono::NaiveDate` を返します。
- エラー時には `Result::Err(&'static str)` が返ります。不正な日付や範囲外の日付を与えた場合のハンドリングに利用できます。
