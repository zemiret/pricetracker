pub struct PageInfo {
    pub price: f32,
    pub title: String,
    // TODO: Availability, sizes
}

impl std::fmt::Debug for PageInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PageInfo")
            .field("price", &self.price)
            .field("title", &self.title)
            .finish()
    }
}

#[derive(Debug)]
pub enum Error{
    ExtractPageInfoError(String)
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ExtractPageInfoError(inner) => write!(f, "Error extracting page info: {}", inner)
        }
    }
}
