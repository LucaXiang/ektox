use ektox::common::Version;
fn main() {
    let version = Version::from_cargo_package();
    println!("{}", version);
}
