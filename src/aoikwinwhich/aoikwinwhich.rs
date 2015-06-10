//
use std::ascii::AsciiExt;
use std::env;
use std::fs;
use std::process;

//
fn string_to_str(string: &String) -> &str {
    return &string;
}

//
fn string_to_lower(string: &String) -> String {
    return string.chars().map(|c| c.to_ascii_lowercase()).collect();
}

//
fn path_is_file(path: &String) -> bool {
    return match fs::metadata(path) {
        Err(_) => false,
        Ok(meta) => meta.is_file(),
    };
}

//
fn strings_uniq(item_s: &Vec<String>) -> Vec<String> {
    //
    let mut item_s_uniq : Vec<String> = Vec::new();

    //
    for item in item_s.iter() {
        if !item_s_uniq.contains(&item) {
            item_s_uniq.push(item.to_string());
        }
    }

    //
    return item_s_uniq
}

//
fn find_exe_paths(prog: &String) -> Vec<String> {
    // 8f1kRCu
    let env_pathext = match env::var("PATHEXT") {
        Ok(v) => v,
        Err(_) => "".to_string(),
    };

    // 4fpQ2RB
    if env_pathext.is_empty() {
        // 9dqlPRg
        return vec![];
    }

    // 6qhHTHF
    // Split into a list of extensions
    let mut ext_s : Vec<String> = env_pathext.split(";").map(|x| x.to_string())
        .collect();

    // 2pGJrMW
    // Strip
    ext_s = ext_s.iter().map(|x| x.trim().to_string()).collect();

    // 2gqeHHl
    // Remove empty.
    // Must be done after the stripping at 2pGJrMW.
    ext_s = ext_s.iter().filter(|x| !x.is_empty()).map(|x| x.to_string())
        .collect();

    // 2zdGM8W
    // Convert to lowercase
    ext_s = ext_s.iter().map(|x| string_to_lower(x)).collect();

    // 2fT8aRB
    // Uniquify
    ext_s = strings_uniq(&ext_s);

    // 4ysaQVN
    let env_path = match env::var("PATH") {
        Ok(v) => v,
        Err(_) => "".to_string(),
    };

    // 5gGwKZL
    let mut dir_path_s : Vec<String> =
        // 7bVmOKe
        // Go ahead with "dir_path_s" being empty
        if env_path.is_empty() {
            vec![]
        }
        else {
        // 6mPI0lg
        // Split into a list of dir paths
        env_path.split(";").map(|x| x.to_string()).collect()
        };

    // 5rT49zI
    // Insert empty dir path to the beginning.
    //
    // Empty dir handles the case that "prog" is a path, either relative or
    //  absolute. See code 7rO7NIN.
    dir_path_s.insert(0, "".to_string());

    // 2klTv20
    // Uniquify
    dir_path_s = strings_uniq(&dir_path_s);

    // 9gTU1rI
    // Check if "prog" ends with one of the file extension in "ext_s".
    //
    // "ext_s" are all in lowercase, ensured at 2zdGM8W.
    let prog_lc : &str = & string_to_lower(prog);

    let prog_has_ext = ext_s.iter().find(|ext|
        prog_lc.ends_with(string_to_str(ext))).is_some();

    // 6bFwhbv
    let mut exe_path_s : Vec<String> = Vec::new();

    for dir_path in dir_path_s.iter() {
        // 7rO7NIN
        // Synthesize a path
        let path =
            if dir_path.is_empty() {
                prog.to_owned()
            }
            else {
                format!("{}\\{}", dir_path, prog)
            };

        // 6kZa5cq
        // If "prog" ends with executable file extension
        if prog_has_ext {
            // 3whKebE
            if path_is_file(&path) {
                // 2ffmxRF
                exe_path_s.push(path.to_owned());
            }
        }

        // 2sJhhEV
        // Assume user has omitted the file extension
        for ext in ext_s.iter() {
            // 6k9X6GP
            // Synthesize a path with one of the file extensions in PATHEXT
            let path_2 = format!("{}{}", path, ext);

            // 6kabzQg
            if path_is_file(&path_2) {
                // 7dui4cD
                exe_path_s.push(path_2);
            }
        }
    }

    // 8swW6Av
    // Uniquify
    exe_path_s = strings_uniq(&exe_path_s);

    // 7y3JlnS
    return exe_path_s;
}

// 4zKrqsC
// Program entry
fn main() {
    //
    let arg_s: Vec<_> = env::args().collect();

    // 9mlJlKg
    // If not exactly one command argument is given
    if arg_s.len() != 2 {
        // 7rOUXFo
        // Print program usage
        let usage = "Usage: aoikwinwhich PROG

#/ PROG can be either name or path
aoikwinwhich notepad.exe
aoikwinwhich C:\\Windows\\notepad.exe

#/ PROG can be either absolute or relative
aoikwinwhich C:\\Windows\\notepad.exe
aoikwinwhich Windows\\notepad.exe

#/ PROG can be either with or without extension
aoikwinwhich notepad.exe
aoikwinwhich notepad
aoikwinwhich C:\\Windows\\notepad.exe
aoikwinwhich C:\\Windows\\notepad";

        println!("{}", usage);

        // 3nqHnP7
        process::exit(1);
    }

    // 9m5B08H
    // Get executable name or path
    let ref prog = arg_s[1];

    // 8ulvPXM
    // Find executable paths
    let ref exe_path_s = find_exe_paths(prog);

    // 5fWrcaF
    // If has found none
    if arg_s.len() == 0 {
        // 3uswpx0
        process::exit(2);
    }
    else {
        // 9xPCWuS
        // Print result
        println!("{}", exe_path_s.connect("\n"));

        // 4s1yY1b
        process::exit(0);
    }
}
