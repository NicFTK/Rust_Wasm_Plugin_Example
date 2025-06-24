


wasmtime::component::bindgen!({
    world: "acme:plugins/prettify",
    path: [
        "../../assets/wit/val.wit",
        "../../assets/wit/acme-plugins.wit"
    ],
    with: {
        "local:val/types@1.0.0": val::val::local::val::types,
    },
});



// impl local::val::types:: {
    
// }