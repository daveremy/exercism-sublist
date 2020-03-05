use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    match first_list.len().cmp(&second_list.len()) {
        Ordering::Equal if first_list == second_list => Comparison::Equal,
        Ordering::Less if is_sublist(first_list, second_list) => Comparison::Sublist,
        Ordering::Greater if is_sublist(second_list, first_list) => Comparison::Superlist,
        _ => Comparison::Unequal,
    }
}

fn is_sublist<T: PartialEq>(shorter: &[T], longer: &[T]) -> bool {
    shorter.len() == 0 || longer.windows(shorter.len()).any(|w| shorter == w)
}
