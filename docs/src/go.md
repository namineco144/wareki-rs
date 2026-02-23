# Go バインディング (`wareki-go`)

CGO (C Foreign Function Interface) を通じて Rust でビルドされた静的ライブラリ (`libwareki_c.a`) を直接リンクして利用するモジュールです。

## 前提事項

Go側から Rust コアの CFFI (C言語互換インターフェース) を呼び出す構成となっているため、**Rust のビルド環境が事前に整っている必要があります**。

## インストールとビルド
ソースファイルから手動でビルドし、環境パスを通す手順は以下の通りです。

```bash
# 1. Rust 側で静的ライブラリ (.a) をビルドする
cargo build --release -p wareki-c

# 2. CGO のリンカフラグ環境変数を設定する
export CGO_LDFLAGS="-L/path/to/maturin-build/target/release -lwareki_c"

# (※Linux環境などで動的リンクにフォールバックされる場合はこちらも要求されます)
export LD_LIBRARY_PATH="/path/to/maturin-build/target/release:$LD_LIBRARY_PATH"

# 3. Go でビルド / 実行
go build .
# または
go run main.go
```

依存先としてダウンロードする場合は以下のようになります。
```bash
go get github.com/namineco144/maturin-build/bindings/go
```

## 使い方

変換後の西暦は、Go 標準ライブラリである `time.Time` オブジェクトとして返ってきます。

```go
package main

import (
	"fmt"
	"time"
	// リモートの場合は "github.com/namineco144/maturin-build/bindings/go"
	"wareki" 
)

func main() {
	// 西暦 -> 和暦
	w, err := wareki.ToWareki(2026, 2, 23)
	if err == nil {
		fmt.Printf("%s%d年\n", w.EraName, w.Year) // "令和8年"
	}

	// 和暦 -> 西暦
	dt, err := wareki.FromWareki("令和", 8, 2, 23)
	if err == nil {
		fmt.Println(dt.Format("2006-01-02")) // "2026-02-23"
	}
}
```

## エラーハンドリング

C側からの例外やエラーが返ると、Goで使い慣れた標準の `error` インターフェースとして出力されます。以下のように処理できます。

```go
_, err := wareki.FromWareki("存在しない元号", 10, 1, 1)
if err != nil {
	fmt.Printf("変換に失敗しました: %v\n", err)
}
```
