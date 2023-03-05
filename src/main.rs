use std::path::Path;
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
use jwalk::{WalkDir};

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
    
    for entry in WalkDir::new("site") {
        let entry = entry.unwrap();
        
        if Path::new(&entry.path()).is_dir() {
            continue; 
        }
        
        let url = mitus::url::to_url(entry.path().to_string_lossy().to_string());
        let source = std::fs::read_to_string(entry.path()).unwrap();
        
        let result = transform.parse(ParseParams {
            specifier: url.clone(),
            media_type: MediaType::TypeScript,
            text_info: SourceTextInfo::new(source.into()),
            capture_tokens: true,
            maybe_syntax: Some(Syntax::Typescript(TsConfig {
                tsx: true,
                decorators: false,
                dts: false,
                no_early_errors: false
            })),
            scope_analysis: false,
        }).unwrap();
        
        println!("url: {}, source: {}", url, result);
    }
}