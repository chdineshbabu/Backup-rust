#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first: &[i32], second: &[i32]) -> Comparison {
    if first.is_empty() && second.is_empty() {
        return Comparison::Equal;
    }
    if first.is_empty() {
        return Comparison::Sublist;
    }
    if second.is_empty() {
        return Comparison::Superlist;
    }
    if first == second {
        return Comparison::Equal;
    }

    if first.len() < second.len() {
        if second.windows(first.len()).any(|w| w == first) {
            return Comparison::Sublist;
        }
        return Comparison::Unequal;
    }

    if first.len() > second.len() {
        if first.windows(second.len()).any(|w| w == second) {
            return Comparison::Superlist;
        }
        return Comparison::Unequal;
    }

    Comparison::Unequal
}
