
pub struct NameIndex<'a>(Vec<(&'a str, usize)>);
impl <'a> NameIndex<'a> {
    pub fn new(v: Vec<(&'a str, usize)>) -> Self {
        Self(v)
    }
    pub fn find(&mut self, s: &'a str) -> usize {
        match self.0.binary_search_by_key(&s, |(z, _)| z) {
            Ok(i) => self.0[i].1,
            Err(ins) => {
                let i = self.0.len();
                self.0.insert(ins, (s, i));
                i
            }
        }
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}