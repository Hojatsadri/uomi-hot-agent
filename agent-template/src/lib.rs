use serde::{Deserialize, Serialize};
use utils::log;

mod utils;

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    role: String,
    content: String,
}

#[no_mangle]
pub extern "C" fn run() {
        log("Hojjatsadri's agent has started!");
        //get the user input
        // get the user input
        let input = utils::read_input();
        let input_str = String::from_utf8(input.clone()).unwrap_or_default();
        let system_message;
        if input_str.to_lowercase().contains("crypto") {
        system_message = utils::system_message("You are a Crypto Expert. Answer all questions about blockchain and digital currencies.".to_string());
        } else {
        system_message = utils::system_message("You are a general helpful assistant named Atom.".to_string());
        }
        // you can parse the input if you expect to receive a json
        let messages = utils::parse_messages(&input);
        //process the messages
        let modified_messages = utils::process_messages(system_message, messages);

        let cid = "bafkreicevizwv5glcsuhsqzokpowk4oh7kn4zl5xl5eiewjgfvxkhjgzdm".as_bytes().to_vec();

        let message_file = utils::get_cid_file_service(cid);
    
        log(&format!("Message comming a source from a file on IPFS: {:?}", String::from_utf8(message_file).unwrap()));

        let message_file_bytes = utils::get_input_file_service();
        log(&format!("Message from input file: {:?}", String::from_utf8(message_file_bytes).unwrap()));
     
        //transform the messages into a json string and add "messages" key, this is for the LLM model
        let modified_messages_str = format!("{{\"messages\": {}}}", serde_json::to_string(&modified_messages).unwrap());

        let request = utils::prepare_request(&modified_messages_str);
        //call the service offchain
        // call the service offchain and get the response
 
        let response = utils::call_ai_service(1, request);

        // 1. Convert the response to editable text
        let response_str = String::from_utf8(response).unwrap();

        // 2. Add our custom prefix to the response
        let final_output = format!("Hojjat's Agent Answer: {}", response_str);

        // 3. Save the final, modified output
        utils::save_output(final_output.as_bytes());

        

