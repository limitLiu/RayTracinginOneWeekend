use std::path::PathBuf;

fn main() {
  let out_dir = PathBuf::from("./generated");

  #[cfg(not(feature = "spm"))]
  {
    let bridges = vec!["src/lib.rs"];
    for path in &bridges {
      println!("cargo:rerun-if-changed={}", path);
    }
    swift_bridge_build::parse_bridges(bridges)
      .write_all_concatenated(out_dir, env!("CARGO_PKG_NAME"));
  }

  #[cfg(feature = "spm")]
  {
    use std::collections::HashMap;
    use swift_bridge_build::{ApplePlatform, CreatePackageConfig};
    swift_bridge_build::create_package(CreatePackageConfig {
      bridge_dir: out_dir,
      paths: HashMap::from([
        (
          ApplePlatform::IOS,
          "target/aarch64-apple-ios/release/libalgorithm.a".into(),
        ),
        (
          ApplePlatform::Simulator,
          "target/aarch64-apple-ios-sim/release/libalgorithm.a".into(),
        ),
      ]),
      out_dir: PathBuf::from("Algorithm"),
      package_name: String::from("Algorithm"),
    });
  }
}
