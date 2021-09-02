#[cfg(test)]
mod test {
  use std::path::Path;

  use swc_ecmascript::{
    ast::{ExprOrSpread, Program},
    parser::{EsConfig, Parser, Syntax, lexer::Lexer, JscTarget, Capturing}};
  use swc_common::{
    sync::Lrc,
    input::StringInput,
  };
  use swc_common::{
    FileName,
    SourceMap,
  };
  
  #[test]
  fn test() {
    let cm: Lrc<SourceMap> = Default::default();
    let fm = cm.load_file(Path::new("test.js")).unwrap();
    // let lexer = Lexer::new(
    //   Syntax::Es(Default::default()),
    //   JscTarget::Es2021,
    //   StringInput::from(&*fm),
    //   None,
    // );
    // let capturing = Capturing::new(lexer);
    let mut parser = Parser::new(Syntax::Es(Default::default()),
    StringInput::from(&*fm),
    None);
    let script = parser.parse_script().unwrap();
    println!("{:#?}", script.body);
  }
}