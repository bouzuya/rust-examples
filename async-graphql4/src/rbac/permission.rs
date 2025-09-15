#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Permission {
    Parent1OutputChild1,
    Parent2OutputChild2,
    QueryRootParent1,
    QueryRootParent2,
}

impl std::fmt::Display for Permission {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Permission::Parent1OutputChild1 => "Parent1Output::child1",
            Permission::Parent2OutputChild2 => "Parent2Output::child2",
            Permission::QueryRootParent1 => "Query::parent1",
            Permission::QueryRootParent2 => "Query::parent2",
        }
        .fmt(f)
    }
}

impl std::str::FromStr for Permission {
    // FIXME
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Parent1Output::child1" => Ok(Permission::Parent1OutputChild1),
            "Parent2Output::child2" => Ok(Permission::Parent2OutputChild2),
            "QueryRoot::parent1" => Ok(Permission::QueryRootParent1),
            "QueryRoot::parent2" => Ok(Permission::QueryRootParent2),
            _ => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_impls() {
        fn assert_impl<T: Send + Sync>() {}

        assert_impl::<Permission>();
    }
}
