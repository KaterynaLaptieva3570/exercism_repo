#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    use Comparison::*;

    // check if `small` is contained within `big`
    fn is_sublist(small: &[i32], big: &[i32]) -> bool {
        if small.is_empty() {
            return true;
        }
        if small.len() > big.len() {
            return false;
        }
        big.windows(small.len()).any(|window| window == small)
    }

    if first_list == second_list {
        Equal
    } else if is_sublist(first_list, second_list) {
        Sublist
    } else if is_sublist(second_list, first_list) {
        Superlist
    } else {
        Unequal
    }
}
