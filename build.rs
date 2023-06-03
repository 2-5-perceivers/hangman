fn main() {
  configure_linux();
}

#[cfg(target_os = "linux")]
fn configure_linux() {
  // Get the path to the raylib shared library

  use copy_to_output::copy_to_output;
  let lib_dir = "./lib";

  // Set the environment variable for the runtime library search path
  println!("cargo:rustc-env=LD_LIBRARY_PATH={}", lib_dir);

  // Link against the raylib shared library
  println!("cargo:rustc-link-search=native={}", lib_dir);
  println!("cargo:rustc-link-lib=dylib=raylib");
  println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN/lib");

  copy_to_output("lib", &std::env::var("PROFILE").unwrap())
    .expect("Could not copy");
}
