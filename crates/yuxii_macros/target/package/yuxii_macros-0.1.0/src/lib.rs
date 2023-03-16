#![crate_type = "proc-macro"]
extern crate proc_macro;



use proc_macro2::{Ident, Span, Literal, Punct, TokenTree};
use proc_macro::{TokenStream};
use quote::{quote, ToTokens};
use syn::{parse_macro_input, DeriveInput, token::{Struct, Semi, Let, self}, ItemFn, AttributeArgs, ExprClosure, Stmt, Token, PatIdent, Pat, Expr, Meta, Lit, parse::Parse, Type, ext::IdentExt};


#[proc_macro_attribute]
pub fn system(
  args: TokenStream,
  item: TokenStream,
) -> TokenStream {

    //let args = parse_macro_input!(args as AttributeArgs);
   // let mut item = parse_macro_input!(item as ItemFn);
    //let block = *item.block.clone();

    //item
    quote!{}.into()
}
/*

default!{
    pub struct Struct{
        field: i32 = 10;
    }
}

*/
#[proc_macro_attribute]
pub fn query(
  args: TokenStream,
  item: TokenStream,
) -> TokenStream {

    //let filter_info = parse_macro_input!(item as FilterInfo);

    //let filter_tokens = 
    //filter_info.to_tokens().into();

    //quote!{ #filter_tokens.search()}.into()

    quote!{}.into()

}
#[derive(Debug)]
struct FilterInfo{
    tokens: Vec<FTokenInfo>
}

