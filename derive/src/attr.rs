use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{parse::ParseStream, spanned::Spanned, Attribute, Error, Ident, LitInt, Result};

#[derive(Debug)]
pub struct Attrs<'a> {
    pub status: Option<ResolveStatus<'a>>,
    pub error_code: Option<ErrorCode<'a>>,
}

#[derive(Clone, Debug)]
pub enum ResolveStatus<'a> {
    Transparent(&'a Attribute),
    Fixed(Status<'a>),
}

#[derive(Clone, Debug)]
pub struct Status<'a> {
    pub original: &'a Attribute,
    pub code: StatusCode,
}

#[derive(Clone, Debug)]
pub enum StatusCode {
    Value(http::StatusCode),
    Name(Ident),
}

#[derive(Clone, Debug)]
pub struct ErrorCode<'a> {
    pub original: &'a Attribute,
    pub error_code: String,
}

mod kw {
    syn::custom_keyword!(transparent);
}

impl ToTokens for StatusCode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            StatusCode::Value(v) => {
                let value = v.as_u16();
                tokens.extend(quote! { ::actix_web::http::StatusCode::from_u16(#value).unwrap() });
            }
            StatusCode::Name(ident) => {
                tokens.extend(quote! { ::actix_web::http::StatusCode::#ident })
            }
        }
    }
}

impl ToTokens for Status<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.code.to_tokens(tokens);
    }
}

impl Attrs<'_> {
    pub fn get(input: &[Attribute]) -> Result<Attrs<'_>> {
        let mut attrs = Attrs {
            status: None,
            error_code: None,
        };

        for attr in input {
            if attr.path.is_ident("status") {
                attrs.parse_status_attribute(attr)?;
            }

            if attr.path.is_ident("error_code") {
                attrs.parse_error_code_attribute(attr)?;
            }
        }

        Ok(attrs)
    }

    pub fn span(&self) -> Option<Span> {
        self.status.as_ref().map(|st| match st {
            ResolveStatus::Transparent(t) => t.span(),
            ResolveStatus::Fixed(fix) => fix.original.span(),
        })
    }
}

impl<'a> Attrs<'a> {
    fn parse_status_attribute(&mut self, attr: &'a Attribute) -> Result<()> {
        if self.status.is_some() {
            return Err(Error::new_spanned(
                attr,
                "duplicate #[status(..)] attribute",
            ));
        }

        attr.parse_args_with(|input: ParseStream| {
            if input.parse::<Option<kw::transparent>>()?.is_some() {
                self.status = Some(ResolveStatus::Transparent(attr));
                return Ok(());
            }

            let status = Status {
                original: attr,
                code: parse_status_expr(input)?,
            };
            self.status = Some(ResolveStatus::Fixed(status));
            Ok(())
        })
    }

    fn parse_error_code_attribute(&mut self, attr: &'a Attribute) -> Result<()> {
        if self.error_code.is_some() {
            return Err(Error::new_spanned(
                attr,
                "duplicate #[error_code(..)] attribute",
            ));
        }

        attr.parse_args_with(|input: ParseStream| {
            let error_code = ErrorCode {
                original: attr,
                error_code: parse_error_code_expr(input)?,
            };
            self.error_code = Some(error_code);
            Ok(())
        })
    }
}

fn parse_status_expr(input: ParseStream) -> Result<StatusCode> {
    match input.parse::<Option<LitInt>>()? {
        Some(lit) => http::status::StatusCode::from_u16(lit.base10_parse::<u16>()?)
            .map_err(|e| Error::new_spanned(lit.token(), e))
            .map(StatusCode::Value),
        None => input.parse::<Ident>().map(StatusCode::Name),
    }
}

fn parse_error_code_expr(input: ParseStream) -> Result<String> {
    let ident = input.parse::<Ident>()?;
    Ok(ident.to_string())
}
