use lazy_regex::{regex_find, regex_captures};

use crate::utils::pretoken::{PreToken,PreTokenLexer};
use logos::Logos;

// Control chars [ \n\x0B\t\x0C]

#[derive(Debug)]
pub struct PreprocessingToken {
	pub kind: PreToken,
	pub originalDiff : usize,
	pub originalDiffEnd : usize,
}

#[derive(Debug)]
pub struct PreLexer<'a> {
	enableHeader: u32,
	currentNonSpliced: &'a str,
	current: String,
	diff: usize,
	line: u32,
	column: u32,
}
impl<'a> PreLexer<'a> {
	pub fn new(content: &String)->PreLexer {
		PreLexer{
			enableHeader: 0,
			currentNonSpliced: content,
			current: content.clone(),
			diff: 0,
			line: 1,
			column: 1,
		}
	}

	pub fn expectHeader(&mut self) {
		self.enableHeader += 1;
	}

	pub fn doNotExpectHeader(&mut self) {
		if self.enableHeader > 0 {
			self.enableHeader -= 1;
		}
	}

	fn spliceNewlinePosition(&self) -> Option<usize> {
		/* This is going to be the next nl found. IT DOES NOT DELIMIT TOKENS.
			if the next one is \n and the previous is a "\", it needs to be spliced.
			If we're unable to generate the token, or the token generated reaches
			the "\", then we splice and try again.
		*/
		let mut maybe_remove : Option<usize> = None;
		if Some("\n") == regex_find!(r"[\n]", &self.current) {
			let salt_pos: usize = self.current.chars().position(|x:char| x == '\n').unwrap();
			if salt_pos > 0 && self.current.chars().nth(salt_pos-1) == Some('\\') {
				maybe_remove = Some(salt_pos-1);
			}
		}
		return maybe_remove;
	}

	fn getNextTokenNonSpliced(&mut self) -> (Option<PreToken>, usize) {
		if self.enableHeader > 0 {
			if let Some(res) = regex_find!(r#"^(?:<[^\n>]+>|"[^\n"]+")"#, &self.current) {
				return (Some(PreToken::HeaderName(res.to_string())), res.len());
			}
		}
		let mut lex = PreTokenLexer::lexer(&self.current);
		if let Some(idxLex) = lex.next() {
			let content = lex.slice().to_string();
			let len = content.len();
			match idxLex {
				PreTokenLexer::RawStringLiteral => {
					if let Some ((_,key)) = regex_captures!(r#"R"(.*)\("#, &content) {
						if let Some(position) = self.current.find((")".to_owned() + key + "\"").as_str()) {
							return (Some(PreToken::RawStringLiteral(self.current[0..position+key.len()+2].to_string())), position+key.len()+2);
						}
					}
				}
				PreTokenLexer::Error => {
					let errContent = lex.slice().to_string();
					let len = errContent.len();
					return (Some(PreToken::Unknown(errContent)), len);
				}
				_ => {
					return (Some(PreToken::new(idxLex, content)),len);
				}
			}
		}
		return (None, 0);
	}

	fn applySplice(&mut self, splice_point: usize) -> () {
		self.current.remove(splice_point);
		self.current.remove(splice_point);
	}

	fn getNextTokenData(&mut self) -> (Option<PreToken>, usize, usize) {
		let (mut kind, mut idx, mut splices) = (None, 0, 0);
		let prevCurrent = self.current.to_string();
		loop {
			if self.current.is_empty() {break;}
			else {
				if let Some(_) = regex_find!(r#"^<::[^:>]"#, &self.current)
				{
					(kind, idx) = (Some(PreToken::new(PreTokenLexer::OperatorPunctuator, "<".to_string())), 1);
					break;
				} else {
					let splice_point_slash_nl = self.spliceNewlinePosition();
					(kind, idx) = self.getNextTokenNonSpliced();
					if splice_point_slash_nl.contains(&idx) || (match kind {Some(PreToken::Unknown(_)) => true, _ => false} && splice_point_slash_nl.is_some()) {
						self.applySplice(splice_point_slash_nl.unwrap());
						splices += 1;
						continue;
					} else if match kind {Some(PreToken::Unknown(_)) => true, _ => false} {
						self.current = prevCurrent;
						splices = 0;
						let splice_point_slash_nl = self.spliceNewlinePosition();
						if splice_point_slash_nl.contains(&idx) {
							self.applySplice(splice_point_slash_nl.unwrap());
							splices += 1;
						}
						break;
					}else if kind.is_some() {
						break;
					} else {
						eprintln!("Encountered unmachable preprocessing token at: {} {}", self.line, self.column);
						return (None, 0,0);
					}
				}
			}
		}
		return (kind, idx, splices);
	}

}
impl<'a> Iterator for PreLexer<'a> {
	type Item = PreprocessingToken;
	fn next(&mut self) -> Option<Self::Item> {
		let mut res: Option<Self::Item> = None;
		let (kind, idx, splices) = self.getNextTokenData();
		if let Some(kind) = kind {
			let (mut lineEnd, mut columnEnd) = (self.line, self.column);
			{
				let (mut idxCpy, mut splicesCpy) = (idx as i64, splices as i64);
				for charGud in self.currentNonSpliced.chars() {
					idxCpy-=1;
					columnEnd += 1;
					if charGud == '\n' {
						columnEnd = 1;
						lineEnd += 1;
						if splicesCpy > 0 {
							splicesCpy -= 1;
							idxCpy+=2;
						}
					}
					if splicesCpy == 0 && idxCpy == 0 {
						break;
					}
				}
			}
			let mut originalString = &self.currentNonSpliced[0..idx+splices*2];
			if originalString.ends_with("\\\n") {
				originalString = &self.currentNonSpliced[0..idx+splices*2-2];
			}
			res = Some(
				Self::Item {
					kind: kind,
					originalDiff: self.diff,
					originalDiffEnd: self.diff+originalString.len(),
				}
			);
			self.diff+=idx+splices*2;
			self.currentNonSpliced = &self.currentNonSpliced[idx+splices*2..];
			self.current = self.current[idx..].to_string();
			(self.line, self.column) = (lineEnd, columnEnd);
		}
		return res;
	}
}