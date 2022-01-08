pub trait BaseUrl {
    fn base_url(&self) -> &str;
}

impl BaseUrl for &str {
    fn base_url(&self) -> &str {
        let end = self
            .find('#')
            .or_else(|| self.find('?'))
            .or_else(|| Some(self.len()))
            .unwrap();
        &self[0..end]
    }
}

impl BaseUrl for String {
    fn base_url(&self) -> &str {
        let end = self
            .find('#')
            .or_else(|| self.find('?'))
            .or_else(|| Some(self.len()))
            .unwrap();
        &self[0..end]
    }
}
