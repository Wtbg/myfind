#[derive(Debug)] 
pub struct MyError {
    pub kind: MyErrorKind,
    pub message: String,
    pub location: String,
}

#[derive(Debug)]
pub enum MyErrorKind {
    Io,
    Parse,
    NotFound,
    #[allow(dead_code)]
    Unknown,
}
impl MyError {
    pub fn new_with_description(kind: MyErrorKind, message: String, location:String) -> Self {
        Self {
            kind,
            message,
            location,
        }
    }
}


impl From<regex::Error> for MyError {
    fn from(error: regex::Error) -> Self {
        Self {
            location: "regex".to_string(), // "regex" is the location of the error
            kind: MyErrorKind::Parse,
            message: error.to_string(),
        }
    }
}
//print error
impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.kind {
            MyErrorKind::Io => write!(f, "IO Error: {}", self.message),
            MyErrorKind::Parse => write!(f, "Parse Error: {}", self.message),
            MyErrorKind::NotFound => write!(f, "Not Found Error: {}", self.message),
            MyErrorKind::Unknown => write!(f, "Unknown Error: {}", self.message),
        }
    }
}

impl From<std::io::Error> for MyError {
    fn from(error: std::io::Error) -> Self {
        Self {
            location: "io".to_string(), // "io" is the location of the error
            kind: MyErrorKind::Io,
            message: error.to_string(),
        }
    }
}