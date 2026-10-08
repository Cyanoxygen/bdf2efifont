use std::range::Range;

#[derive(PartialEq, Eq)]
pub enum AllowedUnicodeRanges {
	/// Basic Latin (U+0020 - U+007F)
	BasicLatin,
	/// Latin 1 Supplement (U+0080 - U+00FF)
	Latin1Supplement,
	/// Latin Extended A (U+0100 - U+017F)
	LatinExtendedA,
	/// Latin Extended B (U+0180 - U+024F)
	LatinExtendedB,
	/// Cyrillic (U+0400 - U+04FF)
	Cyrillic,
	/// Arrows (U+2190 - U+21FF)
	Arrows,
	/// Box Drawing Characters (U+2500 - U+257F)
	BoxDrawing,
	/// Block Elements (U+2580 - U+259F)
	BlockElements,
	/// Geometric Shapes (U+25A0 - U+25FF)
	GeometricShapes,
	/// CJK Punctuations (U+3000 - U+303F)
	CJKPunctuations,
	/// CJK Unified Ideographs (U+4E00 - U+9FFF)
	CJKUnifiedIdeographs,
	/// Half-Width And Full-Width Forms (U+FF00 - U+FFEF)
	HalfAndFullWidthForms,
}

const ALL_RANGES: &[AllowedUnicodeRanges] = &[
	AllowedUnicodeRanges::BasicLatin,
	AllowedUnicodeRanges::Latin1Supplement,
	AllowedUnicodeRanges::LatinExtendedA,
	AllowedUnicodeRanges::LatinExtendedB,
	AllowedUnicodeRanges::Cyrillic,
	AllowedUnicodeRanges::Arrows,
	AllowedUnicodeRanges::BoxDrawing,
	AllowedUnicodeRanges::BlockElements,
	AllowedUnicodeRanges::GeometricShapes,
	AllowedUnicodeRanges::CJKPunctuations,
	AllowedUnicodeRanges::CJKUnifiedIdeographs,
	AllowedUnicodeRanges::HalfAndFullWidthForms,
];

impl AllowedUnicodeRanges {
	pub fn to_range(&self) -> Range<char> {
		match self {
			// We intentionally ignore 0x00 - 0x1F.
			Self::BasicLatin	=> '\u{0020}'..'\u{007F}',
			// We intentionally ignore U+0080 - u+00A1 (NBSP).
			Self::Latin1Supplement	=> '\u{00A1}'..'\u{00FF}',
			Self::LatinExtendedA	=> '\u{0100}'..'\u{017F}',
			Self::LatinExtendedB	=> '\u{0180}'..'\u{024F}',
			Self::Cyrillic		=> '\u{0400}'..'\u{04FF}',
			Self::Arrows		=> '\u{2190}'..'\u{2199}',
			Self::BoxDrawing	=> '\u{2500}'..'\u{257F}',
			Self::BlockElements	=> '\u{2580}'..'\u{259F}',
			// We intentionally ignore 25D0 - 25FF.
			Self::GeometricShapes	=> '\u{25A0}'..'\u{25CF}',
			Self::CJKPunctuations	=> '\u{3000}'..'\u{303F}',
			Self::CJKUnifiedIdeographs
						=> '\u{4E00}'..'\u{9FFF}',
			Self::HalfAndFullWidthForms
						=> '\u{FF00}'..'\u{FFEF}',

		}.into()
	}

	pub fn all_ranges() -> Vec<(&'static AllowedUnicodeRanges, Range<char>)> {
		ALL_RANGES.iter().map(|r| (r, r.to_range())).collect::<Vec<_>>()
	}

	pub fn to_str(&self) -> &'static str {
		match self {
		    AllowedUnicodeRanges::BasicLatin => "Basic Latin 1",
		    AllowedUnicodeRanges::Latin1Supplement => "Latin 1 Supplement",
		    AllowedUnicodeRanges::LatinExtendedA => "Latin Extended A",
		    AllowedUnicodeRanges::LatinExtendedB => "Latin Extended B",
		    AllowedUnicodeRanges::Cyrillic => "Cyrillic",
		    AllowedUnicodeRanges::Arrows => "Arrows",
		    AllowedUnicodeRanges::BoxDrawing => "Box Drawing",
		    AllowedUnicodeRanges::BlockElements => "Block Elements",
		    AllowedUnicodeRanges::GeometricShapes => "Geometric Shapes",
		    AllowedUnicodeRanges::CJKPunctuations => "CJK Punctuations",
		    AllowedUnicodeRanges::CJKUnifiedIdeographs => "CJK Unified Ideographs",
		    AllowedUnicodeRanges::HalfAndFullWidthForms => "Half- and Full-Width Forms",
		}
 	}
}
