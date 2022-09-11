extern crate proc_macro;

use convert_case::Casing;
use proc_macro::TokenStream;


#[proc_macro]
pub fn mem_value(item: TokenStream) -> TokenStream {
    // get_set_value!(health, f64, CURRENT_HEALTH_OFFSETS, true)
    let item = item.to_string();
    let mut item = item.split_terminator(',');
    let name_str = item.next().unwrap().trim();
    let value_type_str = item.next().unwrap().trim();
    let offsets_str = item.next().unwrap().trim();
    let add_set_function_str = item.next().unwrap().trim();

    let address_func_name = syn::Ident::new(&format!("{}_address", name_str), proc_macro2::Span::call_site());
    let get_func_name = syn::Ident::new(&format!("get_{}", name_str), proc_macro2::Span::call_site());
    let set_func_name = syn::Ident::new(&format!("set_{}", name_str), proc_macro2::Span::call_site());
    let value_type = syn::Type::Verbatim(syn::parse_str(value_type_str).unwrap());
    let offsets = syn::Item::Verbatim(syn::parse_str(offsets_str).unwrap());
    let add_set_function: bool = add_set_function_str.parse().unwrap();
    let http_get_func_name = syn::Ident::new(&format!("http_get_{}", name_str), proc_macro2::Span::call_site());
    let http_set_func_name = syn::Ident::new(&format!("http_set_{}", name_str), proc_macro2::Span::call_site());
    let response_struct_name_str = format!("get_{}_response", name_str).to_case(convert_case::Case::UpperCamel);
    let response_struct_name = syn::Ident::new(&response_struct_name_str, proc_macro2::Span::call_site());
    let request_struct_name_str = format!("set_{}_request", name_str).to_case(convert_case::Case::UpperCamel);
    let request_struct_name = syn::Ident::new(&request_struct_name_str, proc_macro2::Span::call_site());
    let variable_name = syn::Ident::new(name_str, proc_macro2::Span::call_site());

    let output = if add_set_function {
        quote::quote!{
            fn #address_func_name(process_memory: &vmemory::ProcessMemory) -> usize {
                #offsets.fetch_address(process_memory)
            }
    
            fn #get_func_name(process: &vmemory::ProcessMemory) -> #value_type {
                let address = #address_func_name(process);
                let value = #value_type::from_le_bytes(process.read_memory(address, 8, false).try_into().unwrap());
                tracing::info!("Read value {} from address {:x} ({})", value, address, stringify!(#get_func_name));
                value
            }
    
            fn #set_func_name(process: &vmemory::ProcessMemory, value: #value_type) {
                let address = #address_func_name(process);
                let bytes = value.to_le_bytes().to_vec();
                process.write_memory(address, &bytes, false);
                tracing::info!("Wrote value {} to address {:x} ({})", value, address, stringify!(#set_func_name));
            }

            #[derive(serde::Serialize, Debug)]
            #[serde(rename_all = "camelCase")]
            struct #response_struct_name {
                #variable_name: #value_type
            }

            #[derive(serde::Deserialize, Debug)]
            #[serde(rename_all = "camelCase")]
            struct #request_struct_name {
                #variable_name: #value_type
            }

            async fn #http_get_func_name(process_extension: axum::Extension<std::sync::Arc<self::UndertaleGame>>) -> impl axum::response::IntoResponse {
                let process = process_extension.process.lock().await;
                let #variable_name = #get_func_name(&process);

                (axum::http::StatusCode::OK, axum::Json(#response_struct_name { #variable_name })).into_response()
            }

            async fn #http_set_func_name(
                process_extension: axum::Extension<std::sync::Arc<self::UndertaleGame>>,
                axum::Json(body): axum::Json<#request_struct_name>
            ) -> impl axum::response::IntoResponse {
                let process = process_extension.process.lock().await;
                #set_func_name(&process, body.#variable_name);

                (axum::http::StatusCode::OK, axum::Json(serde_json::json!({"status": "ok"}))).into_response()
            }
        }.into()
    } else {
        quote::quote!{
            fn #address_func_name(process_memory: &vmemory::ProcessMemory) -> usize {
                #offsets.fetch_address(process_memory)
            }
    
            fn #get_func_name(process: &vmemory::ProcessMemory) -> #value_type {
                let address = #address_func_name(process);
                let value = #value_type::from_le_bytes(process.read_memory(address, 8, false).try_into().unwrap());
                tracing::info!("Read value {} from address {:x} ({})", value, address, stringify!(#get_func_name));
                value
            }

            #[derive(serde::Serialize, Debug)]
            #[serde(rename_all = "camelCase")]
            struct #response_struct_name {
                #variable_name: #value_type
            }

            async fn #http_get_func_name(process_extension: axum::Extension<std::sync::Arc<self::UndertaleGame>>) -> impl axum::response::IntoResponse {
                let process = process_extension.process.lock().await;
                let #variable_name = #get_func_name(&process);

                (axum::http::StatusCode::OK, axum::Json(#response_struct_name { #variable_name })).into_response()
            }
        }.into()
    };

    output
}

#[proc_macro]
pub fn mem_value_structs(item: TokenStream) -> TokenStream {
    // mem_value_structs!(health, f64)
    let item = item.to_string();
    let mut item = item.split_terminator(',');
    let name_str = item.next().unwrap().trim();
    let value_type_str = item.next().unwrap().trim();

    let response_struct_name_str = format!("get_{}_response", name_str).to_case(convert_case::Case::UpperCamel);
    let response_struct_name = syn::Ident::new(&response_struct_name_str, proc_macro2::Span::call_site());
    let request_struct_name_str = format!("set_{}_request", name_str).to_case(convert_case::Case::UpperCamel);
    let request_struct_name = syn::Ident::new(&request_struct_name_str, proc_macro2::Span::call_site());
    let variable_name = syn::Ident::new(name_str, proc_macro2::Span::call_site());
    let value_type = syn::Type::Verbatim(syn::parse_str(value_type_str).unwrap());

    let output = quote::quote!{
        #[derive(serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct #response_struct_name {
            #variable_name: #value_type
        }

        #[derive(serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct #request_struct_name {
            #variable_name: #value_type
        }
    }.into();

    output
}