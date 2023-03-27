#[allow(dead_code)]
pub enum TokenType {
    LEFTPAREN,  // (
    RIGHTPAREN, // )
    PLUS,       // +
    MINUS,      // -
    STAR,       // *
    SLASH,      // /
    CARET,      // ^
    EQUAL,      // =
    INTEGER,    // 整数
    FLOAT,      // 浮点数
    STRING,     // 字符串
    LEFTBRACE,  // {
    RIGHTBRACE, // }
    IF,         // if
    ELSE,       // else
    PRINT,      // print
    RETURN,     // return
    EOF,        // 结束
}
