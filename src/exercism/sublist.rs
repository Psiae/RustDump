#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list == _second_list {
        return Comparison::Equal;
    }

    if is_sublist(_first_list, _second_list) {
        return Comparison::Sublist
    }

    if is_sublist(_second_list, _first_list) {
        return Comparison::Superlist
    }

    Comparison::Unequal
}

fn is_sublist<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    if a.len() > b.len() {
        return false
    }

    if a.is_empty() {
        return true
    }

    for (b_i, b_el) in
    // elements starting at b.len - a.len + 1 is skip-able
    b.iter().take(b.len() - a.len() + 1).enumerate() {
        // find the starting point of possible match
        if a[0] == *b_el &&
            // compare [a] to the slice
            a == &b[b_i..(b_i + a.len())] {
            return true;
        }
    }


    return false
}
