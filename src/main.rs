use mitus::{
    TsConfig,
    EmitOptions,
    Syntax,
    MediaType,
    TransformBuilder,
    ParseParams,
    SourceTextInfo,
    CodegenConfig
};
use dioxus::prelude::*;

fn main() {
    let transform = TransformBuilder::default()
        .codegen_config(CodegenConfig {
            minify: true,
            ..CodegenConfig::default()
        })
        .emit_options(EmitOptions {
            jsx_import_source: Some("preact".to_string()),
            jsx_automatic: true,
            inline_source_map: false,
            jsx_development: false,
            ..EmitOptions::default()
        });
    
    let result = transform.parse(ParseParams {
        specifier: "file:///example.tsx".to_string(),
        media_type: MediaType::TypeScript,
        text_info: SourceTextInfo::new(include_str!("./../site/example.tsx").into()),
        capture_tokens: true,
        maybe_syntax: Some(Syntax::Typescript(TsConfig {
            tsx: true,
            decorators: false,
            dts: false,
            no_early_errors: false
        })),
        scope_analysis: false,
    }).unwrap();
    
    let import_map = include_str!("./../importMap.json");
    
    println!("{}", dioxus_ssr::render_lazy(rsx! {
        head {
            title { "Mitus Example" }
            // TO-DO: Make this automatic or something
            script {
                r#type: "importmap",
                import_map
            },
            script {
                r#type: "module",
                result
            }
        }
        body {
            // figure out how to use web components in dioxus rsx normally
            "<x-component/>"
        }
    }));
}