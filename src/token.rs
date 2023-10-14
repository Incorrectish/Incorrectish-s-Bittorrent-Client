pub enum Token {
    DICTIONARY,
    INTEGER,
    LIST,
    STRING(u32),
    END,
}
