use logos::{Logos};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PreprocessingOperator {
	Hash,
	HashHash,
}
impl PreprocessingOperator {
	fn to_str(&self) -> &str {
		return match self {
			PreprocessingOperator::Hash => "#",
			PreprocessingOperator::HashHash => "##",
		};
	}
}

#[derive(PartialEq, Debug, Logos)]
pub enum PreTokenLexer {
	#[regex(r"[a-zA-Z_[^\x00-\x7F]][a-zA-Z0-9_[^\x00-\x7F]]*")]
	Ident,
	#[token(r"#")]
	#[token(r"%:")]
	PreprocessingOperatorHash,
	#[token(r"##")]
	#[token(r"%:%:")]
	PreprocessingOperatorHashHash,
	#[token(r"{")]
	#[token(r"}")]
	#[token(r"[")]
	#[token(r"]")]
	#[token(r"(")]
	#[token(r")")]
	#[token(r"<:")]
	#[token(r":>")]
	#[token(r"<%")]
	#[token(r"%>")]
	#[token(r";")]
	#[token(r":")]
	#[token(r"...")]
	#[token(r"?")]
	#[token(r"::")]
	#[token(r".")]
	#[token(r".*")]
	#[token(r"->")]
	#[token(r"->*")]
	#[token(r"~!")]
	#[token(r"+")]
	#[token(r"-")]
	#[token(r"*")]
	#[token(r"/")]
	#[token(r"%")]
	#[token(r"^")]
	#[token(r"&")]
	#[token(r"|")]
	#[token(r"=")]
	#[token(r"+=")]
	#[token(r"-=")]
	#[token(r"*=")]
	#[token(r"/=")]
	#[token(r"%=")]
	#[token(r"^=")]
	#[token(r"&=")]
	#[token(r"|=")]
	#[token(r"==")]
	#[token(r"!=")]
	#[token(r"<")]
	#[token(r">")]
	#[token(r"<=")]
	#[token(r">=")]
	#[token(r"<=>")]
	#[token(r"&&")]
	#[token(r"||")]
	#[token(r"<<")]
	#[token(r">>")]
	#[token(r"<<=")]
	#[token(r">>=")]
	#[token(r"++")]
	#[token(r"--")]
	#[token(r",")]
	#[token(r"and")]
	#[token(r"or")]
	#[token(r"xor")]
	#[token(r"not")]
	#[token(r"bitand")]
	#[token(r"bitor")]
	#[token(r"compl")]
	#[token(r"and_eq")]
	#[token(r"or_eq")]
	#[token(r"xor_eq")]
	#[token(r"not_eq")]
	OperatorPunctuator,
	#[token(r"alignas")]
	#[token(r"alignof")]
	#[token(r"asm")]
	#[token(r"auto")]
	#[token(r"bool")]
	#[token(r"break")]
	#[token(r"case")]
	#[token(r"catch")]
	#[token(r"char")]
	#[token(r"char8_t")]
	#[token(r"char16_t")]
	#[token(r"char32_t")]
	#[token(r"class")]
	#[token(r"concept")]
	#[token(r"const")]
	#[token(r"consteval")]
	#[token(r"constexpr")]
	#[token(r"constinit")]
	#[token(r"const_cast")]
	#[token(r"continue")]
	#[token(r"co_await")]
	#[token(r"co_return")]
	#[token(r"co_yield")]
	#[token(r"decltype")]
	#[token(r"default")]
	#[token(r"delete")]
	#[token(r"do")]
	#[token(r"double")]
	#[token(r"dynamic_cast")]
	#[token(r"else")]
	#[token(r"enum")]
	#[token(r"explicit")]
	#[token(r"export")]
	#[token(r"extern")]
	#[token(r"false")]
	#[token(r"float")]
	#[token(r"for")]
	#[token(r"friend")]
	#[token(r"goto")]
	#[token(r"if")]
	#[token(r"inline")]
	#[token(r"int")]
	#[token(r"long")]
	#[token(r"mutable")]
	#[token(r"namespace")]
	#[token(r"new")]
	#[token(r"noexcept")]
	#[token(r"nullptr")]
	#[token(r"operator")]
	#[token(r"private")]
	#[token(r"protected")]
	#[token(r"public")]
	#[token(r"register")]
	#[token(r"reinterpret_cast")]
	#[token(r"requires")]
	#[token(r"return")]
	#[token(r"short")]
	#[token(r"signed")]
	#[token(r"sizeof")]
	#[token(r"static")]
	#[token(r"static_assert")]
	#[token(r"static_cast")]
	#[token(r"struct")]
	#[token(r"switch")]
	#[token(r"template")]
	#[token(r"this")]
	#[token(r"thread_local")]
	#[token(r"throw")]
	#[token(r"true")]
	#[token(r"try")]
	#[token(r"typedef")]
	#[token(r"typeid")]
	#[token(r"typename")]
	#[token(r"union")]
	#[token(r"unsigned")]
	#[token(r"using")]
	#[token(r"virtual")]
	#[token(r"void")]
	#[token(r"volatile")]
	#[token(r"wchar_t")]
	#[token(r"while")]
	Keyword,
	#[token("\n")]
	Newline,
	#[regex(r"[\t \x0B\x0C]")]
	Whitespace,
	#[regex(r"/\*[^\*/]*\*/")]
	Comment,
	/* Lmao, no repetition ranges ???*/
	// Normal strings
	#[regex(r#"(?:u8|u|U|L)?"(?:[\x20-\x7E&&[^"\\\n]]|\\[uU'"?\\abfnrtvx0-7])*""#)]
	StringLiteral,
	#[regex(r#"(?:u8|u|U|L)?"(?:[\x20-\x7E&&[^"\\\n]]|\\[uU'"?\\abfnrtvx0-7])*"[a-zA-Z_[^\x00-\x7F]][a-zA-Z0-9_[^\x00-\x7F]]*"#)]
	UdStringLiteral,
	#[regex(r#"(?:u8|u|U|L)?R"[\x20-\x7E&&[^ \(\)\\\n\x0B\t\x0C]]*\("#)]
	RawStringLiteral,
	#[regex(r#"(?:u8|u|U|L)?'(?:[\x20-\x7E&&[^'\\\n]]|\\[uU'"?\\abfnrtvx0-7])*'"#)]
	CharLiteral,
	#[regex(r#"(?:u8|u|U|L)?'(?:[\x20-\x7E&&[^'\\\n]]|\\[uU'"?\\abfnrtvx0-7])*'[a-zA-Z_[^\x00-\x7F]][a-zA-Z0-9_[^\x00-\x7F]]*"#)]
	UdCharLiteral,
	#[regex(r#"[\.]?[0-9](:?'?[0-9]|'[a-zA-Z_]|[eEpP][+-]|\\u[A-Fa-f0-9][A-Fa-f0-9][A-Fa-f0-9][A-Fa-f0-9]|\\U[A-Fa-f0-9][A-Fa-f0-9][A-Fa-f0-9][A-Fa-f0-9][A-Fa-f0-9][A-Fa-f0-9][A-Fa-f0-9][A-Fa-f0-9])*[\.]?"#)]
	PPNumber,
	#[error]
	Error,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PreToken {
	HeaderName(String),
	Ident(String),
	PreprocessingOperator(PreprocessingOperator),
	OperatorPunctuator(&'static str),
	Keyword(&'static str),
	Newline,
	Whitespace(String),
	Comment(String),
	StringLiteral(String),
	UdStringLiteral(String),
	RawStringLiteral(String),
	CharLiteral(String),
	UdCharLiteral(String),
	PPNumber(String),
	Unknown(String),
}

impl PreToken {
	pub fn new(tok: PreTokenLexer, content: String) -> PreToken {
		match tok {
			PreTokenLexer::Ident => PreToken::Ident(content),
			PreTokenLexer::PreprocessingOperatorHash => PreToken::PreprocessingOperator(PreprocessingOperator::Hash),
			PreTokenLexer::PreprocessingOperatorHashHash => PreToken::PreprocessingOperator(PreprocessingOperator::HashHash),
			PreTokenLexer::OperatorPunctuator => PreToken::OperatorPunctuator(match content.as_str() {
				r"{" => r"{",
				r"}" => r"}",
				r"[" => r"[",
				r"]" => r"]",
				r"(" => r"(",
				r")" => r")",
				r"<:" => r"<:",
				r":>" => r":>",
				r"<%" => r"<%",
				r"%>" => r"%>",
				r";" => r";",
				r":" => r":",
				r"..." => r"...",
				r"?" => r"?",
				r"::" => r"::",
				r"." => r".",
				r".*" => r".*",
				r"->" => r"->",
				r"->*" => r"->*",
				r"~!" => r"~!",
				r"+" => r"+",
				r"-" => r"-",
				r"*" => r"*",
				r"/" => r"/",
				r"%" => r"%",
				r"^" => r"^",
				r"&" => r"&",
				r"|" => r"|",
				r"=" => r"=",
				r"+=" => r"+=",
				r"-=" => r"-=",
				r"*=" => r"*=",
				r"/=" => r"/=",
				r"%=" => r"%=",
				r"^=" => r"^=",
				r"&=" => r"&=",
				r"|=" => r"|=",
				r"==" => r"==",
				r"!=" => r"!=",
				r"<" => r"<",
				r">" => r">",
				r"<=" => r"<=",
				r">=" => r">=",
				r"<=>" => r"<=>",
				r"&&" => r"&&",
				r"||" => r"||",
				r"<<" => r"<<",
				r">>" => r">>",
				r"<<=" => r"<<=",
				r">>=" => r">>=",
				r"++" => r"++",
				r"--" => r"--",
				r"," => r",",
				r"and" => r"and",
				r"or" => r"or",
				r"xor" => r"xor",
				r"not" => r"not",
				r"bitand" => r"bitand",
				r"bitor" => r"bitor",
				r"compl" => r"compl",
				r"and_eq" => r"and_eq",
				r"or_eq" => r"or_eq",
				r"xor_eq" => r"xor_eq",
				r"not_eq" => r"not_eq",
				_ => {panic!("How did you manage to get an operator not in my list")}
			}),
			PreTokenLexer::Keyword => PreToken::Keyword(match content.as_str() {
				r"alignas" => r"alignas",
				r"alignof" => r"alignof",
				r"asm" => r"asm",
				r"auto" => r"auto",
				r"bool" => r"bool",
				r"break" => r"break",
				r"case" => r"case",
				r"catch" => r"catch",
				r"char" => r"char",
				r"char8_t" => r"char8_t",
				r"char16_t" => r"char16_t",
				r"char32_t" => r"char32_t",
				r"class" => r"class",
				r"concept" => r"concept",
				r"const" => r"const",
				r"consteval" => r"consteval",
				r"constexpr" => r"constexpr",
				r"constinit" => r"constinit",
				r"const_cast" => r"const_cast",
				r"continue" => r"continue",
				r"co_await" => r"co_await",
				r"co_return" => r"co_return",
				r"co_yield" => r"co_yield",
				r"decltype" => r"decltype",
				r"default" => r"default",
				r"delete" => r"delete",
				r"do" => r"do",
				r"double" => r"double",
				r"dynamic_cast" => r"dynamic_cast",
				r"else" => r"else",
				r"enum" => r"enum",
				r"explicit" => r"explicit",
				r"export" => r"export",
				r"extern" => r"extern",
				r"false" => r"false",
				r"float" => r"float",
				r"for" => r"for",
				r"friend" => r"friend",
				r"goto" => r"goto",
				r"if" => r"if",
				r"inline" => r"inline",
				r"int" => r"int",
				r"long" => r"long",
				r"mutable" => r"mutable",
				r"namespace" => r"namespace",
				r"new" => r"new",
				r"noexcept" => r"noexcept",
				r"nullptr" => r"nullptr",
				r"operator" => r"operator",
				r"private" => r"private",
				r"protected" => r"protected",
				r"public" => r"public",
				r"register" => r"register",
				r"reinterpret_cast" => r"reinterpret_cast",
				r"requires" => r"requires",
				r"return" => r"return",
				r"short" => r"short",
				r"signed" => r"signed",
				r"sizeof" => r"sizeof",
				r"static" => r"static",
				r"static_assert" => r"static_assert",
				r"static_cast" => r"static_cast",
				r"struct" => r"struct",
				r"switch" => r"switch",
				r"template" => r"template",
				r"this" => r"this",
				r"thread_local" => r"thread_local",
				r"throw" => r"throw",
				r"true" => r"true",
				r"try" => r"try",
				r"typedef" => r"typedef",
				r"typeid" => r"typeid",
				r"typename" => r"typename",
				r"union" => r"union",
				r"unsigned" => r"unsigned",
				r"using" => r"using",
				r"virtual" => r"virtual",
				r"void" => r"void",
				r"volatile" => r"volatile",
				r"wchar_t" => r"wchar_t",
				r"while" => r"while",
				_ => {panic!("How did you manage to get a keyword not in my list");}
			}),
			PreTokenLexer::Newline => PreToken::Newline,
			PreTokenLexer::Whitespace => PreToken::Whitespace(content),
			PreTokenLexer::Comment => PreToken::Comment(content),
			PreTokenLexer::StringLiteral => PreToken::StringLiteral(content),
			PreTokenLexer::UdStringLiteral => PreToken::UdStringLiteral(content),
			PreTokenLexer::RawStringLiteral => PreToken::RawStringLiteral(content),
			PreTokenLexer::CharLiteral => PreToken::CharLiteral(content),
			PreTokenLexer::UdCharLiteral => PreToken::UdCharLiteral(content),
			PreTokenLexer::PPNumber => PreToken::PPNumber(content),
			PreTokenLexer::Error => PreToken::Unknown(content),
		}
	}
	pub fn to_str(&self) -> &str {
		return match self {
			Self::Unknown(string) |
			Self::PPNumber(string) |
			Self::HeaderName(string) |
			Self::Ident(string) |
			Self::Comment(string) |
			Self::StringLiteral(string) |
			Self::UdStringLiteral(string) |
			Self::RawStringLiteral(string) |
			Self::CharLiteral(string) |
			Self::UdCharLiteral(string) |
			Self::Whitespace(string) => string.as_str(),
			Self::OperatorPunctuator(string) |
			Self::Keyword(string) => string,
			Self::PreprocessingOperator(op) => op.to_str(),
			Self::Newline => "\n",
		}
	}
}