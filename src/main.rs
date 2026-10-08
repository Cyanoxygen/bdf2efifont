#![allow(dead_code)]
use anyhow::{Context, Result};
use clap::{Parser};
use std::{env::args, fs::File, io::BufReader, path::{Path, PathBuf}};

use crate::{parser::BDFParser, renderer::{convert, render}};

mod parser;
mod renderer;
mod mini_unicode;

#[derive(Parser)]
struct CmdLine {

}

fn main() -> Result<()> {
	let argv1 = args().nth(1).context("File name expected")?;
	let src_path = Path::new(&argv1);
	let fd = File::options().read(true).write(false).truncate(false).open(&argv1).context("Failed to open file for reading")?;
	let mut bufreader = BufReader::with_capacity(1 << 12, fd);

	eprintln!("Parsing font ...");
	let font = BDFParser::parse(&mut bufreader)?;
	eprintln!("Font: {}", font.name);
	eprintln!("Num of characters: {}", font.num_chars);
	eprintln!("Converting to EFI font data ...");
	let converted = convert(&font)?;
	eprintln!("Generating output files ...");

	let stem = src_path.file_stem().context("Unable to get path information for the input file")?;
	let dst_path = PathBuf::from(src_path.parent().context("Unable to get path information for the input file")?);
	let dst_path = dst_path.join(stem);
	render(&converted, font.name, &dst_path.to_string_lossy(), "gUsStd")?;
	eprintln!("Done!");
	eprintln!("Narrow glyph table	: {}_narrow.c", "output");
	eprintln!("Wide glyph table	: {}_wide.c", "output");
	Ok(())
}
