fn main() {
}

#[cfg(test)]
mod tests {
    use url::Url;

    #[test]
    fn url_shorten_path() {
        let url = Url::parse("https://example.com/path1/path2/path3").unwrap();
        assert_eq!(url.to_string(), "https://example.com/path1/path2/path3");

        let result = Url::options()
            .base_url(Some(&url))
            .parse("path4/path5/path6")
            .unwrap();
        assert_eq!(result.to_string(), "https://example.com/path1/path2/path3/path4/path5/path6");
    }

    #[test]
    fn url_shorten_path_leading_slash() {
        let url = Url::parse("https://example.com/path1/path2/path3").unwrap();
        assert_eq!(url.to_string(), "https://example.com/path1/path2/path3");

        let result = Url::options()
            .base_url(Some(&url))
            .parse("/path4/path5/path6")
            .unwrap();
        assert_eq!(result.to_string(), "https://example.com/path1/path2/path3/path4/path5/path6");
    }
}
