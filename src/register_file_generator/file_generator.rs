use std::{path::Path, fs::{File, OpenOptions}, io::Write, collections::HashSet};
use crate::register_file_generator::register::*;

pub fn create_base_register_files(register_widths: &Vec<u8>, register_family: &String) {
    for register_width in register_widths {
        // Create the base register file
        let file_name = format!("Register{}.h", register_width);
        let path = Path::new(&file_name);
        let display = path.display();

        let mut file = match File::create(&path) {
            Err(why) => panic!("Couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        let full_string = format!(
            "// This file was automatically generated by a register generation tool\n\
            // https://github.com/regerj/register_generator\n\
            // Any changes to this file may be overwritten on subsequent generations\n\
            \n\
            #pragma once\n\
            \n\
            #include <cstdint>\n\
            \n\
            class Register{0} {{\n\
            public:\n\
            \tRegister{0}() = default;\n\
            \tinline uint{0}_t get_register_value() const {{ return register_raw; }};\n\
            \tinline void clear_register_value() {{ register_raw = 0x0; }};\n\
            \tinline void set_register_value(uint{0}_t value) {{ register_raw = value; }};\n\
            \tRegister{0} operator&(const uint{0}_t param) {{ Register{0} buff; buff.register_raw = register_raw & param; return buff; }};\n\
            \tRegister{0} operator&(const Register{0} &param) {{ Register{0} buff; buff.register_raw = register_raw & param.register_raw; return buff; }};\n\
            \tRegister{0} operator|(const uint{0}_t param) {{ Register{0} buff; buff.register_raw = register_raw | param; return buff; }};\n\
            \tRegister{0} operator|(const Register{0} &param) {{ Register{0} buff; buff.register_raw = register_raw | param.register_raw; return buff; }};\n\
            \tRegister{0} operator<<(const uint{0}_t param) {{ Register{0} buff; buff.register_raw = register_raw << param; return buff; }};\n\
            \tRegister{0} operator<<(const Register{0} &param) {{ Register{0} buff; buff.register_raw = register_raw << param.register_raw; return buff; }};\n\
            \tRegister{0} operator>>(const uint{0}_t param) {{ Register{0} buff; buff.register_raw = register_raw >> param; return buff; }};\n\
            \tRegister{0} operator>>(const Register{0} &param) {{ Register{0} buff; buff.register_raw = register_raw >> param.register_raw; return buff; }};\n\
            \tRegister{0} operator~() {{ Register{0} buff; buff.register_raw = ~register_raw; return buff; }};\n\
            protected:\n\
            \tuint{0}_t register_raw = 0x0;\n\
            }};\n",
           register_width 
        );

        match file.write_all(full_string.as_bytes()) {
            Err(why) => panic!("Couldn't write to {}: {}", display, why),
            Ok(_) => println!("Wrote {}", display),
        }
    }

    // Create the register family file or append a new include
    let file_name = format!("{}Registers.h", register_family);
    let path = Path::new(&file_name);

    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("Couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    let mut includes = String::new();
    for register_width in register_widths {
        let include = format!("#include \"Register{}.h\"\n", register_width);
        includes.push_str(&include);
    }

    let full_string = format!(
        "// This file was automatically generated by a register generation tool\n\
        // https://github.com/regerj/register_generator\n\
        // Any changes to this file may be overwritten on subsequent generations\n\
        \n\
        #pragma once\n\
        \n\
        #include <cstdint>\n\
        \n\
        {0}\n",
        includes
    );

    match file.write_all(full_string.as_bytes()) {
        Err(why) => panic!("Couldn't write to {}: {}", display, why),
        Ok(_) => println!("Wrote {}", display),
    }
}

pub fn write_register_to_file(register: &Register, register_family: &String) {
    // Supported register widths
    let supported_register_widths: HashSet<u8> = HashSet::from([8, 16, 32, 64]);
    if !supported_register_widths.contains(&register._size) {
        panic!("Invalid register width!");
    } 

    // Determine the name of the register file and get the path
    let file_name = format!("{}Registers.h", register_family);
    let path = Path::new(&file_name);
    let display = path.display();

    // Open for appending
    let mut file = match OpenOptions::new().append(true).create(false).open(&path) {
        Err(why) => panic!("Couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    // Strings for each set of methods
    let mut get_methods = String::new();
    let mut set_methods = String::new();

    // Check permissions and create a method if allowed
    for current_field in register.fields.iter() {
        if current_field.read {
            get_methods.push_str(&current_field.create_get_method(register._size));
        }

        if current_field.write {
            set_methods.push_str(&current_field.create_set_method(register._size));
        }
    }

    let full_string = format!(
        "class {0} : public Register{3} {{\n\
        public:\n\
        \t{0}() : Register{3}() {{}};\n\
        \n\
        \t// Get methods\n\
        {1}\n\
        \t// Set methods\n\
        {2}\
        }};\n\n",
        register.name,
        get_methods,
        set_methods,
        register._size
    );

    match file.write_all(full_string.as_bytes()) {
        Err(why) => panic!("Couldn't write to {}: {}", display, why),
        Ok(_) => println!("Wrote {}", display),
    }
}
