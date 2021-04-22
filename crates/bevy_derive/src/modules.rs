use bevy_macro_utils::{get_module_path, get_path};
use syn::Attribute;

pub struct Modules {
    pub bevy_render: syn::Path,
    pub bevy_asset: syn::Path,
    pub bevy_core: syn::Path,
    pub bevy_app: syn::Path,
}

const AS_CRATE_ATTRIBUTE_NAME: &str = "as_crate";

pub fn get_modules(attributes: &[Attribute]) -> Modules {
    let mut modules = Modules {
        bevy_render: get_module_path("bevy_render"),
        bevy_asset: get_module_path("bevy_asset"),
        bevy_core: get_module_path("bevy_core"),
        bevy_app: get_module_path("bevy_app"),
    };
    for attribute in attributes.iter() {
        if *attribute.path.get_ident().as_ref().unwrap() == AS_CRATE_ATTRIBUTE_NAME {
            let value = attribute.tokens.to_string();
            if get_path(&value[1..value.len() - 1]) == modules.bevy_render {
                modules.bevy_render = get_path("crate");
            }
        }
    }

    modules
}
