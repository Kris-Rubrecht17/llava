






fn main(){
    if !std::fs::exists("llvm").unwrap() {
        println!("cargo::warning=Unable to find llvm dir")
    }

    println!("cargo::rustc-link-search=native=llvm/bin");
    #[cfg(target_os = "linux")]
    {    
        println!("cargo::rustc-link-arg=-l:libLLVM.so.21.1");
        println!("cargo::rustc-link-lib=static=wrapper_linux")
    }
    #[cfg(target_os = "windows")]
    {
        println!("cargo::rustc-link-lib=dylib=LLVM-C");
    }
    println!("cargo:rustc-link-arg=-Wl,-rpath,llvm/bin");
    
    println!("cargo::rerun-if-changed=build.rs");
}