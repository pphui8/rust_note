//! Cargo offers a feature called workspaces that can help manage multiple related packages that are developed in tandem
//! A workspace is a set of packages that share the same Cargo.lock and output directory

/// how to create a workwpace
fn common_way_to_create_workspace() {
    // 1. creating a new directory for the workspace
    // 2. create the Cargo.toml file that will configure the entire workspace
    // ```toml
    // [workspace]
    //
    // members = [
    //     "adder",
    //     "add-one",
    // ]
    // ```
    // 3. create the adder binary crate by running cargo new within the add directory
    // the file tree would be
    // ```
    // ├── Cargo.lock
    // ├── Cargo.toml
    // ├── add-one
    // │    ├── Cargo.toml
    // │    └── src
    // │        └── lib.rs
    // ├── adder
    // │   ├── Cargo.toml
    // │   └── src
    // │       └── main.rs
    // └── target
    // ```
    // Even if we were to run cargo build from inside the adder directory,
    // the compiled artifacts would still end up in add/target rather than add/adder/target
    // or each create has it`s own target directory

    // Cargo doesn’t assume that crates in a workspace will depend on each other
    // we need to be explicit about the dependency relationships between the crates
    // ```toml
    // add-one = { path = "../add-one" }
    // ```

    // dependences
    // Cargo will resolve both of those to one version of rand and record that in the one Cargo.lock. 
    // Making all crates in the workspace use the same dependencies
    //  even though rand is used somewhere in the workspace, we can’t use it in other crates in the workspace unless we add rand to their Cargo.toml files as well
    // we need to add additional record to the other cargo.toml, but no additional copies will be downloaded

    // If you publish the crates in the workspace to crates.io, each crate in the workspace will need to be published separately
}

/// The cargo install command allows you to install and use binary crates locally
fn install() {
    // Ensure that directory is in your $PATH to be able to run programs you’ve installed with cargo install
}

// Cargo is designed so you can extend it with new subcommands without having to modify Cargo

fn main() {

}