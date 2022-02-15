use std::str::FromStr;

pub enum TokenCategory {
    Keyword,
    Identifier,
    Number,
    Literal,
    Separator,
    Operation,
}

#[derive(Debug)]
pub enum TokenType {
    // Keywords
    Func,
    If,
    While,
    For,
    Var,
    // Operation
    Plus,
    Minus,
    Divide,
    Multiply,
    Mod,
    Power,
    PlusPlus,
    MinusMinus,
    Equals,
    EqualsEquals,
    LessThan,
    MoreThan,
    LessThanEquals,
    MoreThanEquals,
    PlusEquals,
    MinusEquals,
    DivideEquals,
    MultiplyEquals,
    // Separators
    Comma,
    Colon,
    OpeningBracket,
    ClosingBracket,
    OpeningSquareBracket,
    ClosingSquareBracket,
    Tab,
    // Identifier
    Identifier(String),
    // Literal
    IntNumber(i32),
    DecimalNumber(f32),
    Char(String),
    String(String),
}

pub struct Token {
    pub token_type: TokenType,
    pub token_category: TokenCategory,
}

pub struct Context {
    pub file_name: String,
    pub file_length: usize,
    pub file_pointer: usize,
    pub file_end: usize,
    pub buffer: String,
    pub tokens: Vec<Token>,
}

pub fn scan_tokens(context: &mut Context) {
    while context.file_pointer < context.file_end {
        scan_token(context);
    }
}

fn scan_token(context: &mut Context) {
    let character: char = get_char(context, context.file_pointer).unwrap();

    match character {
        // Keyword / Indentifier
        'a'..='z' => scan_word(context),
        '_' => scan_word(context),
        // Separator
        ',' => add_token(context, TokenType::Comma, TokenCategory::Separator, 1),
        ':' => add_token(context, TokenType::Colon, TokenCategory::Separator, 1),
        '(' => add_token(
            context,
            TokenType::OpeningBracket,
            TokenCategory::Separator,
            1,
        ),
        ')' => add_token(
            context,
            TokenType::ClosingBracket,
            TokenCategory::Separator,
            1,
        ),
        '[' => add_token(
            context,
            TokenType::OpeningSquareBracket,
            TokenCategory::Separator,
            1,
        ),
        ']' => add_token(
            context,
            TokenType::ClosingSquareBracket,
            TokenCategory::Separator,
            1,
        ),
        '"' => scan_word_between(context, '"'),
        '\'' => scan_word_between(context, '\''),
        // Operation
        '+' => {
            if get_char(context, context.file_pointer + 1).unwrap() == '+' {
                add_token(context, TokenType::PlusPlus, TokenCategory::Operation, 2)
            } else if get_char(context, context.file_pointer + 1).unwrap() == '=' {
                add_token(context, TokenType::PlusEquals, TokenCategory::Operation, 2)
            } else {
                add_token(context, TokenType::Plus, TokenCategory::Operation, 1)
            }
        }
        '-' => {
            if get_char(context, context.file_pointer + 1).unwrap() == '-' {
                add_token(context, TokenType::MinusMinus, TokenCategory::Operation, 2)
            } else if get_char(context, context.file_pointer + 1).unwrap() == '=' {
                add_token(context, TokenType::MinusEquals, TokenCategory::Operation, 2)
            } else if matches!(
                get_char(context, context.file_pointer + 1).unwrap(),
                '0'..='9'
            ) {
                scan_number(context)
            } else {
                add_token(context, TokenType::Minus, TokenCategory::Operation, 1)
            }
        }
        '*' => {
            if get_char(context, context.file_pointer + 1).unwrap() == '=' {
                add_token(
                    context,
                    TokenType::MultiplyEquals,
                    TokenCategory::Operation,
                    2,
                )
            } else {
                add_token(context, TokenType::Multiply, TokenCategory::Operation, 1)
            }
        }
        '/' => {
            if get_char(context, context.file_pointer + 1).unwrap() == '=' {
                add_token(
                    context,
                    TokenType::DivideEquals,
                    TokenCategory::Operation,
                    2,
                )
            } else {
                add_token(context, TokenType::Divide, TokenCategory::Operation, 1)
            }
        }
        '\t' => add_token(context, TokenType::Tab, TokenCategory::Separator, 1),
        '%' => add_token(context, TokenType::Mod, TokenCategory::Operation, 1),
        '*' => add_token(context, TokenType::Power, TokenCategory::Operation, 1),
        '=' => {
            if get_char(context, context.file_pointer + 1).unwrap() == '=' {
                add_token(
                    context,
                    TokenType::EqualsEquals,
                    TokenCategory::Operation,
                    2,
                )
            } else {
                add_token(context, TokenType::Equals, TokenCategory::Operation, 1)
            }
        }
        '>' => {
            if get_char(context, context.file_pointer + 1).unwrap() == '=' {
                add_token(
                    context,
                    TokenType::MoreThanEquals,
                    TokenCategory::Operation,
                    2,
                )
            } else {
                add_token(context, TokenType::MoreThan, TokenCategory::Operation, 1)
            }
        }
        '<' => {
            if get_char(context, context.file_pointer + 1).unwrap() == '=' {
                add_token(
                    context,
                    TokenType::LessThanEquals,
                    TokenCategory::Operation,
                    2,
                )
            } else {
                add_token(context, TokenType::LessThan, TokenCategory::Operation, 1)
            }
        }
        // Anything else
        _ => context.file_pointer += 1,
    }
}

