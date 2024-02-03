use super::model::{Model, TypeAlias};
use anyhow::Result;

/// Utlity function to provide a model of given size with no cycles for testing purposes.
///
/// Every node in any level is connected to three nodes.
///
/// Traversing the model in BFS from the "0" node gives nodes in order (0, 1, 2, 3, ...).
///
/// For example, for size = 10:
/// ```text
///               (0)
///       __________________
///      |         |       |
///     (1)       (2)     (3)
///   ------    ------
///  |  |  |   |  |  |
/// (4)(5)(6) (7)(8)(9)
/// ```
pub fn model_mock(len: usize, model_id: &str) -> Result<Model> {
    let mut model = Model::new("test", model_id);

    for i in 0..len {
        model.add_table(&format!("{i}"), i);
    }

    for i in 0..len {
        for j in 0..3 {
            model
                .add_column(i, &format!("{j}"), TypeAlias::Integer, "int4", false)
                .expect("Should exist");
        }
    }

    let mut o = 1;
    for i in 0..len {
        let s = i + o;
        for j in s..s + 3 {
            if j >= len {
                break;
            }
            model.add_relation(i, 0, j, 1).expect("Should exist");
        }
        o += 2;
    }

    Ok(model)
}

/// Utlity function to provide a cyclic model for testing purposes.
///
/// The (undirected, unweighted) relations are:
/// ```text
///
///  (0)_________
///   |          |
///  (1)________(3)
///   |          |
///  (2)_________|
///
/// ```
///
pub fn model_with_cycles() -> Result<Model> {
    let mut model = Model::new("test", "test");
    for i in 0..4 {
        model.add_table(&format!("{i}"), i);
    }

    for i in 0..4 {
        for j in 0..3 {
            model
                .add_column(i, &format!("{j}"), TypeAlias::Integer, "int4", false)
                .expect("Should exist");
        }
    }
    model.add_relation(0, 0, 1, 1)?;
    model.add_relation(0, 0, 3, 1)?;
    model.add_relation(1, 0, 2, 1)?;
    model.add_relation(1, 0, 3, 1)?;
    model.add_relation(2, 0, 3, 1)?;
    Ok(model)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(1, 0; "One single node")]
    #[test_case(2, 1; "Two nodes")]
    #[test_case(4, 3; "Four nodes")]
    #[test_case(10, 9; "Ten nodes")]
    #[test_case(1000, 999; "A bunch of nodes")]
    fn model_mock_of_size_x_should_have_x_minus_one_relations(model_size: usize, expected: usize) {
        let model = model_mock(model_size, "test");
        match model {
            Ok(model) => {
                assert_eq!(model.relations.len(), expected);
                ()
            }
            Err(e) => panic!("{e}"),
        }
    }
}
