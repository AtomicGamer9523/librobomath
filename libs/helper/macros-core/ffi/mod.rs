use crate::TokenStream;

/// Simplifies the creation of FFI functions
///
/// # Example
/// ```rust,ignore,no_run
/// #[macros::ffi(type = "system")]
/// pub fn Java_Main_greet<'a>(
///     mut env: JNIEnv<'a>, _class: JClass<'a>, input: JString<'a>
/// ) -> jstring {
///     // First, we have to get the string out of Java. Check out the `strings`
///     // module for more info on how this works.
///     let input: String = env.get_string(&input).expect("Couldn't get java string!").into();
///
///     // Then we have to create a new Java string to return. Again, more info
///     // in the `strings` module.
///     let output = env.new_string(
///         format!("Hello, {}!", input)
///     ).expect("Couldn't create java string!");
///
///     // Finally, extract the raw pointer to return.
///     output.into_raw()
/// }
/// ```
pub fn ffi<T: Into<TokenStream>>(cfg: T, input: T) -> TokenStream {
    let settings = parse_settings(cfg.into());
    let res = parse_func(input.into(), settings.2).to_string();
    let res = res.replace("__FFI_RAW_MODIFIERS__", &settings.0);
    let res = res.replace("__FFI_EXTERN_MODIFIER__", &settings.1);
    match syn::parse_str::<TokenStream>(&res) {
        Ok(res) => res,
        Err(err) => err.to_compile_error(),
    }
}

/// parses the function it should be attached to
fn parse_func(item: TokenStream, name_override: Option<proc_macro2::Ident>) -> TokenStream {
    let input = match syn::parse2::<syn::ItemFn>(item) {
        Ok(input) => input,
        Err(err) => {
            return err.to_compile_error();
        }
    };
    let ret = &input.sig.output;
    let inputs = &input.sig.inputs;
    let name = name_override.unwrap_or(input.sig.ident);
    let generics = &input.sig.generics;
    let body = &input.block;
    let attrs = &input.attrs;
    let vis = &input.vis;
    let result = quote::quote! {
        #(#attrs)*
        #[no_mangle]
        #vis __FFI_RAW_MODIFIERS__ extern __FFI_EXTERN_MODIFIER__ fn #name #generics(#inputs) #ret {
            #body
        }
    };
    result
}

/// parses the settings (can be none, const, unsafe or both)
fn parse_settings(attr: TokenStream) -> (String, String, Option<proc_macro2::Ident>) {
    let modifiers_str = attr.to_string();
    let mut name_override = None;
    let mut res = String::new();
    if modifiers_str.contains("const") {
        res.push_str("const ");
    }
    if modifiers_str.contains("unsafe") {
        res.push_str("unsafe ");
    }
    if modifiers_str.contains('@') {
        let type_str = modifiers_str.split('@').collect::<Vec<&str>>()[1];
        let type_str = type_str.trim();
        let mut namespace = String::new();
        let export_name;
        if type_str.contains('+') {
            let type_str = type_str.split('+').collect::<Vec<&str>>();
            namespace = type_str[0].to_string();
            export_name = type_str[1].to_string();
        } else {
            export_name = type_str.to_string();
        }
        let mut export_name = export_name.replace('.', "_");
        if !namespace.is_empty() {
            export_name = format!("{}{}", namespace, export_name)
        }
        export_name = export_name.replace(' ', "");
        name_override = Some(syn::parse_str::<proc_macro2::Ident>(&export_name).unwrap());
    };
    let t = if modifiers_str.contains("type=") {
        let type_str = modifiers_str.split("type=").collect::<Vec<&str>>()[1];
        type_str.split(' ').collect::<Vec<&str>>()[0]
    } else {
        "C"
    };
    (res, format!("\"{}\"", t), name_override)
}