impl FilterInfo{
    fn to_tokens(self) -> proc_macro2::TokenStream{
        let mut tokens = quote!{};

        for ftoken_info in self.tokens{
            let ftoken_tokens = ftoken_info.to_tokens();

            tokens = quote!{ #ftoken_tokens };
        }
        

        quote!{
            Filter::new(vec![#tokens])
        }
    }
}
#[derive(Debug)]
enum FScopeInfo{
    Children(FilterInfo),
    Parent(FilterInfo),
    Child(FilterInfo),
    Current(FilterInfo),
}

impl FScopeInfo{
    fn to_tokens(self) -> proc_macro2::TokenStream{

        let mut tokens = quote!{};
        match self{
            FScopeInfo::Children(_) => todo!(),
            FScopeInfo::Parent(filter) => {
                let filter_tokens = filter.to_tokens();
                tokens = quote!{ #filter_tokens, #tokens };
            },
            FScopeInfo::Child(_) => todo!(),
            FScopeInfo::Current(_) => todo!(),
        }

        tokens
    }
}
#[derive(Debug)]
enum FTokenInfo{
    Scope(FScopeInfo),

    Has{
        is_not: bool,
        ty: Type,
    },
    Added{
        is_not: bool,
        ty: Type,
    },
    Removed{
        is_not: bool,
        ty: Type,
    },
    Changed{
        is_not: bool,
        ty: Type,
    },
    Node{
        is_not: bool,
        ty: Type,
    },
    Name{
        is_not: bool,
        ty: Type,
    },
    Or(Vec<FTokenInfo>, Vec<FTokenInfo>),
}

impl FTokenInfo{
    fn to_tokens(self) -> proc_macro2::TokenStream{

        match self{
            FTokenInfo::Scope(scope) => { scope.to_tokens() },
            FTokenInfo::Has { is_not, ty } => quote!{ FToken::has::<#ty>(#is_not) },
            FTokenInfo::Added { is_not, ty } => quote!{ FToken::added::<#ty>(#is_not) },
            FTokenInfo::Removed { is_not, ty } => todo!(),
            FTokenInfo::Changed { is_not, ty } => todo!(),
            FTokenInfo::Node { is_not, ty } => todo!(),
            FTokenInfo::Name { is_not, ty } => todo!(),
            FTokenInfo::Or(_, _) => todo!(),
        }

    }
}

fn parse_ftoken(){

}

impl Parse for FilterInfo{
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {

        let mut info = FilterInfo{tokens: Vec::new()};

        let mut current_scope = &mut info;

        while !input.is_empty(){

            let change_detection = |is_not: bool, ident: syn::Type| -> FTokenInfo{
                if input.peek(Token![-])
                {
                    let _punct = input.parse::<Punct>();
                    return FTokenInfo::Removed { is_not, ty: ident };
                } 
                else if input.peek(Token![+])
                {
                    let _punct = input.parse::<Punct>();
                    return FTokenInfo::Added { is_not, ty: ident };
                } 
                else if input.peek(Token![~])
                {
                    let _punct = input.parse::<Punct>();
                    return FTokenInfo::Changed { is_not, ty: ident };
                } 
                else 
                {
                    let _punct = input.parse::<Punct>();
                    return FTokenInfo::Has { is_not, ty: ident}
                }
            };


            if input.peek(Ident::peek_any){
                dbg!("first");
                let ident: Type = input.parse()?;
                current_scope.tokens.push(change_detection(false, ident));


            } else if input.peek2(Ident::peek_any){
                dbg!("second");
               let mut is_not = false;

                if input.peek(Token![!]){
                    is_not = true;
                }
                let _ident: Punct = input.parse()?;

                let ident: Type = input.parse()?;
                current_scope.tokens.push(change_detection(is_not, ident));
            } else {
                let ident: Type = input.parse()?;
            }
            
        }

        dbg!(&info);


        //info.tokens.push(FTokenInfo::Has { is_not: false, ty: ident });

        Ok(info)
    }
}




#[proc_macro_attribute]
pub fn system2(
  args: TokenStream,
  item: TokenStream,
) -> TokenStream {

    let args = parse_macro_input!(args as AttributeArgs);
    let mut item = parse_macro_input!(item as ItemFn);
    let block = *item.block.clone();



    for (i, stmt) in block.stmts.into_iter().enumerate(){

        if let Stmt::Semi(syn::Expr::Closure(mut closure), ..) = stmt{

            let name = format!("autogen_query_closure_{i}");

            let ident = Ident::new(&name, Span::call_site());
            
            let mut call = quote!{
                //let y = 18;
                #ident();
            };
            //dbg!(&closure.attrs);
            if closure.attrs.len() > 0{
                if let Ok(tokens) = closure.attrs[0].clone().parse_meta(){
                    if let Meta::NameValue(meta) = tokens{
                        if let Lit::Str(str) = meta.lit.clone(){
                            let query_str = str.value();
    
                            let component_ident = Ident::new(&query_str, meta.lit.span());
    
                            call = quote!{
                                //let y = 18;
                                #ident(#component_ident::default());
                            }
                            
                        }
                    }
                }
                closure.attrs.clear();
            }

            let stmt = Stmt::Local(syn::Local { 
                attrs: vec![], 
                let_token: Let::default(), 
                pat: Pat::Ident(PatIdent { 
                    attrs: vec![], 
                    by_ref: None, 
                    mutability: None, 
                    ident: ident.clone(),
                    subpat: None, 
                }),
                init: Some((
                    syn::token::Eq::default(),
                    Box::new(Expr::Closure(closure))
                )), 
                semi_token: Semi::default()
            });

            let mut deff = quote!{
                let pos = Pos::default();
            };


            item.block.stmts[i] = syn::parse(deff.into()).unwrap();
            //item.block.stmts.push(syn::parse(call.into()).unwrap());

        } 
    }
    item.into_token_stream().into()
}


#[proc_macro_derive(Resource)]
pub fn resource_derive(tokens: TokenStream) -> TokenStream{
    resource_derive_builder(tokens)
}

fn resource_derive_builder(tokens: TokenStream) -> TokenStream{
    let input:  DeriveInput = parse_macro_input!(tokens as DeriveInput);

    let vis = input.vis;
    let name = input.ident;
    let fname = format!("{}_RESOURCE", name).to_uppercase();
    let storage_name = syn::Ident::new(&fname, name.span());

    quote!{
        #[dynamic]
        #vis static mut #storage_name: #name = #name::default();

        impl ResourceInterface for #name{
            fn read() -> static_init::lazy::lesser_locked_lazy::ReadGuard<'static, Self>{
                #storage_name.read()
            }
            fn write() -> static_init::lazy::lesser_locked_lazy::WriteGuard<'static, Self>{
                #storage_name.write()
            }

        }
    }.into()
}

#[proc_macro_derive(Component)]
pub fn derive(tokens: TokenStream) -> TokenStream{
    builder(tokens)
}

fn builder(tokens: TokenStream) -> TokenStream{
    let input:  DeriveInput = parse_macro_input!(tokens as DeriveInput);

    let vis = input.vis;
    let generic = input.ident;
    let fname = format!("{}_STORAGE", generic).to_uppercase();
    let storage_name = syn::Ident::new(&fname, generic.span());
    let return_value = match generic.to_string().as_str(){
        "Name" => quote!{
            return Some(( value.0.to_string(), true));   
        },
        _ => quote!{
            return Some((format!("{:?}", value), false));   
        },
    };


    quote!{
        #[dynamic]
        #vis static mut #storage_name: Table<#generic> = 
        {
            TABLE_ACCESSERS.write().push(Box::new(#generic::default()));
            Table::<#generic>::new()
        };

        #[typetag::serde]
        impl ComponentInterface for #generic{
            fn add_entity(&self, entity: Entity){
                entity << self.clone();
            }
            fn read() -> static_init::lazy::lesser_locked_lazy::ReadGuard<'static, Table<Self>>{
                #storage_name.read()
            }
            fn write() -> static_init::lazy::lesser_locked_lazy::WriteGuard<'static, Table<Self>>{
                #storage_name.write()
            }

        }

         impl Not for #generic {
            type Output = ExportedModifier;
        
            fn not(self) -> Self::Output {
                ExportedModifier(Modifier::Component(true, Box::new(self)))
            }
        }
        impl Shl<#generic> for Entity {
            type Output = Self;
        
            fn shl(self, rhs: #generic) -> Self::Output {
                self.add(rhs)
            }
        }


        impl Sub<#generic> for Entity{
            type Output = Entity;
    
            fn sub(self, _rhs: #generic) -> Self::Output {
                self.remove::<#generic>()
            }
        }

