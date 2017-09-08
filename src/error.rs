error_chain! {
    foreign_links {
        Reqwest(::reqwest::Error);
        Url(::url::ParseError);
    }
}
