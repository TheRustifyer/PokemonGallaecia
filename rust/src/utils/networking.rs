use gdnative::prelude::*;
use gdnative::api::{HTTPClient, HTTPRequest, JSON};

// struct Net<T>(T); // Generic type `SGen`.

// /// Performs and HTTP request over the network given an URI, inserting a new HTTP Godot node on the tree
// /// and creating and asingning a signal to a request, sending the response to the connected method by the designed signal.
// pub fn http_get_request<T>(_owner: Net<T>) {
//     let http_request: Ref<HTTPRequest, Unique> = HTTPRequest::new();
//     let http_request_as_node = unsafe { http_request.assume_safe_unchecked().assume_shared().assume_safe() };

   
// }

/// Method to parse an http body that comes as a ByteArray
pub fn http_body_to_string(body: ByteArray) -> Dictionary {
    // JSON class to parse the retrieved content
    let json = JSON::godot_singleton();
    // Allocates a new vector where the bites from the ByteArray will be pushed
    let mut vector = Vec::new();
    // Iterates over the bites to pass them to our vector
    for number in 0..body.len() {
        vector.push(body.get(number))
    }
    // Converts the bytes to a human-readable version
    let final_vec = std::str::from_utf8(&vector).unwrap();
    // Returns the result as a Rust String
    unsafe { json.parse(final_vec).unwrap().assume_safe().result().to_dictionary() }
}