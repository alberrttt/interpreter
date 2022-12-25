use std::collections::HashMap;

pub struct InternerIndex(usize);

#[derive(Debug, Clone)]
pub struct StringInterner {
    strings: HashMap<String, usize>,
    vec: Vec<String>,
}

impl StringInterner {
    pub fn new() -> Self {
        StringInterner {
            strings: (HashMap::new()),
            vec: (Vec::new()),
        }
    }

    pub fn get_or_intern(&mut self, s: &str) -> InternerIndex {
        let strings = &mut self.strings;
        let vec = &mut self.vec;
        if let Some(idx) = strings.get(s) {
            InternerIndex(*idx)
        } else {
            let idx = vec.len();
            vec.push(s.to_owned());
            strings.insert(s.to_owned(), idx);
            InternerIndex(idx)
        }
    }

    pub fn get(&self, idx: InternerIndex) -> &str {
        &self.vec[idx.0]
    }
}
