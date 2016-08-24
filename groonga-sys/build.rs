extern crate pkg_config;

use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();

    let lib_dir = env::var("GROONGA_LIB_DIR").ok();
    let include_dir = env::var("GROONGA_INCLUDE_DIR").ok();

    if lib_dir.is_none() && include_dir.is_none() {
        // rustc doesn't seem to work with pkg-config's output in mingw64
        if !target.contains("windows") {
            if let Ok(info) = pkg_config::find_library("groonga") {
                // avoid empty include paths as they are not supported by GCC
                if info.include_paths.len() > 0 {
                    let paths = env::join_paths(info.include_paths).unwrap();
                    println!("cargo:include={}", paths.to_str().unwrap());
                }
                return;
            }
        }
        if let Some(mingw_paths) = get_mingw_in_path() {
            for path in mingw_paths {
                println!("cargo:rustc-link-search=native={}", path);
            }
        }
    }

    let lib = "groonga";

    let mode = if env::var_os("GROONGA_STATIC").is_some() {
    	"static"
    } else {
    	"dylib"
    };

    if let Some(lib_dir) = lib_dir {
    	println!("cargo:rustc-link-search=native={}", lib_dir);
    }

    println!("cargo:rustc-link-lib={}={}", mode, lib);

    if let Some(include_dir) = include_dir {
        println!("cargo:include={}", include_dir);
    }
}

fn get_mingw_in_path() -> Option<Vec<String>> {
    match env::var_os("PATH") {
        Some(env_path) => {
            let paths: Vec<String> = env::split_paths(&env_path).filter_map(|path| {
                use std::ascii::AsciiExt;

                match path.to_str() {
                    Some(path_str) => {
                        if path_str.to_ascii_lowercase().contains("mingw") {
                            Some(path_str.to_string())
                        } else { None }
                    },
                    None => None
                }
            }).collect();

            if paths.len() > 0 { Some(paths) } else { None }
        },
        None => None
    }
}
