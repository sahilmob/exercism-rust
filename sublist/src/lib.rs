#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    let swapped = if first_list.len() >= second_list.len() {
        false
    } else {
        true
    };
    let list_1 = if swapped { second_list } else { first_list };
    let list_2 = if swapped { first_list } else { second_list };

    match list_1.len() {
        0 => match list_2.len() {
            0 => Comparison::Equal,
            _ => {
                if swapped {
                    Comparison::Sublist
                } else {
                    Comparison::Superlist
                }
            }
        },
        l => match list_2.len() {
            0 => {
                if swapped {
                    Comparison::Sublist
                } else {
                    Comparison::Superlist
                }
            }
            r => {
                let mut c1 = 0;
                let mut c2 = 0;
                let mut break_loop = false;

                let result = loop {
                    if c1 >= l {
                        break Comparison::Unequal;
                    }
                    let mut num1 = &list_1[c1];
                    let inner_result = loop {
                        if c2 >= r {
                            if l == c2 {
                                break_loop = true;
                                break Comparison::Equal;
                            } else if swapped {
                                break_loop = true;
                                break Comparison::Sublist;
                            } else {
                                break_loop = true;
                                break Comparison::Superlist;
                            }
                        } else {
                            let num2 = &list_2[c2];
                            if num1 != num2 {
                                if l - c1 >= list_2.len() && c1 > 0 && num1 != &list_1[c1 - 1] {
                                    c2 = 0;
                                }
                                break Comparison::Unequal;
                            } else {
                                c1 += 1;
                                if c1 < l {
                                    num1 = &list_1[c1];
                                }
                            }
                        }

                        c2 += 1;
                    };

                    if break_loop {
                        break inner_result;
                    }

                    c1 += 1;
                };
                result
            }
        },
    }
}
