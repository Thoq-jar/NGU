#[derive(Debug, Clone, PartialEq)]
pub struct Options {
    pub show_hidden: bool,
    pub show_almost_all: bool,
    pub long_format: bool,
    pub human_readable: bool,
    pub sort_time: bool,
    pub reverse_sort: bool,
    pub recursive: bool,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            show_hidden: false,
            show_almost_all: false,
            long_format: false,
            human_readable: false,
            sort_time: false,
            reverse_sort: false,
            recursive: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_options() {
        let opts = Options::default();
        assert!(!opts.show_hidden);
        assert!(!opts.long_format);
        assert!(!opts.human_readable);
    }
}
