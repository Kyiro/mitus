use deno_ast::{
    fold_program,
    parse_module,
    EmitOptions,
    ParseParams,
    swc::codegen::{
        text_writer::JsWriter,
        Config as CodegenConfig,
        Emitter as CodegenEmitter
    },
    swc::common::{
        GLOBALS,
        Globals,
        Mark,
        SourceMap
    }
};
use std::rc::Rc;

#[derive(Clone, Default)]
pub struct TransformBuilder {
    emit_options: EmitOptions,
    codegen_config: CodegenConfig,
    comments: bool
}

impl TransformBuilder {
    pub fn emit_options(mut self, emit_options: EmitOptions) -> Self {
        self.emit_options = emit_options;
        self
    }
    
    pub fn codegen_config(mut self, codegen_config: CodegenConfig) -> Self {
        self.codegen_config = codegen_config;
        self
    }
    
    pub fn parse(&self, parse: ParseParams) -> Result<String, deno_ast::Diagnostic> {
        let parsed_source = parse_module(parse)?;
        
        let program = (*parsed_source.program()).clone();
        let source_map = Rc::new(SourceMap::default());
        let comments = parsed_source.comments().as_single_threaded();
        let globals = Globals::new();
        
        let transpiled = GLOBALS
            .set(&globals, || -> Result<String, Box<dyn std::error::Error>> {
                let top_level_mark = Mark::fresh(Mark::root());
                let program = fold_program(
                    program,
                    &self.emit_options,
                    source_map.clone(),
                    &comments,
                    top_level_mark,
                    parsed_source.diagnostics(),
                )?;
            
                let mut buf = Vec::new();
                let writer = Box::new(JsWriter::new(source_map.clone(), "\n", &mut buf, None));
            
                let mut emitter = CodegenEmitter {
                    cfg: self.codegen_config,
                    comments: self.comments.then_some(&comments),
                    cm: source_map.clone(),
                    wr: writer,
                };
            
                emitter
                    .emit_program(&program)?;
            
                Ok(String::from_utf8(buf)?)
            });
        
        Ok(transpiled.unwrap())
    }
}