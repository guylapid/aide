use std::{borrow::Cow, rc::Rc, sync::Arc};

use crate::{
    openapi::{MediaType, Operation, RequestBody, Response},
    operation::set_body,
    OperationInput,
};
use indexmap::IndexMap;

use crate::operation::OperationOutput;

#[cfg(feature = "bytes")]
mod bytes;

#[cfg(feature = "http")]
mod http;

#[cfg(feature = "serde_qs")]
mod serde_qs;

impl<T, E> OperationInput for Result<T, E>
where
    T: OperationInput,
{
    fn operation_input(ctx: &mut crate::gen::GenContext, operation: &mut Operation) {
        T::operation_input(ctx, operation);
    }

    fn inferred_early_responses(
        ctx: &mut crate::gen::GenContext,
        operation: &mut Operation,
    ) -> Vec<(Option<u16>, Response)> {
        T::inferred_early_responses(ctx, operation)
    }
}

impl<T, E> OperationOutput for Result<T, E>
where
    T: OperationOutput,
    E: OperationOutput,
{
    fn operation_response(
        ctx: &mut crate::gen::GenContext,
        operation: &mut Operation,
    ) -> Option<Response> {
        T::operation_response(ctx, operation)
    }

    fn inferred_responses(
        ctx: &mut crate::gen::GenContext,
        operation: &mut Operation,
    ) -> Vec<(Option<u16>, Response)> {
        let mut responses = T::inferred_responses(ctx, operation);
        responses.extend(E::inferred_responses(ctx, operation));
        responses
    }
}

impl<T> OperationInput for Option<T>
where
    T: OperationInput,
{
    fn operation_input(ctx: &mut crate::gen::GenContext, operation: &mut Operation) {
        // Make parameters proudced by T optional if T is wrapped in an Option.
        // TODO: we should probably do this for the body as well.
        let mut temp_op = Operation::default();
        T::operation_input(ctx, &mut temp_op);
        T::operation_input(ctx, operation);

        if temp_op.parameters.is_empty() {
            return;
        }

        for param in &mut operation.parameters {
            if let Some(param) = param.as_item_mut() {
                let new_param = temp_op.parameters.iter().any(|p| {
                    let p = match p.as_item() {
                        Some(p) => p,
                        None => return false,
                    };

                    p.parameter_data_ref().name == param.parameter_data_ref().name
                });

                if new_param {
                    param.parameter_data_mut().required = false;
                }
            }
        }
    }
}

impl<T> OperationOutput for Option<T>
where
    T: OperationOutput,
{
    fn operation_response(
        ctx: &mut crate::gen::GenContext,
        operation: &mut Operation,
    ) -> Option<Response> {
        T::operation_response(ctx, operation)
    }

    fn inferred_responses(
        ctx: &mut crate::gen::GenContext,
        operation: &mut Operation,
    ) -> Vec<(Option<u16>, Response)> {
        T::inferred_responses(ctx, operation)
    }
}

impl<T> OperationInput for Box<T>
where
    T: OperationInput,
{
    fn operation_input(ctx: &mut crate::gen::GenContext, operation: &mut Operation) {
        T::operation_input(ctx, operation);
    }
}

impl<T> OperationOutput for Box<T>
where
    T: OperationOutput,
{
    fn operation_response(
        ctx: &mut crate::gen::GenContext,
        operation: &mut Operation,
    ) -> Option<Response> {
        T::operation_response(ctx, operation)
    }

    fn inferred_responses(
        ctx: &mut crate::gen::GenContext,
        operation: &mut Operation,
    ) -> Vec<(Option<u16>, Response)> {
        T::inferred_responses(ctx, operation)
    }
}

impl<T> OperationInput for Rc<T>
where
    T: OperationInput,
{
    fn operation_input(ctx: &mut crate::gen::GenContext, operation: &mut Operation) {
        T::operation_input(ctx, operation);
    }
}

impl<T> OperationOutput for Rc<T>
where
    T: OperationOutput,
{
    fn operation_response(
        ctx: &mut crate::gen::GenContext,
        operation: &mut Operation,
    ) -> Option<Response> {
        T::operation_response(ctx, operation)
    }

    fn inferred_responses(
        ctx: &mut crate::gen::GenContext,
        operation: &mut Operation,
    ) -> Vec<(Option<u16>, Response)> {
        T::inferred_responses(ctx, operation)
    }
}

impl<T> OperationInput for Arc<T>
where
    T: OperationInput,
{
    fn operation_input(ctx: &mut crate::gen::GenContext, operation: &mut Operation) {
        T::operation_input(ctx, operation);
    }
}

impl<T> OperationOutput for Arc<T>
where
    T: OperationOutput,
{
    fn operation_response(
        ctx: &mut crate::gen::GenContext,
        operation: &mut Operation,
    ) -> Option<Response> {
        T::operation_response(ctx, operation)
    }

    fn inferred_responses(
        ctx: &mut crate::gen::GenContext,
        operation: &mut Operation,
    ) -> Vec<(Option<u16>, Response)> {
        T::inferred_responses(ctx, operation)
    }
}

