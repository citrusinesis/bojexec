use os_info::Type;

pub trait UserAgent {
    fn get_user_agent(&self) -> String;
}

impl UserAgent for Type {
    fn get_user_agent(&self) -> String {
        match self {
            Type::Windows => "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/42.0.2311.135 Safari/537.36 Edge/12.246".parse().unwrap(),
            Type::Macos => "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_11_2) AppleWebKit/601.3.9 (KHTML, like Gecko) Version/9.0.2 Safari/601.3.9".parse().unwrap(),
            _ => "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:15.0) Gecko/20100101 Firefox/15.0.1".parse().unwrap(),
        }
    }
}
