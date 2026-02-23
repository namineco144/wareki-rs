package wareki

import (
	"testing"
	"time"
)

func TestToWarekiNormal(t *testing.T) {
	w, err := ToWareki(2026, 2, 23)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if w.EraName != "令和" {
		t.Errorf("expected EraName 令和, got %s", w.EraName)
	}
	if w.Year != 8 {
		t.Errorf("expected Year 8, got %d", w.Year)
	}
}

func TestToWarekiHeisei(t *testing.T) {
	w, err := ToWareki(1989, 1, 8)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if w.EraName != "平成" {
		t.Errorf("expected EraName 平成, got %s", w.EraName)
	}
	if w.Year != 1 {
		t.Errorf("expected Year 1, got %d", w.Year)
	}
}

func TestToWarekiOutOfRange(t *testing.T) {
	_, err := ToWareki(1868, 1, 24)
	if err == nil {
		t.Fatal("expected error for date before Meiji, got nil")
	}
}

func TestFromWarekiNormal(t *testing.T) {
	date, err := FromWareki("令和", 8, 2, 23)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	expected := time.Date(2026, time.February, 23, 0, 0, 0, 0, time.UTC)
	if !date.Equal(expected) {
		t.Errorf("expected %v, got %v", expected, date)
	}
}

func TestFromWarekiAbbreviations(t *testing.T) {
	tests := []string{"令", "r", "R"}
	expected := time.Date(2026, time.February, 23, 0, 0, 0, 0, time.UTC)

	for _, era := range tests {
		date, err := FromWareki(era, 8, 2, 23)
		if err != nil {
			t.Fatalf("expected no error for %s, got %v", era, err)
		}
		if !date.Equal(expected) {
			t.Errorf("expected %v, got %v", expected, date)
		}
	}
}

func TestFromWarekiLeapYear(t *testing.T) {
	date, err := FromWareki("令和", 6, 2, 29)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	expected := time.Date(2024, time.February, 29, 0, 0, 0, 0, time.UTC)
	if !date.Equal(expected) {
		t.Errorf("expected %v, got %v", expected, date)
	}
}

func TestFromWarekiInvalidDate(t *testing.T) {
	// 令和5年は閏年ではない
	_, err := FromWareki("令和", 5, 2, 29)
	if err == nil {
		t.Fatal("expected error for invalid leap year, got nil")
	}
}
