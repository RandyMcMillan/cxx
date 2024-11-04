fn main() {
    cxx_build::bridge("src/main.rs")
        .file("src/blobstore.cc")
        .std("c++14")
        .compile("cxxbridge-gnostr");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/blobstore.cc");
    println!("cargo:rerun-if-changed=include/blobstore.h");
}
