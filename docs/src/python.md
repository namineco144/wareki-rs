# Python バインディング (`wareki-python`)

Python ネイティブ拡張として動作するバインディングです。
内部で `PyO3` を利用しており、高速な相互変換を提供します。

## インストール
ソースコードからビルドするには、以下の手順を実行してください。

```bash
cd bindings/python
python -m venv .venv
source .venv/bin/activate
pip install maturin
maturin develop
```

## 使い方

変換後の西暦は、Python標準ライブラリの `datetime.date` オブジェクトとして返ってきます。

```python
import wareki
import datetime

# 西暦 -> 和暦
w = wareki.to_wareki(2026, 2, 23)
print(f"{w.era_name}{w.year}年") # "令和8年"

# 和暦 -> 西暦
dt = wareki.from_wareki("令和", 8, 2, 23)
print(dt.isoformat()) # "2026-02-23"

# 略称もサポート
dt2 = wareki.from_wareki("R", 8, 2, 23)
print(dt == dt2) # True
```

## エラーハンドリング

無効な日付などを入力した場合は `ValueError` がスローされます。

```python
try:
    # 存在しない日付 (令和5年に2月29日はありません)
    wareki.from_wareki("令和", 5, 2, 29)
except ValueError as e:
    print(f"エラー: {e}")
```
