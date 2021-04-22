extern crate proc_macro;

use cargo_manifest::{DepsSet, Manifest};
use proc_macro::TokenStream;
use std::{env, path::PathBuf};
use syn::Path;

pub fn get_module_path(name: &str) -> Path {
    const BEVY: &str = "bevy";
    const BEVY_INTERNAL: &str = "bevy_internal";

    fn find_in_deps(deps: DepsSet) -> Option<Path> {
        if let Some(dep) = deps.get(BEVY) {
            Some(get_path(dep.package().unwrap_or(BEVY)))
        } else if let Some(dep) = deps.get(BEVY_INTERNAL) {
            Some(get_path(dep.package().unwrap_or(BEVY_INTERNAL)))
        } else {
            None
        }
    }

    let manifest = env::var_os("CARGO_MANIFEST_DIR")
        .map(PathBuf::from)
        .map(|mut path| {
            path.push("Cargo.toml");
            Manifest::from_path(path).unwrap()
        })
        .unwrap();
    let deps = manifest.dependencies;
    let deps_dev = manifest.dev_dependencies;

    manifest
        .package
        .and_then(|p| {
            if p.name == name {
                Some(get_path("crate"))
            } else {
                None
            }
        })
        .or_else(|| deps.and_then(find_in_deps))
        .or_else(|| deps_dev.and_then(find_in_deps))
        .unwrap_or_else(|| get_path(name))
}

pub fn get_path(path: &str) -> Path {
    syn::parse(path.parse::<TokenStream>().unwrap()).unwrap()
}
