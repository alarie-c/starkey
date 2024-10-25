#[derive(Debug, PartialEq, Eq)]
pub enum Types {
    Int,
    Str,
    Bool,
    Float,
}

pub struct Value<T> {
    pub typ: Types,
    pub inner: T,
}

impl Value<i32> {
    pub fn from(inner: i32) -> Self {
        Self {
            typ: Types::Int,
            inner,
        }
    }
}

impl Value<f32> {
    pub fn from(inner: f32) -> Self {
        Self {
            typ: Types::Int,
            inner,
        }
    }
}

impl Value<bool> {
    pub fn from(inner: bool) -> Self {
        Self {
            typ: Types::Int,
            inner,
        }
    }
}

impl Value<String> {
    pub fn from_string(inner: String) -> Self {
        Self {
            typ: Types::Int,
            inner,
        }
    }

    pub fn from_str(inner: &str) -> Self {
        Self {
            typ: Types::Int,
            inner: String::from(inner),
        }
    }
}

impl<T> Value<T> {
    pub fn assert(&self, typ: Types) -> bool {
        self.typ == typ 
    }
}
