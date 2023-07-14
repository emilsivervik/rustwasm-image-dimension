use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, punctuated::Punctuated, token::Comma, Ident, ItemFn};
use wasm_bindgen_macro_support;

pub fn expand_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attrs: Punctuated<Ident, Comma> =
        parse_macro_input!(attr with Punctuated::parse_terminated);

    enum HandlerType {
        Fetch,
        Scheduled,
    }
    use HandlerType::*;

    let mut handler_type = None;
    let mut respond_with_errors = false;

    for attr in attrs {
        match attr.to_string().as_str() {
            "fetch" => handler_type = Some(Fetch),
            "scheduled" => handler_type = Some(Scheduled),
            "respond_with_errors" => {
                respond_with_errors = true;
            }
            _ => panic!("Invalid attribute: {}", attr.to_string()),
        }
    }
    let handler_type = handler_type
        .expect("must have either 'fetch' or 'scheduled' attribute, e.g. #[event(fetch)]");

    // create new var using syn item of the attributed fn
    let mut input_fn = parse_macro_input!(item as ItemFn);

    match handler_type {
        Fetch => {
            // TODO: validate the inputs / signature
            // save original fn name for re-use in the wrapper fn
            let input_fn_ident = Ident::new(
                &(input_fn.sig.ident.to_string() + "_fetch_glue"),
                input_fn.sig.ident.span(),
            );
            let wrapper_fn_ident = Ident::new("fetch", input_fn.sig.ident.span());
            // rename the original attributed fn
            input_fn.sig.ident = input_fn_ident.clone();

            let error_handling = match respond_with_errors {
                true => {
                    quote! {
                        Response::error(e.to_string(), 500).unwrap().into()
                    }
                }
                false => {
                    quote! { panic!("{}", e) }
                }
            };

            // create a new "main" function that takes the worker_sys::Request, and calls the
            // original attributed function, passing in a converted worker::Request
            let wrapper_fn = quote! {
                pub async fn #wrapper_fn_ident(req: worker_sys::Request, env: ::worker::Env) -> worker_sys::Response {
                    // get the worker::Result<worker::Response> by calling the original fn
                    match #input_fn_ident(worker::Request::from(req), env).await.map(worker_sys::Response::from) {
                        Ok(res) => res,
                        Err(e) => {
                            ::worker::console_log!("{}", &e);
                            #error_handling
                        }
                    }
                }
            };
            let wasm_bindgen_code =
                wasm_bindgen_macro_support::expand(TokenStream::new().into(), wrapper_fn)
                    .expect("wasm_bindgen macro failed to expand");

            let output = quote! {
                #input_fn

                #wasm_bindgen_code
            };

            TokenStream::from(output)
        }
        Scheduled => {
            // TODO: validate the inputs / signature
            // save original fn name for re-use in the wrapper fn
            /* let wrapper_fn_ident = input_fn.sig.ident.clone();
            let input_fn_ident = Ident::new(&("scheduled"), input_fn.sig.ident.span()); */

            let input_fn_ident = Ident::new(
                &(input_fn.sig.ident.to_string() + "_scheduled_glue"),
                input_fn.sig.ident.span(),
            );
            let wrapper_fn_ident = Ident::new("scheduled", input_fn.sig.ident.span());

            input_fn.sig.ident = input_fn_ident.clone();

            let error_handling = quote! { panic!("{}", e) };
            // rename the original attributed fn
            // println!("{:?}", worker_sys::Scheduled::from);

            /* pub fn _fetch(&mut self, req: worker_sys::Request) -> js_sys::Promise {
                // SAFETY:
                // On the surface, this is unsound because the Durable Object could be dropped
                // while JavaScript still has possession of the future. However,
                // we know something that Rust doesn't: that the Durable Object will never be destroyed
                // while there is still a running promise inside of it, therefore we can let a reference
                // to the durable object escape into a static-lifetime future.
                let static_self: &'static mut Self = unsafe { &mut *(self as *mut _) };

                wasm_bindgen_futures::future_to_promise(async move {
                    static_self
                        ._fetch_raw(req.into())
                        .await
                        .map(worker_sys::Response::from)
                        .map(wasm_bindgen::JsValue::from)
                        .map_err(wasm_bindgen::JsValue::from)
                })
            } */

            let wrapper_fn = quote! {
                pub async fn #wrapper_fn_ident(event: worker_sys::Scheduled , env: ::worker::Env, ctx: worker_sys::ScheduledContext) {

                    let lel = &wasm_bindgen_futures::future_to_promise(async move {
                        match #input_fn_ident(worker::Scheduled::from(event), env).await {
                            Ok(_) => (),
                            Err(e) => {
                                ::worker::console_log!("{}", &e);
                                #error_handling
                            }
                        };

                        Ok(wasm_bindgen::JsValue::UNDEFINED)
                    });
                    ctx.wait_until(lel);


                    // #input_fn_ident(worker::Scheduled::from(event), env).await
                    /* match #input_fn_ident(worker::Scheduled::from(event), env).await {
                        Ok(_) => (),
                        Err(e) => {
                            ::worker::console_log!("{}", &e);
                            #error_handling
                        }
                    }; */

                    // Ok(())

                    /* let fn = match #input_fn_ident(worker::Scheduled::from(event), env).await {
                        Ok(v) => (),
                        Err(e) => {
                            ::worker::console_log!("{}", &e);
                            #error_handling
                        }
                    };
                     */
                    // js_sys::Promise::resolve(#input_fn_ident(worker::Scheduled::from(event), env).await)

                    // ctx.waitUntil(#input_fn_ident(worker::Scheduled::from(event), env).await)
                    // ()
                    /* let static_self: &'static mut Self = unsafe {&mut *(self as *mut _)};

                    wasm_bindgen_futures::future_to_promise(async move {
                        match static_self._scheduled_glue(req.into()).await {
                            Ok(v) => v,
                            Err(e) => {
                                ::worker::console_log!("{}", &e);
                                #error_handling
                            }
                            // _ => ()
                        };
                    }) */

                    /* wasm_bindgen_futures::future_to_promise(async move {
                        match #input_fn_ident(worker::Scheduled::from(event), env).await{
                            Ok(v) => v,
                            Err(e) => {
                                ::worker::console_log!("{}", &e);
                                #error_handling
                            }
                            // _ => ()
                        };
                    }) */
                    /* match #input_fn_ident(worker::Scheduled::from(event), env).await {
                        Err(e) => {
                            ::worker::console_log!("{}", &e);
                            #error_handling
                        }
                        _ => ()
                    } */
                }
            };
            println!("{:?}", input_fn.sig.ident);
            println!("{:?}", wrapper_fn_ident);
            println!("{:?}", input_fn_ident);
            let wasm_bindgen_code =
                wasm_bindgen_macro_support::expand(TokenStream::new().into(), wrapper_fn)
                    .expect("wasm_bindgen macro failed to expand");

            let output = quote! {
                #input_fn

                #wasm_bindgen_code
            };

            TokenStream::from(output)
        }
    }
}
