# Node.js バインディング (`wareki-node`)

`napi-rs` によって構築された Node.js 用の高速な C++ バックエンドアドオンです。
JSから同期関数として利用できます。

## インストール

```bash
npm install wareki-node
```

ソースコードからビルドする場合は、リポジトリを clone し、以下の手順を踏みます。

```bash
cd bindings/node
npm install
npm run build
```

## 使い方

```javascript
const wareki = require('wareki-node'); // ビルド環境のディレクトリでは require('./index.js') となります

// 西暦 -> 和暦
const w = wareki.toWareki(2026, 2, 23);
console.log(`${w.eraName}${w.year}年`); // "令和8年"

// 和暦 -> 西暦 (ISO 8601形式の文字列として返ります)
const ds = wareki.fromWareki("令和", 8, 2, 23);
console.log(ds); // "2026-02-23"
```

## エラーハンドリング

Node.js パッケージでは、不正な引数や範囲外の日付が渡されると JavaScript の `Error` (Status: InvalidArg) がスローされます。

```javascript
try: {
    wareki.toWareki(1868, 1, 24);
} catch (e) {
    console.error(e.message); // Date is out of supported range
}
```
