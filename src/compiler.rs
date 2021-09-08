use std::path::{Path, PathBuf};

use swc_ecmascript::{ast::{Expr, ExprStmt, Lit, Program, Script, Stmt, Str}, parser::{Parser, Syntax, lexer::Lexer, JscTarget, Capturing}, visit::{All, Visit}};
use swc_common::{
  sync::Lrc,
  input::StringInput,
};
use swc_common::{
  SourceMap,
};


pub struct Compiler {
  path: PathBuf,
  script: Script,
  strict: bool,
}

impl Compiler {
  pub fn parse_script(path: PathBuf) -> Compiler {
    let cm: Lrc<SourceMap> = Default::default();
    let fm = cm.load_file(&path).unwrap();
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
    Compiler {
      path,
      strict: Compiler::is_strict(&script),
      script,
    }
  }

  #[inline]
  pub fn is_strict(script: & Script) -> bool {
    if !script.body.is_empty() {
      let statement = &script.body[0];
      if let Stmt::Expr(ExprStmt {expr, ..}) = statement {
        if let Expr::Lit(lit) = &**expr {
          if let Lit::Str(Str {
            ref value,
            has_escape: false,
            ..
          }) = lit {
            return value == "use strict";
          }
        }
      }
    }
    false
  }

  pub fn decode(&self) {
    // 1. 先验证是否严格模式？
  }
}


#[cfg(test)]
mod test {
  use super::*;
  #[test]
  fn is_strict_work() {
    let c = Compiler::parse_script(Path::new("./testJs/isStrict.js").into());
    assert!(c.strict);
    let not = Compiler::parse_script(Path::new("./testJs/notStrict.js").into());
    assert!(!not.strict);
  }
}