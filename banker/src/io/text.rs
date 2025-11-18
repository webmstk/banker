use thiserror::Error;

use std::io::{self, BufRead, Write};
use std::string::FromUtf8Error;

/// Ошибка, описывающая проблемы, которые могут возникнуть при парсинге
/// текстовых файлов.
#[derive(Debug, Error)]
pub(crate) enum ParseError {
    /// Ошибка, встречающаяся при парсинге полей, которые по спецификации обязаны быть обёрнуты
    /// двойными кавычками. Такое поле обязано начинаться с двойной кавычки.
    #[error("must start with qoute")]
    MustStartWithQuote,
    /// Ошибка, встречающаяся при парсинге полей, которые по спецификации обязаны быть обёрнуты
    /// двойными кавычками. Такое поле обязано заканчиваться двойной кавычкой.
    #[error("must end with qoute")]
    MustEndWithQuote,
    /// Ошибка может возникнуть при попытке прочитать поле после того, как всё уже было прочитано.
    #[error("unexpected end of file")]
    UnexpectedEOF,
    /// Обёртка надо обычной ошибкой [std::io::Error]
    #[error("{0}")]
    IOError(#[from] io::Error),
    /// Ошибка возникает, когда содержимое не является корректной строкой в кодировке UTF-8.
    #[error("{0}")]
    EncodingError(#[from] FromUtf8Error),
}

/// Читает строку, обёрнутую в двойные кавычки. Строка обязана начинаться и заканчиваться
/// двойной кавычкой. Строка может содержать в себе кавычки, в таком случае ожидается,
/// что они экранированы второй кавычкой `""`. Потребляет следующий после строки символ.
pub(crate) fn read_quoted(reader: impl BufRead) -> Result<String, ParseError> {
    let quote_char: char = '"';

    let mut bytes = reader.bytes();

    // Проверяем наличие первой кавычки, она должна быть, согласно
    // спекам текстовых форматов
    if let Some(next_byte) = bytes.next().transpose()? {
        if next_byte != quote_char as u8 {
            return Err(ParseError::MustStartWithQuote);
        }
    } else {
        return Err(ParseError::UnexpectedEOF);
    }

    let mut value = Vec::new();

    // Заранее считываем первой символ из строки, он нам понадобится далее в цикле.
    let mut prev_char: u8 = match bytes.next().transpose()? {
        Some(ch) => ch,
        None => return Err(ParseError::UnexpectedEOF),
    };

    let mut qoute_mode = false;

    // В цикле анализируем текущий и предыдущий символы, это нужно для обработки
    // экранированных кавычек.
    while let Some(next_byte) = bytes.next().transpose()? {
        // если в предыдущей итерации была кавычка
        if qoute_mode {
            // то в этой итерации либо ожидаем вторую кавычку (считаем, что первая
            // была экранирующая)
            if next_byte == quote_char as u8 {
                qoute_mode = false;
                continue;
            // либо любой другой символ, тогда считаем, что прошлая кавычка была закрывающей,
            // и с парсингом мы завершили
            } else {
                break;
            }
        }

        if next_byte == quote_char as u8 {
            qoute_mode = true;
        }

        value.push(prev_char);
        prev_char = next_byte;
    }

    // Проверяем, что последний символ был кавычкой.
    if prev_char != quote_char as u8 {
        return Err(ParseError::MustEndWithQuote);
    }

    // Если кавычка была закрывающей, то `quote_mode` должен быть `true`.
    // В противном случае это значит, что последняя кавычка - экранированная,
    // а закрывающей попросту нет.
    if !qoute_mode && prev_char == quote_char as u8 {
        return Err(ParseError::MustEndWithQuote);
    }

    Ok(String::from_utf8(value)?)
}

/// Читает до указанного символа либо до конца строки или (EOF). Указанный символ
/// или конец строки потребляется.
pub(crate) fn read_until(reader: impl BufRead, stop: u8) -> Result<String, ParseError> {
    let newline_char: char = '\n';

    let mut value = Vec::new();
    let mut bytes = reader.bytes();

    while let Some(next_byte) = bytes.next().transpose()? {
        if next_byte == stop || next_byte == newline_char as u8 {
            break;
        }
        value.push(next_byte);
    }

    if value.is_empty() {
        return Err(ParseError::UnexpectedEOF);
    }

    Ok(String::from_utf8(value)?)
}

/// Записывает в буффер строку, обёрнутую в двойные кавычки. Каждая кавычка внтури
/// экранируется дополнительной кавычкой.
pub(crate) fn write_quoted(mut writer: impl Write, value: &String) -> io::Result<()> {
    write!(writer, "\"")?;

    value.chars().try_for_each(|c| {
        if c == b'"' as char {
            write!(writer, "\"")?;
        }
        write!(writer, "{}", c)
    })?;

    write!(writer, "\"")?;

    Ok(())
}

#[cfg(test)]
mod tests;
