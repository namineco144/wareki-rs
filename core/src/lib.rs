use chrono::{Datelike, NaiveDate};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Era {
    Meiji,
    Taisho,
    Showa,
    Heisei,
    Reiwa,
}

#[derive(Debug, Clone)]
pub struct EraData {
    pub era: Era,
    pub name: &'static str,
    pub short_name: &'static str,
    pub romaji: &'static str,
    pub start_date: NaiveDate,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Wareki {
    pub era: Era,
    pub era_name: &'static str,
    pub year: u32,
}

impl Wareki {
    pub fn era_name(&self) -> &'static str {
        self.era_name
    }
}

// 新しい順に定義
pub fn eras() -> Vec<EraData> {
    vec![
        EraData {
            era: Era::Reiwa,
            name: "令和",
            short_name: "令",
            romaji: "R",
            start_date: NaiveDate::from_ymd_opt(2019, 5, 1).unwrap(),
        },
        EraData {
            era: Era::Heisei,
            name: "平成",
            short_name: "平",
            romaji: "H",
            start_date: NaiveDate::from_ymd_opt(1989, 1, 8).unwrap(),
        },
        EraData {
            era: Era::Showa,
            name: "昭和",
            short_name: "昭",
            romaji: "S",
            start_date: NaiveDate::from_ymd_opt(1926, 12, 25).unwrap(),
        },
        EraData {
            era: Era::Taisho,
            name: "大正",
            short_name: "大",
            romaji: "T",
            start_date: NaiveDate::from_ymd_opt(1912, 7, 30).unwrap(),
        },
        EraData {
            era: Era::Meiji,
            name: "明治",
            short_name: "明",
            romaji: "M",
            start_date: NaiveDate::from_ymd_opt(1868, 1, 25).unwrap(), // 新暦換算
        },
    ]
}

/// 西暦から和暦への変換
pub fn to_wareki(year: i32, month: u32, day: u32) -> Result<Wareki, &'static str> {
    let date = NaiveDate::from_ymd_opt(year, month, day).ok_or("Invalid gregorian date")?;

    for era_data in eras() {
        if date >= era_data.start_date {
            let wareki_year = (year - era_data.start_date.year()) as u32 + 1;
            return Ok(Wareki {
                era: era_data.era,
                era_name: era_data.name,
                year: wareki_year,
            });
        }
    }

    Err("Date is out of supported range (before Meiji)")
}

/// 和暦から西暦への変換
/// era_str には "令和", "令", "R", "r" などを許容
pub fn from_wareki(era_str: &str, year: u32, month: u32, day: u32) -> Result<NaiveDate, &'static str> {
    if year == 0 {
        return Err("Year must be greater than 0");
    }

    let era_str_lower = era_str.to_lowercase();
    
    let era_data = eras().into_iter().find(|e| {
        e.name == era_str || e.short_name == era_str || e.romaji.to_lowercase() == era_str_lower
    }).ok_or("Unknown era")?;

    let gregorian_year = era_data.start_date.year() + (year as i32) - 1;
    
    let date = NaiveDate::from_ymd_opt(gregorian_year, month, day).ok_or("Invalid date for the given year/month/day (e.g. leap year mismatch or invalid day)")?;

    // 変換された日付が、元号の開始日より前になってしまう場合は不正 (例: 令和1年4月30日 などは平成なのでエラーとするか、許容するか。厳密には許容しない方が良い)
    if date < era_data.start_date {
        return Err("Date is before the start of the era");
    }
    
    // 次の元号の開始日以降の場合も不正とするか？ 一般的には許容されることもある（例：平成32年）が、要件に合わせてエラーとすることも可能。
    // 今回は厳密な変換を主眼とするため、次の元号に被る日付はエラーとする。
    if let Some(next_era) = eras().into_iter().rev().find(|e| e.start_date > era_data.start_date) {
        if date >= next_era.start_date {
            return Err("Date is after the end of the era");
        }
    }

    Ok(date)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_wareki_normal() {
        let w = to_wareki(2024, 5, 1).unwrap();
        assert_eq!(w.era, Era::Reiwa);
        assert_eq!(w.year, 6);

        let w = to_wareki(1989, 1, 8).unwrap();
        assert_eq!(w.era, Era::Heisei);
        assert_eq!(w.year, 1);
    }

    #[test]
    fn test_to_wareki_boundaries() {
        let w = to_wareki(1989, 1, 7).unwrap();
        assert_eq!(w.era, Era::Showa);
        assert_eq!(w.year, 64);
    }

    #[test]
    fn test_from_wareki_normal() {
        let d = from_wareki("令和", 6, 5, 1).unwrap();
        assert_eq!(d, NaiveDate::from_ymd_opt(2024, 5, 1).unwrap());

        let d2 = from_wareki("令", 6, 5, 1).unwrap();
        assert_eq!(d2, NaiveDate::from_ymd_opt(2024, 5, 1).unwrap());

        let d3 = from_wareki("r", 6, 5, 1).unwrap();
        assert_eq!(d3, NaiveDate::from_ymd_opt(2024, 5, 1).unwrap());
    }

    #[test]
    fn test_from_wareki_leap_year() {
        let d = from_wareki("令和", 6, 2, 29).unwrap();
        assert_eq!(d, NaiveDate::from_ymd_opt(2024, 2, 29).unwrap());

        let d_err = from_wareki("令和", 5, 2, 29);
        assert!(d_err.is_err());
    }

    #[test]
    fn test_from_wareki_invalid_date_for_era() {
        let d_err = from_wareki("令和", 1, 4, 30);
        assert!(d_err.is_err());
    }
}
