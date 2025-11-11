pub fn map<Input, Output, F>(input: Vec<Input>, function: F) -> Vec<Output>
where
    F: FnMut(Input) -> Output,
{
    let mut result = Vec::with_capacity(input.len());
    let mut f = function;
    for value in input {
        result.push(f(value));
    }
    result
}
