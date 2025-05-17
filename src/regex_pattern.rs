// MEMO: データの検知や抽出等の処理に使用する正規表現をまとめたモジュール

use once_cell::sync::Lazy;
use regex::Regex;

// スケールファクタ抽出用（例: 7845(gal)/8223790）
pub static RE_SCALE_FACTOR: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?P<numerator>\d+)\(gal\)/(?P<denominator>\d+)")
        .expect("Failed to initialize the regular expression for scale factor extraction.")
});

#[cfg(test)]
mod tests {
    use crate::regex_pattern::RE_SCALE_FACTOR;

    #[test]
    fn test_scale_factor_match_basic() {
        let text = "7845(gal)/8223790";
        let caps = RE_SCALE_FACTOR.captures(text).unwrap();
        assert_eq!(&caps["numerator"], "7845");
        assert_eq!(&caps["denominator"], "8223790");
    }

    #[test]
    fn test_scale_factor_with_spaces() {
        let text = "  123(gal)/4567  ";
        let caps = RE_SCALE_FACTOR.captures(text.trim()).unwrap();
        assert_eq!(&caps["numerator"], "123");
        assert_eq!(&caps["denominator"], "4567");
    }

    #[test]
    fn test_scale_factor_no_match() {
        let text = "no scale factor here";
        assert!(RE_SCALE_FACTOR.captures(text).is_none());
    }

    #[test]
    fn test_scale_factor_partial_string() {
        let text = "prefix 9999(gal)/8888 suffix";
        let caps = RE_SCALE_FACTOR.captures(text).unwrap();
        assert_eq!(&caps["numerator"], "9999");
        assert_eq!(&caps["denominator"], "8888");
    }

    #[test]
    fn test_scale_factor_multiple_matches() {
        let text = "First: 1(gal)/2, Second: 3(gal)/4";
        let all: Vec<_> = RE_SCALE_FACTOR.captures_iter(text).collect();
        assert_eq!(all.len(), 2);
        assert_eq!(&all[0]["numerator"], "1");
        assert_eq!(&all[0]["denominator"], "2");
        assert_eq!(&all[1]["numerator"], "3");
        assert_eq!(&all[1]["denominator"], "4");
    }

    #[test]
    fn test_scale_factor_large_numbers() {
        let text = "1234567890(gal)/9876543210";
        let caps = RE_SCALE_FACTOR.captures(text).unwrap();
        assert_eq!(&caps["numerator"], "1234567890");
        assert_eq!(&caps["denominator"], "9876543210");
    }
}
