use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse::ParseStream, Attribute, Error, Ident, LitInt, Result};

pub struct Attrs<'a> {
    pub status: Option<ResolveStatus<'a>>,
}

#[derive(Clone)]
pub enum ResolveStatus<'a> {
    Transparent(&'a Attribute),
    Fixed(Status<'a>),
}

#[derive(Clone)]
pub struct Status<'a> {
    pub original: &'a Attribute,
    pub code: Code,
}

#[derive(Clone)]
pub enum Code {
    Value(http::StatusCode),
    Name(Ident),
}

pub fn get(input: &[Attribute]) -> Result<Attrs> {
    let mut attrs = Attrs { status: None };

    for attr in input {
        if attr.path.is_ident("status") {
            parse_status_attribute(&mut attrs, attr)?;
        }
    }

    Ok(attrs)
}

fn parse_status_attribute<'a>(attrs: &mut Attrs<'a>, attr: &'a Attribute) -> Result<()> {
    syn::custom_keyword!(transparent);

    attr.parse_args_with(|input: ParseStream| {
        if attrs.status.is_some() {
            return Err(Error::new_spanned(
                attr,
                "duplicate #[status(..)] attribute",
            ));
        }
        if input.parse::<Option<transparent>>()?.is_some() {
            attrs.status = Some(ResolveStatus::Transparent(attr));
            return Ok(());
        }

        let status = Status {
            original: attr,
            code: parse_status_expr(input)?,
        };
        attrs.status = Some(ResolveStatus::Fixed(status));
        Ok(())
    })
}

fn parse_status_expr(input: ParseStream) -> Result<Code> {
    match input.parse::<Option<LitInt>>()? {
        Some(lit) => http::status::StatusCode::from_u16(lit.base10_parse::<u16>()?)
            .map_err(|e| Error::new_spanned(lit.token(), e))
            .map(Code::Value),
        None => input.parse::<Ident>().map(Code::Name),
    }
}

impl ToTokens for Code {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Code::Value(v) => {
                let value = v.as_u16();
                tokens.extend(quote! { ::actix_web::http::StatusCode::from_u16(#value).unwrap() });
            }
            Code::Name(ident) => tokens.extend(quote! { ::actix_web::http::StatusCode::#ident }),
        }
    }
}

impl ToTokens for Status<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.code.to_tokens(tokens);
    }
}
