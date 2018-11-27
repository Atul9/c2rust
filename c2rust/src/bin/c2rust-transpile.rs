#[macro_use]
extern crate clap;
extern crate c2rust_transpile;

use std::path::Path;
use clap::App;

use c2rust_transpile::{TranspilerConfig, ReplaceMode};


fn main() {
    let yaml = load_yaml!("../transpile.yaml");
    let matches = App::from_yaml(yaml)
        .get_matches();

    // Build a TranspilerConfig from the command line
    let c_path = Path::new(matches.value_of("INPUT").unwrap()).canonicalize().unwrap();
    let extra_args: Vec<&str> = match matches.values_of("extra-clang-args") {
        Some(args) => args.collect(),
        None => Vec::new(),
    };
    let tcfg = TranspilerConfig {
        dump_untyped_context:   matches.is_present("dump-untyped-clang-ast"),
        dump_typed_context:     matches.is_present("dump-typed-clang-ast"),
        pretty_typed_context:   matches.is_present("pretty-typed-clang-ast"),
        dump_function_cfgs:     matches.is_present("dump-function-cfgs"),
        json_function_cfgs:     matches.is_present("json-function-cfgs"),
        dump_cfg_liveness:      matches.is_present("dump-cfgs-liveness"),
        dump_structures:        matches.is_present("dump-structures"),

        incremental_relooper:   !matches.is_present("no-incremental-relooper"),
        fail_on_error:          matches.is_present("fail-on-error"),
        fail_on_multiple:       matches.is_present("fail-on-multiple"),
        debug_relooper_labels:  matches.is_present("debug-labels"),
        cross_checks:           matches.is_present("cross-checks"),
        cross_check_configs:    matches.values_of("cross-check-config")
            .map(|vals| vals.map(String::from).collect::<Vec<_>>())
            .unwrap_or_default(),
        prefix_function_names:  matches.value_of("prefix-function-names")
            .map(String::from),
        translate_asm:          matches.is_present("translate-asm"),
        translate_entry:        matches.is_present("translate-entry"),
        translate_valist:       matches.is_present("translate-valist"),
        use_c_loop_info:        !matches.is_present("ignore-c-loop-info"),
        use_c_multiple_info:    !matches.is_present("ignore-c-multiple-info"),
        simplify_structures:    !matches.is_present("no-simplify-structures"),
        reduce_type_annotations:matches.is_present("reduce-type-annotations"),
        reorganize_definitions: matches.is_present("reorganize-definitions"),
        emit_module:            matches.is_present("emit-module"),
        main_file:              c_path.with_extension(""),
        output_file:            matches.value_of("output-file").map(|s| s.to_string()),
        panic_on_translator_failure: {
            match matches.value_of("invalid-code") {
                Some("panic") => true,
                Some("compile_error") => false,
                _ => panic!("Invalid option"),
            }
        },
        replace_unsupported_decls: ReplaceMode::Extern,
    };

    c2rust_transpile::transpile(tcfg, &c_path, &extra_args);
}