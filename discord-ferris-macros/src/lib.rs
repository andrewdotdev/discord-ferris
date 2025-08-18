use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{FnArg, ItemFn, PatType, Type, parse_macro_input, spanned::Spanned};

/// #[event_handler] on_* free functions
#[proc_macro_attribute]
pub fn event_handler(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let fn_vis = &input.vis;
    let fn_sig = &input.sig;
    let fn_block = &input.block;
    let fn_name = &fn_sig.ident;

    let name_str = fn_name.to_string();
    if !name_str.starts_with("on_") {
        return syn::Error::new(
            fn_name.span(),
            "expected fn name starting with `on_` (e.g. on_ready)",
        )
        .to_compile_error()
        .into();
    }

    // event name: on_message_create -> MessageCreate
    let event_snake = &name_str["on_".len()..];
    let event_pascal = snake_to_pascal(event_snake);
    let event_ident = syn::Ident::new(&event_pascal, fn_name.span());

    // Infer payload type by default: Gateway{Event}DispatchData
    let inferred_ty_ident = format_ident!("Gateway{}DispatchData", event_pascal);
    let inferred_ty = quote!(::discord_ferris::models::gateway::#inferred_ty_ident);

    // parameters: allow (ctx) or (ctx, payload)
    let mut has_payload = false;
    let mut payload_ty: Option<proc_macro2::TokenStream> = None;

    match fn_sig.inputs.len() {
        1 => {
            // ok: (ctx)
        }
        2 => {
            has_payload = true;
            let second = fn_sig.inputs.iter().nth(1).unwrap();
            match second {
                FnArg::Typed(PatType { ty, .. }) => {
                    // If user wrote a concrete type, use it; otherwise infer.
                    payload_ty = Some(match &**ty {
                        Type::Infer(_) => {
                            // fn param types cannot be `_`; guide the user.
                            return syn::Error::new(
                                ty.span(),
                                "type `_` is not allowed in fn params; omit the payload param or write its concrete type",
                            )
                            .to_compile_error()
                            .into();
                        }
                        _ => quote!(#ty),
                    });
                }
                FnArg::Receiver(_) => {
                    return syn::Error::new(
                        second.span(),
                        "handler must be a free fn, not a method",
                    )
                    .to_compile_error()
                    .into();
                }
            }
        }
        _ => {
            return syn::Error::new(
                fn_sig.inputs.span(),
                "expected signature like: async fn on_<event>(ctx[, payload])",
            )
            .to_compile_error()
            .into();
        }
    }

    // If user didn't type the payload, infer it from the event.
    let payload_ty = payload_ty.unwrap_or_else(|| {
        if event_pascal == "Resumed" {
            quote!(())
        } else {
            inferred_ty.clone()
        }
    });

    // Shim name
    let shim_ident = format_ident!("__df_shim_{}", fn_name);

    // Shim body depends on arity and on Resumed (unit)
    let shim_call = if has_payload {
        if event_pascal == "Resumed" {
            // Never try to serde () for Resumed; call with unit directly.
            quote! {
                Box::pin(async move {
                    #fn_name(ctx, ()).await
                })
            }
        } else {
            quote! {
                Box::pin(async move {
                    match ::serde_json::from_value::<#payload_ty>(raw) {
                        Ok(payload) => #fn_name(ctx, payload).await,
                        Err(_) => ::discord_ferris::log!("WARN", "failed to decode payload for {}", #name_str),
                    }
                })
            }
        }
    } else {
        // No payload in user fn; just call it.
        quote! {
            Box::pin(async move { #fn_name(ctx).await })
        }
    };

    // Expand: original fn + shim + inventory submit
    let expanded = quote! {
        #fn_vis #fn_sig #fn_block

        #[doc(hidden)]
        fn #shim_ident(
            ctx: ::discord_ferris::framework::context::Ctx,
            raw: ::serde_json::Value,
        ) -> ::discord_ferris::framework::events::registry::DynFut {
            #shim_call
        }

        ::inventory::submit! {
            ::discord_ferris::framework::events::registry::Handler {
                event: ::discord_ferris::models::gateway::GatewayDispatchEvents::#event_ident,
                f: #shim_ident,
                name: #name_str,
            }
        }
    };
    expanded.into()
}

// --- tiny helper: snake_case -> PascalCase (no extra deps) ---
fn snake_to_pascal(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut up = true;
    for ch in s.chars() {
        if ch == '_' {
            up = true;
            continue;
        }
        if up {
            out.extend(ch.to_uppercase());
            up = false;
        } else {
            out.push(ch);
        }
    }
    out
}
