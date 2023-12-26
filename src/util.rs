
/// This is a storage for names associated with their indices.
/// The index is based on current length of the Vec.
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

pub struct Grid {
    pub data: Box<[u8]>,
    pub offset: usize,
}
impl Grid {
    pub fn from_data(s: &str) -> Self {
        let mut lines = s.lines().peekable();
        let line_len = lines.peek().map_or(0, |l| l.len());
        Self {
            data: lines.flat_map(str::as_bytes).copied().collect::<Box<_>>(),
            offset: line_len,
        }
    }
    pub fn from_digits(s: &str) -> Self {
        let mut lines = s.lines().peekable();
        let line_len = lines.peek().map_or(0, |l| l.len());
        Self {
            data: lines.flat_map(str::as_bytes).map(|&c| c - b'0').collect::<Box<_>>(),
            offset: line_len,
        }
    }
    pub const fn next_pos(&self, p: usize, dir: u8) -> Option<usize> {
        // 0 = up, 1 = right, 2 = down, 3 = left
        Some(match dir {
            0 if p >= self.offset => p - self.offset,
            1 if (p + 1) % self.offset != 0 => p + 1,
            2 if p < self.data.len() - self.offset => p + self.offset,
            3 if p % self.offset != 0 => p - 1,
            _ => { return None }
        })
    }
}