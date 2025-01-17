use serde_json::json;
use std::ffi::CStr;
use std::io::{self, Write};
use std::ffi::CString;
use std::os::raw::{c_char, c_float, c_int};

pub type OnInitCb = extern "C" fn();
pub type OnTextChangedCb = extern "C" fn(id: c_int, text: *const c_char);
pub type OnComboChangedCb = extern "C" fn(id: c_int, index: c_int);
pub type OnNumericValueChangedCb = extern "C" fn(id: c_int, value: c_float);
pub type OnBooleanValueChangedCb = extern "C" fn(id: c_int, value: bool);
pub type OnMultipleNumericValuesChangedCb =
extern "C" fn(id: c_int, values: *const c_float, num_values: c_int);
pub type OnClickCb = extern "C" fn(id: c_int);


#[link(name = "xframesshared")]
extern "C" {
    pub fn init(
        assets_base_path: *const c_char,
        raw_font_definitions: *const c_char,
        raw_style_override_definitions: *const c_char,
        on_init: OnInitCb,
        on_text_changed: OnTextChangedCb,
        on_combo_changed: OnComboChangedCb,
        on_numeric_value_changed: OnNumericValueChangedCb,
        on_boolean_value_changed: OnBooleanValueChangedCb,
        on_multiple_numeric_values_changed: OnMultipleNumericValuesChangedCb,
        on_click: OnClickCb,
    );

    fn setElement(element_json: *const libc::c_char);
    fn setChildren(id: libc::c_int, children_ids: *const libc::c_char);
}

extern "C" fn on_init_callback() {
    println!("Init callback called");

    // For some reason `0` as integer isn't serialized / passed correctly. Same as in Free Pascal on x64-linux
    let root_node = json!({
        "id": 0f64,
        "type": "node",
        "root": true
    });

    let unformatted_text = json!({
        "id": 1,
        "type": "unformatted-text",
        "text": "Hello, world"
    });

    let root_node_json = root_node.to_string();
    let unformatted_text_json = unformatted_text.to_string();

    let root_node_json_cstring = CString::new(root_node_json).expect("CString::new failed");
    let unformatted_text_json_cstring = CString::new(unformatted_text_json).expect("CString::new failed");

    let children = json!([1]);
    let children_json = children.to_string();
    let children_json_cstring = CString::new(children_json).expect("CString::new failed");

    unsafe {
        setElement(root_node_json_cstring.as_ptr());
        setElement(unformatted_text_json_cstring.as_ptr());

        setChildren(0, children_json_cstring.as_ptr());
    }
}

extern "C" fn on_text_changed_callback(id: c_int, text: *const c_char) {
    unsafe {
        let text_str = CStr::from_ptr(text).to_string_lossy();
        println!("Text changed (id: {}, text: {})", id, text_str);
    }
}

extern "C" fn on_combo_changed_callback(id: c_int, index: c_int) {
    println!("Combo changed (id: {}, index: {})", id, index);
}

extern "C" fn on_numeric_value_changed_callback(id: c_int, value: c_float) {
    println!("Numeric value changed (id: {}, value: {})", id, value);
}

extern "C" fn on_boolean_value_changed_callback(id: c_int, value: bool) {
    println!("Boolean value changed (id: {}, value: {})", id, value);
}

extern "C" fn on_multiple_numeric_values_changed_callback(
    id: c_int,
    values: *const c_float,
    num_values: c_int,
) {
    unsafe {
        let values_slice = std::slice::from_raw_parts(values, num_values as usize);
        println!("Multiple numeric values changed (id: {}, values: {:?})", id, values_slice);
    }
}

extern "C" fn on_click_callback(id: c_int) {
    println!("Click callback called (id: {})", id);
}

fn main() {
    unsafe {
        let font_defs = json!({
            "defs": [{
                "name": "roboto-regular",
                "size": 16
            }]
        });

        let font_defs_json = font_defs.to_string();

        // Prepare strings
        let assets_base_path = CString::new("./assets").unwrap();
        let raw_font_definitions = CString::new(font_defs_json).unwrap();
        let raw_style_override_definitions = CString::new("{}").unwrap();

        // Call init
        init(
            assets_base_path.as_ptr(),
            raw_font_definitions.as_ptr(),
            raw_style_override_definitions.as_ptr(),
            on_init_callback,
            on_text_changed_callback,
            on_combo_changed_callback,
            on_numeric_value_changed_callback,
            on_boolean_value_changed_callback,
            on_multiple_numeric_values_changed_callback,
            on_click_callback,
        );

        println!("Press Enter to exit...");
        let mut buffer = String::new();
        io::stdout().flush().unwrap(); // Ensure the prompt is shown
        io::stdin().read_line(&mut buffer).unwrap(); // Wait for user input
        println!("Exiting...");

    }
}
