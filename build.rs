extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=czmq");
    println!("cargo:rustc-link-lib=zmq");
    let mods = &[
        "zsock",
        "zstr",
        "zmsg",
        "zframe",
        "zactor",
        "zloop",
        "zpoller",
        "zproxy",
        "zmonitor",

        "zauth",
        "zcert",
        "zcertstore",

        // the following modules are optional

        // "zhash",
        // "zhashx",
        // "zlist",
        // "zlistx",

        // "zbeacon",
        // "zclock",
        // "zdir",
        // "zdir_patch",
        // "zfile",
        // "zsys",
        // "zuuid",
        // "ziflist",

        // "zchunk",
        // "zconfig",
        // "zrex",
        // "zgossip",
    ];

    let mut builder = bindgen::Builder::default()
        .header("wrapper.h")
        .whitelist_var("^ZMQ_.*");

    // load listed modules
    for module in mods {
        let regex = format!("^{}_.*", module);
        builder = builder.whitelist_function(&regex);
        builder = builder.whitelist_type(&regex);
    }

    builder = builder
        // hide file types
        .blacklist_type("^__.*")
        .blacklist_type("fpos_t")
        .blacklist_type(".*FILEX?")
        .blacklist_type("zmsg_load")
        .blacklist_type("zmsg_save")
        .blacklist_type(".*_fprint")

        // hide va_list types
        .blacklist_type(".*va_list.*")
        .blacklist_type("zsock_vsend")
        .blacklist_type("zsock_vrecv");

    let bindings = builder.generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
