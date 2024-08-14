use crate::whisper_core::whisper_command::WhisperCommand;
use crate::whisper_core::whisper_argument::{
    WhisperArgument, 
    WhisperAction
};

pub fn setup_example() -> [WhisperCommand<example_completion>; 1] {
    return [
        WhisperCommand {
            name: "test".to_string(),
            arguments: [
                WhisperArgument {
                  short: "t".to_string(),
                  long: "test".to_string(),
                  value_name: "test_input".to_string(),
                  default_value: "test_value".to_string(),
                  action: example_completion {},
                  help: "testing owo".to_string()
                }
            ]
        }
    ];
}

pub struct example_completion {

}

impl WhisperAction for example_completion {

   fn action(&self) {
       println!("Gotcha reach there!");
   } 

}
