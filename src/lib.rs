pub trait Create<T> {
    fn create(&self, chunk_id: u16, value: T) -> &Sdxf;
}

pub trait Write<'a, T> {
    fn create_structured(chunk_id: u16) -> &'a T;
    fn leave() -> &'a T;
}

pub trait Read<'a, T> {
    fn enter() -> &'a T;
    fn next() -> &'a T;
    fn extract() -> Value<'a>;
    fn leave() -> &'a T;
}

pub struct Sdxf <'a> {
    pub options: SdxfOptions,
    pub chunk: Option<Chunk<'a>>,
    current_chunk: Option<Chunk<'a>>,
}

pub struct SdxfOptions {
    pub encryption: Option<Encryption>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Chunk <'a> {
    pub id: u16,
    pub value: Option<Value<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value <'a> {
    Structure(Vec<Chunk<'a>>),
    BitString(Vec<u8>),
    Numeric(i64),
    Character(&'a str),
    Float(f64),
    Utf8(Vec<u8>),
}

pub struct Encryption {
    pub encrypt: fn(Vec<u8>, password: &str) -> Result<Vec<u8>, SdxfError>,
    pub decrypt: fn(Vec<u8>, password: &str) -> Result<Vec<u8>, SdxfError>,
}

impl<'a> Sdxf<'a> {
    pub fn new() -> Sdxf<'a> {
        let options = SdxfOptions::new();
        Sdxf {
            options,
            chunk: None,
            current_chunk: None,
        }
    }

    pub fn create_structured(&self, chunk_id: u16) -> &Sdxf {
        self
    }

    pub fn enter(&self) -> &Sdxf<'a> {
        self
    }

    pub fn leave(&self) -> &Sdxf {
        self
    }

    pub fn extract(&self) -> Result<Value<'a>, SdxfError> {
        match self.current_chunk {
            Some(ref chunk) => {
                match chunk.value {
                    Some(ref value) => {
                        match value {
                            &Value::Structure(_) => return Err(SdxfError { kind: SdxfErrorKind::CannotExtractFromStructuredChunk }),
                            _ => return Ok(value.clone()),
                        }
                    }
                    None => return Err(SdxfError { kind: SdxfErrorKind::InconsistentChunk { chunk_id: chunk.id }, })
                }
            }
            None => return Err(SdxfError { kind: SdxfErrorKind::NoCurrentChunk })
        }
    }
}

impl<'a> Create<&'a str> for Sdxf<'a> {
    fn create(&self, chunk_id: u16, value: &str) -> &Sdxf {
        self
    }
}

impl<'a> Create<i64> for Sdxf<'a> {
    fn create(&self, chunk_id: u16, value: i64) -> &Sdxf {
        self
    }
}

impl<'a> Create<f64> for Sdxf<'a> {
    fn create(&self, chunk_id: u16, value: f64) -> &Sdxf {
        self
    }
}

impl SdxfOptions {
    pub fn new() -> SdxfOptions {
        SdxfOptions {
            encryption: None,
        }

    }
}
impl<'a> Chunk<'a> {
    fn create(id: u16) -> Chunk<'a> {
        Chunk {
            id,
            value: None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum SdxfErrorKind {
    InconsistentChunk { chunk_id: u16 },
    MaximumLevelExceeded { max_level: u32 },
    NoCurrentChunk,
    CannotExtractFromStructuredChunk,
    EncryptionFailed,
    DecryptionFailed,
}

#[derive(Debug, Clone)]
pub struct SdxfError {
    pub kind: SdxfErrorKind,
}

impl SdxfError {
    pub fn new(kind: SdxfErrorKind) -> SdxfError {
        SdxfError {
            kind: kind,
        }
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
