mod bindings {
    wit_bindgen::generate!({ generate_all });

    use crate::Glyphic;
    export! {Glyphic}
}

use crate::bindings::exports::wasco_dev::glyphic::glyphic::Guest;

struct Glyphic;

impl Guest for Glyphic {
    fn get_calls() -> String {
        todo!()
    }

    fn get_call_by_id(_id: String) -> String {
        todo!()
    }

    fn get_call_media_by_id(_id: String) -> String {
        todo!()
    }

    fn get_call_snippets_by_id(_id: String) -> String {
        todo!()
    }
}

