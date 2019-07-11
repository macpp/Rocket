use proc_macro::{TokenStream, Diagnostic,Level};
use crate::proc_macro2::{Literal,Span};
use crate::proc_macro2::TokenStream as TokenStream2;
use devise::syn::{ItemMod,ItemStatic,parse_macro_input,parse_quote, spanned::Spanned, 
    parse::{Parse,ParseStream},
    Result,Ident,ext::IdentExt,Token};
use crate::quote::ToTokens;

pub fn auto_mount_hint_attribute(args: TokenStream,input: TokenStream) -> TokenStream {
    let mut item_mod = parse_macro_input!(input as ItemMod);
    let config = parse_macro_input!(args as AutoMountConfig);

    let mh_ident = quote!{rocket::auto_mount::AutoMountHint};
    let config_expanded = config.expand();
    let static_to_add:ItemStatic =  parse_quote! {
        //static __ROCKED_AUTO_MOUNT_HINT : #mh_ident = #mh_ident {mount_point: "/test", enabled: true};
        static __ROCKED_AUTO_MOUNT_HINT : #mh_ident = #mh_ident {#config_expanded};
    };
    if let Some(x) = &mut item_mod.content {
        x.1.push(static_to_add.into());
    }else {
       Diagnostic::spanned(mh_ident.span().unwrap(), Level::Warning,  "#[auto_mount_hint] cannot be applied to module declarations")
        .emit();
    }

    let out: TokenStream = item_mod.into_token_stream().into();
    println!("out: {}", out.clone().to_string());

    out
    
}

struct AutoMountConfig {
    pub mount_point: String,
    pub enabled: bool,
}

impl Parse for AutoMountConfig {
    //TODO: remove parsing/quoting cyccle -> change to verification
    //TODO: verify mounting poing /asdf
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        let mut config = AutoMountConfig { mount_point: "/".to_string(), enabled: true};

        if lookahead.peek(Ident::peek_any) {
            let id : Ident = input.parse()?;
            config.set_enabled(&id);
        } else /*if lookahead.peek(Literal::peek_any)*/ {
            let lit : Literal = input.parse()?;
            let mount_point = lit.to_string();
            config.mount_point = mount_point[1..mount_point.len()-1].to_string();
            println!("loaded base: {}", &config.mount_point);
            if lookahead.peek(Token![,]){
            let _commna : Token![,] = input.parse()?;
            let id : Ident = input.parse()?;
            config.set_enabled(&id);
        }
        }
        Ok(config)
    }
}

impl AutoMountConfig {
    fn set_enabled(&mut self, ident: &Ident) {
        match ident.to_string().as_str(){
            "true" => self.enabled = true,
            "false" => self.enabled = false,
            _ => panic!("invalid enablation")
        }
    }
    fn expand(self) -> TokenStream2 {
        let mount_point = Literal::string(&self.mount_point);
        println!("mount point: {}", &mount_point);
        let enabled = Ident::new(&format!("{:?}", self.enabled), Span::call_site());
        quote!{
            mount_point: #mount_point, enabled: #enabled
        }
    }
}
