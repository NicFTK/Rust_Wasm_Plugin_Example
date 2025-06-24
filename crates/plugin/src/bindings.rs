// Note that we are using the full notation of the macro in order
// to add some options that allow the bindings to live in this independent file

// wit_bindgen::generate!({
//     world:"acme:plugins/prettify",
//     path: ["../../assets/wit/val.wit","../../assets/wit/acme-plugins.wit",],
//     default_bindings_module: "crate::bindings",
//     pub_export_macro: true,
//     generate_all,
// });






wit_bindgen::generate!({

    world:"acme:plugins/prettify",
    path: ["../../assets/wit/val.wit","../../assets/wit/acme-plugins.wit",],

    default_bindings_module: "crate::bindings",
    pub_export_macro: true,
    with: {

        "local:val/types@1.0.0": val::val::local::val::types, // world 路径和名字要和你的 wit 文件一致
    }
});