fn scan_word(context: &mut Context) {
    let mut current_pointer: usize = context.file_pointer + 1;
    let start_pointer = context.file_pointer;

    if current_pointer >= context.file_end {
        current_pointer = start_pointer;
    }

    while current_pointer < context.file_end
        && matches!(get_char(context, current_pointer).unwrap(), 'a'..='z' | '_')
    {
        current_pointer += 1;
    }

    let word = context.buffer.get(start_pointer..current_pointer).unwrap();
    let length: usize = current_pointer - start_pointer;

    match word {
        // Keywords
        "func" => add_token(context, TokenType::Func, TokenCategory::Keyword, length),
        "if" => add_token(context, TokenType::If, TokenCategory::Keyword, length),
        "while" => add_token(context, TokenType::While, TokenCategory::Keyword, length),
        "for" => add_token(context, TokenType::For, TokenCategory::Keyword, length),
        "var" => add_token(context, TokenType::Var, TokenCategory::Keyword, length),
        _ => add_token(
            context,
            TokenType::Identifier(word.to_string()),
            TokenCategory::Identifier,
            length,
        ),
    }
}

fn scan_number(context: &mut Context) {
    let mut current_pointer: usize = context.file_pointer + 1;
    let start_pointer = context.file_pointer;

    if current_pointer >= context.file_end {
        current_pointer = start_pointer;
    }

    let mut is_decimal: bool = false;

    let mut current_char: char = get_char(context, current_pointer).unwrap();
    while current_pointer < context.file_end && matches!(current_char, '0'..='9' | '.') {
        if current_char == '.' {
            is_decimal = true;
        }
        current_pointer += 1;
        current_char = get_char(context, current_pointer).unwrap();
    }

    let number = context.buffer.get(start_pointer..current_pointer).unwrap();
    let length: usize = current_pointer - start_pointer;

    if is_decimal {
        add_token(
            context,
            TokenType::DecimalNumber(f32::from_str(number).unwrap()),
            TokenCategory::Number,
            length,
        )
    } else {
        add_token(
            context,
            TokenType::IntNumber(i32::from_str(number).unwrap()),
            TokenCategory::Number,
            length,
        )
    }
}

fn scan_word_between(context: &mut Context, character: char) {
    let mut current_pointer: usize = context.file_pointer + 1;
    let start_pointer = context.file_pointer + 1;

    if current_pointer >= context.file_end {
        panic!("No closing {} found!", character);
    }

    let mut current_char: char = get_char(context, current_pointer).unwrap();
    while current_char != character {
        current_pointer += 1;
        current_char = get_char(context, current_pointer).unwrap();

        if current_pointer == context.file_end {
            if current_char != character {
                panic!("No closing {} found!", character);
            }
            break;
        }
    }

    let word = context.buffer.get(start_pointer..current_pointer).unwrap();

    let length: usize = current_pointer - start_pointer + 2;

    match character {
        '"' => add_token(
            context,
            TokenType::String(word.to_string()),
            TokenCategory::Literal,
            length,
        ),
        '\'' => add_token(
            context,
            TokenType::Char(word.to_string()),
            TokenCategory::Literal,
            length,
        ),
        _ => context.file_pointer += 1,
    }
}

fn add_token(
    context: &mut Context,
    token_type: TokenType,
    token_category: TokenCategory,
    token_length: usize,
) {
    let token = Token {
        token_type,
        token_category,
    };
    context.file_pointer += token_length;
    context.tokens.push(token);
}

fn get_char(context: &mut Context, index: usize) -> Option<char> {
    if index > context.file_end {
        return None;
    }
    context.buffer.chars().nth(index)
}
