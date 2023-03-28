/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

// This crate is just a shim around godot-bindings.
// See also gdext's godot-ffi/build.rs which is similar in nature.

use std::path::Path;

fn main() {
    let mut watch = godot_bindings::StopWatch::start();

    let gen_path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../target/godot-gen"));
    // let header_c_path = Path::new(env!("HEADER_C_PATH"));
    // let header_c_path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/src/gen/"));
    let header_c_path = gen_path.join("gdextension_interface.h");
    let header_rs_path = gen_path.join("gdextension_interface.rs");

    // godot_bindings::clear_dir(gen_path, &mut watch);

    // Note: do not call clear_dir(), as we pass in the C header into the gen directory.
    godot_bindings::write_gdextension_header_rs_from_c(&header_c_path, &header_rs_path, &mut watch);

    watch.write_stats_to(&gen_path.join("godot-ffi-stats.txt"));
}
