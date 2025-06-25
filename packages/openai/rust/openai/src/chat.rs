use anyhow::Result;
use sdfg::sdf;
use crate::bindings::example::openai_types::types::OpenaiRequest;
use crate::bindings::example::openai_types::types::OpenaiResponse;
#[allow(unused_imports)]
use crate::bindings::example::openai_types::types::*;
#[sdf(fn_name = "chat")]
pub(crate) fn chat(_request: OpenaiRequest) -> Result<OpenaiResponse> {
    println!("chat called - not implemented!");
    Err(anyhow::anyhow!("chat is not implemented"))
}
