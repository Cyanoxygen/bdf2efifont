use anyhow::{Context, Ok, Result, bail};
use std::{collections::HashMap, io::BufRead};

/// Represents a glyph bitmap.
///
/// A "narrow" glyph is defined as 8 pixels wide, and a wide glyph is 16 pixels wide.
///
/// A "cell" is defined as one narrow glyph, similar to other terminal emulators.
///
/// Each glyph is exactly 19 rows tall, possibly due to there are usually 3 pixels for
/// descend marks.
pub type BDFCharBitmap = Vec<Vec<u8>>;

/// Represents a defined character in the BDF format.
#[derive(Default, Debug, Clone)]
pub struct BDFChar {
	/// Name of the character, following STARTCHAR at the same line.
	pub charname: String,
	/// Device width (X, Y).
	pub dwidth: (u32, u32),
	/// Horizontal bounding box.
	pub bb_x: u32,
	/// Vertical bounding box.
	pub bb_y: u32,
	/// Offset of the glyph on the X axis.
	pub bb_offx: i32,
	/// Offset of the glyph on the Y axis.
	pub bb_offy: i32,
	/// The codepoint of the character.
	///
	/// EFI only uses UTF-16, however there will be characters that is
	/// out of this range.
	pub encoding: i64,
	/// The glyph bitmap of this character.
	pub bitmap: BDFCharBitmap,
	// NOTE: Rest of the properties that you can see in the
	// specification are ignored.
	// NOTE 2: Bounding boxes also specifyes the baseline information.
}

#[derive(Default)]
pub struct BDFFont {
	pub name: String,
	pub height: u32,
	pub num_chars: u32,
	pub chars: HashMap<u64, BDFChar>,
}

pub struct BDFParser {
}

fn next_line<I>(iter: &mut I) -> Result<(usize, String)>
	where I: Iterator<Item = (usize, Result<String, std::io::Error>)> {
	loop {
		if let Some((n, l)) = iter.next() {
			let l = l?;
			if l.is_empty() || l.to_ascii_lowercase().starts_with("comment") {
				continue;
			}
			return Ok((n, l));
		} else {
			bail!("Unexpected EOF while reading file")
		};
	}
}

impl BDFParser {
	fn consume_bitmap<I>(height: u32, iter: &mut I) -> Result<BDFCharBitmap>
	where I: Iterator<Item = (usize, Result<String, std::io::Error>)> {
		let mut last_line;
		let mut last_lineno;
		let mut remaining = height as i32;
		let mut bitmap = BDFCharBitmap::new();
		let mut padded_width = None;
		loop {
			(last_lineno, last_line) = next_line(iter)?;
			let trimmed = last_line.trim().to_ascii_lowercase();
			let trimmed = trimmed.as_str();
			let mut width = 0;
			if trimmed.eq_ignore_ascii_case("endchar") {
				for _ in 0..remaining {
					bitmap.push(vec![0u8; padded_width.unwrap_or(1)]);
				}
				return Ok(bitmap);
			}
			if trimmed.contains(|c: char| !c.is_ascii_hexdigit()) {
				bail!(format!("Unexpected line {} '{}' when processing bitmap data", last_lineno, last_line));
			}
			let mut iter = trimmed.chars();
			let mut row_cells = Vec::new();
			// Process hexadecimal encoded bitmap data
			while let Some(high) = iter.next() {
				// NOTE: Even though the BDF standard specifies that the bitmap data must be padded
				// to the nearest byte, I've seen some fonts that does not comply to this rule.
				let mut byte;
				byte = (high.to_digit(16).unwrap() as u8) << 4;
				byte |= iter.next().unwrap_or('0').to_digit(16).unwrap() as u8;
				row_cells.push(byte);
				width += 1;
			}
			if padded_width.is_none() {
				padded_width = Some(width);
			}
			bitmap.push(row_cells);
			remaining -= 1;
		}
	}

