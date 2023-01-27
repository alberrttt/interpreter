use std::{collections::HashMap, sync::Mutex};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct InternedString(pub usize);
impl From<InternedString> for String {
    fn from(value: InternedString) -> Self {
        let interner = STRING_INTERNER.lock().expect("already?");
        interner.get(value).to_owned()
    }
}

impl From<&str> for InternedString {
    fn from(value: &str) -> Self {
        let mut interner = STRING_INTERNER.lock().expect("already?");
        interner.get_or_intern(value)
    }
}
#[derive(Debug, Clone, Default)]
pub struct StringInterner {
    strings: HashMap<String, usize>,
    vec: Vec<String>,
}
use lazy_static::lazy_static;
use once_cell::sync::Lazy;
pub static STRING_INTERNER: Lazy<Mutex<StringInterner>> = Lazy::new(|| {
    Mutex::new(StringInterner {
        strings: Default::default(),
        vec: Default::default(),
    })
});
impl StringInterner {
    pub fn get_or_intern(&mut self, s: &str) -> InternedString {
        let strings = &mut self.strings;
        let vec = &mut self.vec;
        if let Some(idx) = strings.get(s) {
            InternedString(*idx)
        } else {
            let idx = vec.len();
            vec.push(s.to_owned());
            strings.insert(s.to_owned(), idx);
            InternedString(idx)
        }
    }

    pub fn get(&self, idx: InternedString) -> &str {
        &self.vec[idx.0]
    }

    pub fn get_ref(&self, idx: InternedString) -> Option<&String> {
        self.vec.get(idx.0)
    }
}
