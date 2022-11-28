#[derive(Debug)]
pub struct LambdaGeneralError<T> {
    pub messages: Vec<T>,
}