        impl Shl<#generic> for Scene {
            type Output = Self;
        
            fn shl(mut self, rhs: #generic) -> Self::Output {
                self.push_component(rhs, false);
                self
            }
        }

        impl Shl<#generic> for SEntity {
            type Output = Scene;
        
            fn shl(mut self, rhs: #generic) -> Self::Output {
                let mut scene = Scene::new();
                scene.push_component(rhs, false);
                scene
                
            }
        }

        impl Shl<#generic> for SNode {
            type Output = Self;
        
            fn shl(mut self, rhs: #generic) -> Self::Output {
                self.add_modifier(Modifier::Component(false, Box::new(rhs)));
                self
            }
        }

    }.into()
}


#[proc_macro_derive(Plug)]
pub fn derive2(tokens: TokenStream) -> TokenStream{
    plugin_builder(tokens)
}

fn plugin_builder(tokens: TokenStream) -> TokenStream{
    let input:  DeriveInput = parse_macro_input!(tokens as DeriveInput);

    let name = input.ident;


    quote!{

        impl Plug for #name{}

    }.into()
}


#[proc_macro_derive(Bundle)]
pub fn derive_bundle(tokens: TokenStream) -> TokenStream{
    bundle_builder(tokens)
}

fn bundle_builder(tokens: TokenStream) -> TokenStream{
    let input:  DeriveInput = parse_macro_input!(tokens as DeriveInput);

    let name = input.ident;

    let mut addition = quote!{ };

    if let syn::Data::Struct(data) = input.data{
        for field in data.fields{

            let ident = field.ident.unwrap();
            addition = quote!{
                #addition
                //self.add(rhs.#ident);
                vec.push(Box::new(self.#ident.clone()));
            }
        }
    }
    

    quote!{
        #[typetag::serde]
        impl BundleMarker for #name{
            fn to_components(&self) -> Vec<Box<dyn ComponentInterface>>{
                let mut vec: Vec<Box<dyn ComponentInterface>> = Vec::new();
                #addition
                vec
            }
            fn name(&self) -> String{

                format!(stringify!(#name))
            }
        }
        
        #[typetag::serde]
        impl ComponentInterface for #name{
            fn add_entity(&self, entity: Entity){
                entity << self.clone();
            }
            fn read() -> static_init::lazy::lesser_locked_lazy::ReadGuard<'static, Table<Self>>{
                //#storage_name.read()
                todo!();
            }
            fn write() -> static_init::lazy::lesser_locked_lazy::WriteGuard<'static, Table<Self>>{
                todo!();
            }
         }

         impl Not for #name {
            type Output = ExportedModifier;
        
            fn not(self) -> Self::Output {
                ExportedModifier(Modifier::Bundle(true, Box::new(self)))
            }
        }

        // impl Shl<#name> for Entity {
        //     type Output = Self;
        
        //     fn shl(self, rhs: #name) -> Self::Output {
        //         //#addition
        //         for component in rhs.to_components().iter(){
        //             self << component.as_mut();
        //         }
        //         self
        //     }
        // }
        impl Shl<#name> for Scene {
            type Output = Self;
        
            fn shl(mut self, rhs: #name) -> Self::Output {
                //#addition
                self.push_bundle(rhs, false);
                // for component in rhs.to_components().iter(){
                //     //self.add(component);
                //     self.bundles.push(*component.clone());
                // }
                self
            }
        }

        impl BitXor<#name> for Scene {
            type Output = Self;
    
            fn bitxor(mut self, rhs: #name) -> Self::Output {
                self.push_bundle(rhs, true);
                // for component in rhs.to_components().iter(){
                //     //self.add(component);
                //     self.bundles.push(*component.clone());
                // }
                self
            }
        }

    }.into()
}



/*

        
*/

