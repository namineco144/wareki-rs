import wareki
import datetime
import pytest

def test_to_wareki_normal():
    # 2026-02-23 -> 令和8年
    w = wareki.to_wareki(2026, 2, 23)
    assert w.era_name == "令和"
    assert w.year == 8
    assert str(w) == "令和8年"

def test_to_wareki_heisei():
    # 1989-01-08 -> 平成1年
    w = wareki.to_wareki(1989, 1, 8)
    assert w.era_name == "平成"
    assert w.year == 1

def test_to_wareki_error_before_meiji():
    # 明治以前 (1868-01-24)
    with pytest.raises(ValueError, match="Date is out of supported range"):
        wareki.to_wareki(1868, 1, 24)

def test_from_wareki_normal():
    # 令和8年2月23日 -> 2026-02-23
    d = wareki.from_wareki("令和", 8, 2, 23)
    assert d == datetime.date(2026, 2, 23)

def test_from_wareki_abbreviations():
    assert wareki.from_wareki("令", 8, 2, 23) == datetime.date(2026, 2, 23)
    assert wareki.from_wareki("r", 8, 2, 23) == datetime.date(2026, 2, 23)
    assert wareki.from_wareki("R", 8, 2, 23) == datetime.date(2026, 2, 23)

def test_from_wareki_leap_year():
    # 令和6年2月29日(2024年)
    d = wareki.from_wareki("令和", 6, 2, 29)
    assert d == datetime.date(2024, 2, 29)

def test_from_wareki_invalid_date():
    # 令和5年は閏年ではないためエラー
    with pytest.raises(ValueError):
        wareki.from_wareki("令和", 5, 2, 29)
