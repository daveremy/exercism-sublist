#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    if first_list.len() == second_list.len() && first_list == second_list {
        Comparison::Equal
    } else if first_list.len() < second_list.len() && is_subset(first_list, second_list) {
        Comparison::Sublist
    } else if first_list.len() > second_list.len() && is_subset(second_list, first_list) {
        Comparison::Superlist
    } else {
        Comparison::Unequal
    }
}

fn is_subset<T: PartialEq>(shorter: &[T], longer: &[T]) -> bool {
    for i in 0..=longer.len() - shorter.len() {
        if shorter == &longer[i..i + shorter.len()] {
            return true;
        }
    }
    false
}
