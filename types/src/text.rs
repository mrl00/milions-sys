use unicode_normalization::UnicodeNormalization;

/// Removes accents from a string by decomposing Unicode characters and stripping
/// combining marks (e.g., "São Paulo" -> "Sao Paulo").
pub fn remove_accents(s: &str) -> String {
    s.nfd()
        .filter(|c| !unicode_normalization::char::is_combining_mark(*c))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn removes_accents_from_portuguese() {
        assert_eq!(remove_accents("São Paulo"), "Sao Paulo");
        assert_eq!(remove_accents("João"), "Joao");
        assert_eq!(remove_accents("Ação"), "Acao");
        assert_eq!(remove_accents("Café"), "Cafe");
        assert_eq!(remove_accents("Praça da Sé"), "Praca da Se");
    }

    #[test]
    fn leaves_ascii_unchanged() {
        assert_eq!(remove_accents("Hello World"), "Hello World");
        assert_eq!(remove_accents("123"), "123");
    }

    #[test]
    fn handles_empty_string() {
        assert_eq!(remove_accents(""), "");
    }
}
