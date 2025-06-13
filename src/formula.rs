// y = mx + c

pub(crate) struct Formula {
    pub m: i32,
    pub c: i32,
}

impl Formula {
    pub fn new(m: i32, c: i32) -> Self {
        Formula { m, c }
    }
}
impl std::fmt::Display for Formula {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "y = {}x + {}", self.m, self.c)
    }
}
