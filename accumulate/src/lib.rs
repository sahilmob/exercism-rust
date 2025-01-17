/// What should the type of _function be?
pub fn map<T, U>(input: Vec<T>, mut function: impl FnMut(T) -> U) -> Vec<U> {
    let mut result: Vec<U> = Vec::new();

    for i in input {
        result.push(function(i));
    }

    result
}
