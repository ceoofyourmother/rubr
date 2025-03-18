use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "Vixi" => "Err",
        "Tudocorreto" => "Ok",
        "linguicaodebits" => "String", //Fabio Akita references
        "tabelinha" => "HashMap",
        "padraozao" => "Default",
        "errofudido" => "Error",
        "Opcao" => "Option",
        "Qualquer" => "Some",
        "Nada" => "None",
        "Resultado" => "Result",
        "global" => "Self",
        "printaisso" => "println",
        "paraessaporra" => "break",
        "assincrono" => "async",
        "espera" => "await",
        "naoparanao"  => "loop",
        "move"  => "move",
        "pacotes" => "crate",
        "codigo_inacessivel" => "unreachable_code",
        "como" => "as",
        "constante" => "const",
        "contrato" => "trait",
        "issovaidarmerda" => "unsafe",
        "em" => "in",
        "de" => "from",
        "dyn" => "dyn",
        "desempacotar" => "unwrap",
        "Padrao" => "Default",
        "padrao" => "default",
        "como_referencia" => "as_ref",
        "io" => "io",
        "externo" => "extern",
        "falso"  => "false",
        "funcao"  => "fn",
        "superman" => "super",
        "meteisso" => "insert",
        "pegaisso" => "get",
        "permitir" => "allow",
        "Panico"  => "panic",
        "modulo"  => "mod",
        "mutavel" => "mut",
        "novo" => "new",
        "aonde" => "where",
        "para" => "for",
        "pega_ou_inseri_em" => "get_or_insert_with",
        "principal" => "main",
        "geralvaiver" => "pub",
        "none" => None?,
        "retornaessaporra" => "return",
        "implementacao" => "impl",
        "ref"  => "ref",
        "tabatendo"  => "match",
        "se" => "if",
        "senao"  => "else",
        "proprio" => "self",
        "salvar" => "let",
        "estatico" => "static",
        "estrutura"  => "struct",
        "toexperando" => "expect",
        "enquanto" => "while",
        "usar" => "use",
        "dentro" => "into",
        "verdadeiro"  => "true",
        "vaisefuderenum" => "enum", //nao sei que nome dar pro enum nn
        _ => &ident_str,
    };

    let new_ident = Ident::new(new_str, ident.span());
    Some(TokenTree::Ident(new_ident))
}

fn replace_tree(tok: TokenTree, out: &mut Vec<TokenTree>) {
    match tok {
        TokenTree::Group(group) => {
            let mut group_elem = Vec::new();
            replace_stream(group.stream(), &mut group_elem);
            let mut new_stream = TokenStream::new();
            new_stream.extend(group_elem);
            out.push(TokenTree::Group(Group::new(group.delimiter(), new_stream)));
        }
        TokenTree::Ident(ident) => {
            if let Some(ident) = replace_ident(ident) {
                out.push(ident);
            }
        }
        TokenTree::Punct(..) | TokenTree::Literal(..) => {
            out.push(tok);
        }
    }
}

fn replace_stream(ts: TokenStream, out: &mut Vec<TokenTree>) {
    for tok in ts {
        replace_tree(tok, out)
    }
}

#[proc_macro]
pub fn rustrj(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}
