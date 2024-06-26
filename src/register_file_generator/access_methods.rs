use crate::register_file_generator::register::*;

pub fn create_get_method(field: &Field, register_width: u8) -> String {
    return match field.negative {
        Some(true) => format!(
            "\tinline int{3}_t get_{0}() const {{\n\
            \t\tuint{3}_t buffer = register_raw >> {1};\n\
            \t\tuint{3}_t field_raw = buffer & (UINT{3}_MAX >> ({3} - 1 - ({2} - {1})));\n\
            \t\tif (field_raw & (1 << ({2} - {1}))) {{\n\
            \t\t\tfield_raw |= (UINT{3}_MAX << ({2} - {1} + 1));\n\
            \t\t}}\n\
            \t\treturn field_raw;\n\
            \t}}\n",
            field.name,
            field.lsb,
            field.msb,
            register_width
        ),
        _ => format!(
            "\tinline uint{3}_t get_{0}() const {{\n\
            \t\tuint{3}_t buffer = register_raw >> {1};\n\
            \t\treturn buffer & (UINT{3}_MAX >> ({3} - 1 - ({2} - {1})));\n\
            \t}}\n",
            field.name,
            field.lsb,
            field.msb,
            register_width
        )
    }
}

pub fn create_set_method(field: &Field, register_width: u8) -> String {
    // Negative numbers need to be bounds checked differently
    return match field.negative {
        Some(true) => format!(
            "\tinline bool set_{0}(int{3}_t value) {{\n\
            \t\tif (value < 0) {{\n\
            \t\t\tif (-value > ((int{3}_t)1 << ({2} - {1}))) {{\n\
            \t\t\t\treturn false;\n\
            \t\t\t}}\n\
            \t\t}} else {{\n\
            \t\t\tif (value >= ((int{3}_t)1 << ({2} - {1}))) {{\n\
            \t\t\t\treturn false;\n\
            \t\t\t}}\n\
            \t\t}}\n\
            \t\tuint{3}_t mask = static_cast<uint{3}_t>(~((UINT{3}_MAX >> ({3} - 1 - ({2} - {1}))) << {1}));\n\
            \t\tregister_raw &= mask;\n\
            \t\tvalue &= (UINT{3}_MAX >> ({3} - 1 - ({2} - {1})));\n\
            \t\tvalue = value << {1};\n\
            \t\tregister_raw |= value;\n\
            \t\treturn true;\n\
            \t}}\n",
            field.name,
            field.lsb,
            field.msb,
            register_width
        ),
        _ => format!(
            "\tinline bool set_{0}(uint{3}_t value) {{\n\
            \t\tif (value >= ((uint{3}_t)1 << ({2} - ({1} - 1)))) {{\n\
            \t\t\treturn false;\n\
            \t\t}}\n\
            \t\tuint{3}_t mask = static_cast<uint{3}_t>(~((UINT{3}_MAX >> ({3} - 1 - ({2} - {1}))) << {1}));\n\
            \t\tregister_raw &= mask;\n\
            \t\tvalue = value << {1};\n\
            \t\tregister_raw |= value;\n\
            \t\treturn true;\n\
            \t}}\n",
            field.name,
            field.lsb,
            field.msb,
            register_width
        )
    }
}
