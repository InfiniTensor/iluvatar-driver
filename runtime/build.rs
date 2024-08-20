use std::{env, path::PathBuf};

fn main() {
    use build_script_cfg::Cfg;
    use search_iluvatar_tools::find_ilu_home;

    println!("cargo:rereun-if-changed=build.rs");

    let ilu = Cfg::new("detected_ilu");
    let Some(ilu_home) = find_ilu_home() else {
        return;
    };
    ilu.define();
    println!("{}", ilu_home.join("lib").display());
    println!(
        "cargo:rustc-link-search=native={}",
        ilu_home.join("lib").display()
    );
    println!("cargo:rustc-link-lib=dylib=nvrtc");
    println!("cargo:rustc-link-lib=dylib=cuda");

    println!("cargo:rustc-env=CUDA_ROOT={}", ilu_home.display());

    println!("cargo-rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-I{}", ilu_home.join("include").display()))
        .allowlist_item("cu.*")
        .allowlist_function("nvrtc.*")
        .allowlist_item("CU.*")
        .must_use_type("CUresult")
        .must_use_type("nvrtcResult")
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: true,
        })
        .use_core()
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
