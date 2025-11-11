use crate::openapi::{MediaType, Operation, Response, SchemaObject};
use axum::{
    response::{Html, Redirect},
    Form, Json,
};
use indexmap::IndexMap;
use schemars::{
    schema::{InstanceType, SingleOrVec},
    JsonSchema,
};

use crate::{gen::GenContext, operation::OperationOutput};

impl<T> OperationOutput for Json<T>
where
    T: JsonSchema,
{
    fn operation_response(ctx: &mut GenContext, _operation: &mut Operation) -> Option<Response> {
        let mut schema = ctx.schema.subschema_for::<T>().into_object();

        Some(Response {
            description: schema.metadata().description.clone().unwrap_or_default(),
            content: IndexMap::from_iter([(
                "application/json".into(),
                MediaType {
                    schema: Some(SchemaObject {
                        json_schema: schema.into(),
                        example: None,
                        external_docs: None,
                    }),
                    ..Default::default()
                },
            )]),
            ..Default::default()
        })
    }

    fn inferred_responses(
        ctx: &mut crate::gen::GenContext,
        operation: &mut Operation,
    ) -> Vec<(Option<u16>, Response)> {
        if let Some(res) = Self::operation_response(ctx, operation) {
            vec![(Some(200), res)]
        } else {
            Vec::new()
        }
    }
}

impl<T> OperationOutput for Form<T>
where
    T: JsonSchema,
{
    fn operation_response(ctx: &mut GenContext, _operation: &mut Operation) -> Option<Response> {
        let mut schema = ctx.schema.subschema_for::<T>().into_object();

        Some(Response {
            description: schema.metadata().description.clone().unwrap_or_default(),
            content: IndexMap::from_iter([(
                "application/x-www-form-urlencoded".into(),
                MediaType {
                    schema: Some(SchemaObject {
                        json_schema: schema.into(),
                        example: None,
                        external_docs: None,
                    }),
                    ..Default::default()
                },
            )]),
            ..Default::default()
        })
    }

    fn inferred_responses(
        ctx: &mut crate::gen::GenContext,
        operation: &mut Operation,
    ) -> Vec<(Option<u16>, Response)> {
        if let Some(res) = Self::operation_response(ctx, operation) {
            vec![(Some(200), res)]
        } else {
            Vec::new()
        }
    }
}

impl<T> OperationOutput for Html<T> {
    fn operation_response(_ctx: &mut GenContext, _operation: &mut Operation) -> Option<Response> {
        Some(Response {
            description: "HTML content".into(),
            content: IndexMap::from_iter([(
                "text/html".into(),
                MediaType {
                    schema: Some(SchemaObject {
                        json_schema: schemars::schema::SchemaObject {
                            instance_type: Some(SingleOrVec::Single(Box::new(
                                InstanceType::String,
                            ))),
                            ..Default::default()
                        }
                        .into(),
                        example: None,
                        external_docs: None,
                    }),
                    ..Default::default()
                },
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

impl OperationOutput for Redirect {
    fn operation_response(_ctx: &mut GenContext, _operation: &mut Operation) -> Option<Response> {
        Some(Response {
            description: "A redirect to the described URL".to_string(),
            ..Default::default()
        })
    }
}

#[cfg(feature = "axum-extra")]
#[allow(unused_imports)]
mod extra {
    use axum_extra::extract;

    use super::*;
    use crate::operation::OperationOutput;


    #[cfg(feature = "axum-extra-cookie")]
    impl OperationOutput for extract::CookieJar { type Inner = (); }

    #[cfg(feature = "axum-extra-cookie-private")]
    impl OperationOutput for extract::PrivateCookieJar { type Inner = (); }
}
