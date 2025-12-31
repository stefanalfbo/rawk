#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub items: Vec<Item>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Statement;

#[derive(Debug, Clone, PartialEq)]
pub struct Action {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    Action,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skeleton() {
        assert!(true);
    }
}
