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
    let profile = std::env::var("PROFILE").unwrap();
    let ios = format!("target/aarch64-apple-ios/{}/libalgorithm.a", profile);
    let simulator = format!("target/aarch64-apple-ios-sim/{}/libalgorithm.a", profile);
    swift_bridge_build::create_package(CreatePackageConfig {
      bridge_dir: out_dir,
      paths: HashMap::from([
        (ApplePlatform::IOS, ios.into()),
        (ApplePlatform::Simulator, simulator.into()),
      ]),
      out_dir: PathBuf::from("Algorithm"),
      package_name: String::from("Algorithm"),
    });
  }
}
