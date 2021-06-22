use serde::{Deserialize, Serialize};
pub type Id = u64;

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd)]
pub struct IDGenerator {
    curr: Id,
}

impl IDGenerator {
    pub fn new() -> Self {
        Self {
            curr: Default::default(),
        }
    }

    pub fn next_id(&mut self) -> Id {
        self.curr += 1;
        self.curr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_id_is_1() {
        let mut idg = IDGenerator::new();
        assert_eq!(idg.next_id(), 1);
    }
}
