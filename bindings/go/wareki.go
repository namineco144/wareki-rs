package wareki

/*
#cgo LDFLAGS: -L${SRCDIR}/../../target/release -lwareki_c
#include <stdlib.h>
#include <stdbool.h>

// C data structures
typedef struct {
    char* era_name;
    unsigned int year;
    bool is_error;
} CWareki;

typedef struct {
    int year;
    unsigned int month;
    unsigned int day;
    bool is_error;
} CGregorianDate;

// C functions exposed from Rust
extern CWareki to_wareki(int year, unsigned int month, unsigned int day);
extern CGregorianDate from_wareki(const char* era_name, unsigned int year, unsigned int month, unsigned int day);
extern void free_wareki_string(char* s);
*/
import "C"

import (
	"errors"
	"time"
	"unsafe"
)

type Wareki struct {
	EraName string
	Year    uint32
}

// ToWareki converts Gregorian date to Japanese calendar (Wareki)
func ToWareki(year int, month, day uint32) (Wareki, error) {
	cWareki := C.to_wareki(C.int(year), C.uint(month), C.uint(day))
	
	if cWareki.is_error {
		return Wareki{}, errors.New("date is out of supported range (before Meiji)")
	}
	
	// Convert C string to Go string and free the C string memory handled by Rust
	eraName := C.GoString(cWareki.era_name)
	C.free_wareki_string(cWareki.era_name)
	
	return Wareki{
		EraName: eraName,
		Year:    uint32(cWareki.year),
	}, nil
}

// FromWareki converts Japanese calendar (Wareki) to Gregorian date (time.Time)
func FromWareki(eraName string, year, month, day uint32) (time.Time, error) {
	cEraName := C.CString(eraName)
	defer C.free(unsafe.Pointer(cEraName))

	cDate := C.from_wareki(cEraName, C.uint(year), C.uint(month), C.uint(day))

	if cDate.is_error {
		return time.Time{}, errors.New("invalid wareki date or unknown era")
	}

	return time.Date(int(cDate.year), time.Month(cDate.month), int(cDate.day), 0, 0, 0, 0, time.UTC), nil
}
