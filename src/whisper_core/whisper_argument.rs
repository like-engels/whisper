pub struct WhisperArgument<T: WhisperAction> {
    pub short: String,
    pub long: String,
    pub value_name: String,
    pub default_value: String,
    pub action: T,
    pub help: String
}

pub trait WhisperAction {

    fn action(&self);

}
