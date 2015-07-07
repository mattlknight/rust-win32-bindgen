use itertools::Itertools;
use clang::Cursor;
use features::Features;

use super::{EMIT_STUBS, file_stem};
use super::output::OutputItems;

/**
Process a single macro definition.
*/
pub fn process_macro_defn(defn_cur: Cursor, output: &mut OutputItems, feat: Features) -> Result<(), String> {
    use ::ppmac::parse;
    use ::ppmac::parse::Result as PResult;

    debug!("process_macro_defn({}, ..)", defn_cur);

    // Note: we skip the last token because it's just a newline.
    let toks = defn_cur.tokenize();
    let first_tok = toks.at(0);
    let next_tok = toks.get(1);
    let toks: Vec<_> = toks.into_iter().dropping(1).dropping_back(1).map(|t| t.spelling()).collect();

    // If it has no tokens... well, there's not much point.
    if toks.len() == 0 { return Ok(()); }

    let name = defn_cur.spelling();
    let header = file_stem(&defn_cur);
    let annot = defn_cur.location().display_short().to_string();

    // Work out whether this is a functionish macro or not.
    let is_fn_macro = {
        let first_col = first_tok.extent().expect("extent for macro first tok").end().column();
        let next_col = next_tok.map(|t| t.extent().expect("extent for macro next tok").end().column()).unwrap_or(!0);
        first_col + 1 == next_col
    };

    let (args, exp_toks) = if is_fn_macro {
        let args_end = toks.iter().take_while(|tok| *tok != ")").count();
        (&toks[0..args_end+1], &toks[args_end+1..])
    } else {
        (&toks[0..0], &toks[0..])
    };

    let exp_ast = match parse::expression(exp_toks) {
        PResult::Parsed(node, rem) => {
            if rem.len() != 0 {
                return Err(format!("incomplete parse: {:?}, leaving {:?}", node, rem));
            }
            node
        },
        PResult::Mismatch(err, rem) => {
            return Err(format!("could not parse {}, leaving {:?}", err, rem));
        }
    };

    if let Some((v, t)) = try_trans_inty_macro(&exp_ast) {
        let decl = format!("pub const {}: {} = {}; /* {:?} */", name, t, v, exp_ast);
        output.add_header_item(name, header, feat, decl, annot);
        return Ok(());
    }

    if EMIT_STUBS {
        let decl = format!("// #define {}{} {:?}", name, args.connect(""), exp_ast);
        output.add_header_item(name, header, feat, decl, annot);
    }

    Err("unsupported-macro".into())
}

fn try_trans_inty_macro(node: &::ppmac::Node) -> Option<(String, String)> {
    use ::ppmac::{Node, Signed, Size, UnaryOp};
    use self::try_trans_inty_macro as ttim;

    debug!("try_trans_inty_macro({:?})", node);

    match *node {
        Node::Call { ref subject, ref args } => match **subject {
            Node::Ident(ref s) => match (&**s, args.len()) {
                ("TEXT", 1) => ttim(&args[0]),
                _ => {debug!("ttim: unknown call ident"); None}
            },
            _ => {debug!("ttim: non-ident call subject"); None}
        },
        Node::Cast { ref ty, ref value } => {
            match **ty {
                Node::Type(ref name, ptr) => {
                    let ptr = if ptr { "*mut " } else { "" };
                    let ty = format!("{}{}", ptr, name);
                    ttim(value)
                        .map(|(value, _)| (format!("{} as {}", value, ty), ty))
                },
                _ => None
            }
        },
        Node::Integer(v, signed, size) => {
            Some(match (signed, size) {
                (Signed::No, Size::Unknown) => (format!("{:x}u32", v), "u32".into()),
                (Signed::No, Size::Long) => (format!("{:x}u64", v), "u64".into()),
                (Signed::Yes, Size::Unknown) => (format!("{:x}i32", v as i64), "i32".into()),
                (Signed::Yes, Size::Long) => (format!("{:x}i64", v as i64), "i64".into()),
            })
        },
        Node::String(ref s, _) => Some((format!("\"{}\"", s), "&'static str".into())),
        Node::Unary(UnaryOp::Com, ref expr) => ttim(expr).map(|(expr, ty)| (format!("!{}", expr), ty)),
        Node::Unary(UnaryOp::Neg, ref expr) => ttim(expr).map(|(expr, ty)| (format!("-{}", expr), ty)),
        ref node => {debug!("ttim: unsupported node: {:?}", node); None}
    }
}