	fn consume_glyph<S, I>(font: &mut BDFFont, charname: S, iter: &mut I) -> Result<()>
	where 	I: Iterator<Item = (usize, Result<String, std::io::Error>)>,
		S: AsRef<str> {
		let mut last_line;
		let mut last_lineno;
		let mut cur_glyph = BDFChar::default();
		cur_glyph.charname = charname.as_ref().into();
		loop {
			(last_lineno, last_line) = next_line(iter)?;
			let tok = last_line.split_whitespace().collect::<Vec<_>>();
			match tok[0].to_ascii_lowercase().as_str() {
				// "ENDCHAR" will be handled by consume_bitmap().
				"bitmap" => {
					cur_glyph.bitmap = Self::consume_bitmap(cur_glyph.bb_y, iter).context(format!("Failed to process bitmap data for glyph {} '{}' at line {}", cur_glyph.encoding, charname.as_ref(), last_lineno))?;
					break;
				},
				"encoding" => {
					let codepoint = tok.get(1).context(format!("Expecting codepoint number at line {}", last_lineno))?.parse::<i64>()?;
					if codepoint < 0 {
						continue;
					}
					cur_glyph.encoding = codepoint;
				},
				"dwidth" => cur_glyph.dwidth = (
					tok.get(1).context(format!("Expecting device width at line {}", last_lineno))?.parse::<u32>()?,
					tok.get(2).context(format!("Expecting device height at line {}", last_lineno))?.parse::<u32>()?,
				),
				"bbx" => {
					cur_glyph.bb_x = tok.get(1).context(format!("Expecting X bounding box at line {}", last_lineno))?.parse::<u32>()?;
					cur_glyph.bb_y = tok.get(2).context(format!("Expecting Y bounding box at line {}", last_lineno))?.parse::<u32>()?;
					cur_glyph.bb_offx = tok.get(3).context(format!("Expecting X bounding box offset at line {}", last_lineno))?.parse::<i32>()?;
					cur_glyph.bb_offy = tok.get(4).context(format!("Expecting y bounding box offset at line {}", last_lineno))?.parse::<i32>()?;
				},
				_ => {}
			}
		}
		font.chars.insert(cur_glyph.encoding as u64, cur_glyph);
		Ok(())
	}

	fn consume_chars<I>(font: &mut BDFFont, iter: &mut I) -> Result<()>
	where I: Iterator<Item = (usize, Result<String, std::io::Error>)> {
		let mut last_line;
		let mut last_lineno;
		let mut remaining = font.num_chars ;
		loop {
			(last_lineno, last_line) = next_line(iter)?;
			let tok = last_line.split_whitespace().collect::<Vec<_>>();
			match tok[0].to_ascii_lowercase().as_str() {
				"endfont" => {
					return Ok(());
				}
				"startchar" => {
					let charname = if let Some(n) = tok.get(1) {
						n
					} else {
						"(unknown)"
					};
					Self::consume_glyph(font, charname, iter).context(format!("Error occurred when processing glyph {} at line {}", font.num_chars - remaining, last_lineno))?;
					remaining -= 1;
					continue;
				}
				_ => {
					bail!("WARNING: Unexpected line {} at line {}", last_line, last_lineno);
				}
			}
		}
	}

	fn consume_properties<I>(font: &mut BDFFont, iter: &mut I) -> Result<()>
	where I: Iterator<Item = (usize, Result<String, std::io::Error>)> {
		let mut last_line;
		let mut last_lineno;
		loop {
			(last_lineno, last_line) = next_line(iter)?;
			let tok = last_line.split_whitespace().collect::<Vec<_>>();
			match tok[0].to_ascii_lowercase().as_str() {
				"endproperties" => break,
				"family_name" => font.name = last_line.split_once(' ').context(format!("Expecting font family name at line {}", last_lineno))?.1.trim_matches(['\'', '"']).to_string(),
				_ => {}
			}
		}
		Ok(())
	}

	fn consume_font<I>(font: &mut BDFFont, iter: &mut I) -> Result<()>
	where I: Iterator<Item = (usize, Result<String, std::io::Error>)>
	{
		let mut last_line;
		let mut last_lineno;
		loop {
			(last_lineno, last_line) = next_line(iter)?;
			let tok = last_line.split_whitespace().collect::<Vec<_>>();
			match tok[0].to_ascii_lowercase().as_str() {
				"size" => {
					font.height = tok.get(1).context(format!("Expecting font size at line {}", last_lineno))?.parse::<u32>()?;
				},
				"startproperties" => {
					Self::consume_properties(font, iter)?;
				},
				"chars" => {
					font.num_chars = tok.get(1).context(format!("Expecting character count at line {}", last_lineno))?.parse::<u32>()?;
					Self::consume_chars(font, iter).context("Error occurred when processing character data")?;
					// NOTE: there is no ENDCHARS directive. ENDFONT is consumed by consume_chars().
					break;
				},

				_ => {},
			}
		}
		Ok(())
	}

	pub fn parse(reader: &mut impl BufRead) -> Result<BDFFont> {
		let mut font = BDFFont::default();

		let mut lines = reader.lines().enumerate();
		match lines.next() {
			Some((lineno, content)) => {
				let content = content.context(format!("Failed to read line {}", lineno))?;
				if !content.split_ascii_whitespace().next().unwrap().eq_ignore_ascii_case("startfont") {
					bail!(format!("Expecting STARTFONT at line {}", lineno));
				}
			},
			None => bail!("File is empty and nothing to read"),
		};

		BDFParser::consume_font(&mut font, &mut lines)?;
		Ok(font)
	}
}