impl OperationInput for String {
    fn operation_input(ctx: &mut crate::gen::GenContext, operation: &mut Operation) {
        set_body(
            ctx,
            operation,
            RequestBody {
                description: None,
                content: IndexMap::from_iter([(
                    "text/plain; charset=utf-8".into(),
                    MediaType::default(),
                )]),
                required: true,
                extensions: IndexMap::default(),
            },
        );
    }
}

impl OperationOutput for String {
    fn operation_response(
        _ctx: &mut crate::gen::GenContext,
        _operation: &mut Operation,
    ) -> Option<crate::openapi::Response> {
        Some(Response {
            description: "plain text".into(),
            content: IndexMap::from_iter([(
                "text/plain; charset=utf-8".into(),
                MediaType::default(),
            )]),
            ..Default::default()
        })
    }

    fn inferred_responses(
        ctx: &mut crate::gen::GenContext,
        operation: &mut Operation,
    ) -> Vec<(Option<u16>, Response)> {
        if let Some(res) = Self::operation_response(ctx, operation) {
            Vec::from([(Some(200), res)])
        } else {
            Vec::new()
        }
    }
}

impl<'a> OperationOutput for &'a str {
    fn operation_response(
        ctx: &mut crate::gen::GenContext,
        operation: &mut Operation,
    ) -> Option<crate::openapi::Response> {
        String::operation_response(ctx, operation)
    }

    fn inferred_responses(
        ctx: &mut crate::gen::GenContext,
        operation: &mut Operation,
    ) -> Vec<(Option<u16>, Response)> {
        String::inferred_responses(ctx, operation)
    }
}

impl<'a> OperationOutput for Cow<'a, str> {
    fn operation_response(
        ctx: &mut crate::gen::GenContext,
        operation: &mut Operation,
    ) -> Option<crate::openapi::Response> {
        String::operation_response(ctx, operation)
    }

    fn inferred_responses(
        ctx: &mut crate::gen::GenContext,
        operation: &mut Operation,
    ) -> Vec<(Option<u16>, Response)> {
        String::inferred_responses(ctx, operation)
    }
}

impl OperationOutput for () {
    fn operation_response(
        _ctx: &mut crate::gen::GenContext,
        _operation: &mut Operation,
    ) -> Option<crate::openapi::Response> {
        Some(Response {
            description: "no content".to_string(),
            ..Default::default()
        })
    }

    fn inferred_responses(
        ctx: &mut crate::gen::GenContext,
        operation: &mut Operation,
    ) -> Vec<(Option<u16>, Response)> {
        if let Some(res) = Self::operation_response(ctx, operation) {
            Vec::from([(Some(ctx.no_content_status), res)])
        } else {
            Vec::new()
        }
    }
}

impl OperationInput for Vec<u8> {
    fn operation_input(
        ctx: &mut crate::gen::GenContext,
        operation: &mut crate::openapi::Operation,
    ) {
        set_body(
            ctx,
            operation,
            RequestBody {
                description: None,
                content: IndexMap::from_iter([(
                    "application/octet-stream".into(),
                    MediaType::default(),
                )]),
                required: true,
                extensions: IndexMap::default(),
            },
        );
    }
}

impl OperationOutput for Vec<u8> {
    fn operation_response(
        _ctx: &mut crate::gen::GenContext,
        _operation: &mut Operation,
    ) -> Option<crate::openapi::Response> {
        Some(Response {
            description: "byte stream".into(),
            content: IndexMap::from_iter([(
                "application/octet-stream".into(),
                MediaType::default(),
            )]),
            ..Default::default()
        })
    }

    fn inferred_responses(
        ctx: &mut crate::gen::GenContext,
        operation: &mut Operation,
    ) -> Vec<(Option<u16>, Response)> {
        if let Some(res) = Self::operation_response(ctx, operation) {
            Vec::from([(Some(200), res)])
        } else {
            Vec::new()
        }
    }
}

impl<'a> OperationInput for &'a [u8] {
    fn operation_input(
        ctx: &mut crate::gen::GenContext,
        operation: &mut crate::openapi::Operation,
    ) {
        Vec::<u8>::operation_input(ctx, operation);
    }
}

impl<'a> OperationOutput for &'a [u8] {
    fn operation_response(
        ctx: &mut crate::gen::GenContext,
        operation: &mut Operation,
    ) -> Option<crate::openapi::Response> {
        Vec::<u8>::operation_response(ctx, operation)
    }

    fn inferred_responses(
        ctx: &mut crate::gen::GenContext,
        operation: &mut Operation,
    ) -> Vec<(Option<u16>, Response)> {
        Vec::<u8>::inferred_responses(ctx, operation)
    }
}

impl<'a> OperationInput for Cow<'a, [u8]> {
    fn operation_input(
        ctx: &mut crate::gen::GenContext,
        operation: &mut crate::openapi::Operation,
    ) {
        Vec::<u8>::operation_input(ctx, operation);
    }
}

impl<'a> OperationOutput for Cow<'a, [u8]> {
    fn operation_response(
        ctx: &mut crate::gen::GenContext,
        operation: &mut Operation,
    ) -> Option<crate::openapi::Response> {
        Vec::<u8>::operation_response(ctx, operation)
    }

    fn inferred_responses(
        ctx: &mut crate::gen::GenContext,
        operation: &mut Operation,
    ) -> Vec<(Option<u16>, Response)> {
        Vec::<u8>::inferred_responses(ctx, operation)
    }
}
