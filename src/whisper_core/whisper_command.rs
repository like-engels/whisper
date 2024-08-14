use super::whisper_argument::{
    WhisperArgument,
    WhisperAction
};

pub struct WhisperCommand<T: WhisperAction> {
    pub name: String,
    pub arguments: [WhisperArgument<T>; 1]
}
