use lsp_server::{Connection, Message, Request, RequestId, Response};
use lsp_types::{request::GotoDefinition, GotoDefinitionResponse};

use crate::ctx::Context;

use log::info;
use std::cmp::Ordering;

#[derive(clap::Args, Debug)]
#[clap(about, author, version)]
pub struct Lsp {}

impl Lsp {
    pub fn call(_args: Lsp) -> crate::Result<()> {
        let context = crate::ctx::Context::new(())?;

        info!("reindexing the project");
        crate::services::reindex(&context)?;

        info!("starting up lsp server");
        let (connection, io_threads) = Connection::stdio();

        let server_capabilities = serde_json::to_value(crate::lsp::server_capabilities()).unwrap();
        info!("server capabilities: {:?}", &server_capabilities);

        connection.initialize(server_capabilities)?;
        Self::main_loop(&context, connection)?;
        io_threads.join()?;

        Ok(())
    }

    fn main_loop(context: &Context, connection: Connection) -> crate::Result<()> {
        info!("starting main loop");

        for msg in &connection.receiver {
            match msg {
                Message::Request(req) => {
                    info!("got request: {:?}", req);

                    if connection.handle_shutdown(&req)? {
                        info!("shutting down...");
                        return Ok(());
                    }

                    match Self::cast::<GotoDefinition>(req) {
                        Ok((id, params)) => {
                            info!("got gotoDefinition request #{}: {:?}", id, params);

                            let url = params.text_document_position_params.text_document.uri;
                            let position = params.text_document_position_params.position;
                            let mut references =
                                crate::lsp::find_markdown_references(context, url, position)?;

                            let response: Option<GotoDefinitionResponse> =
                                match references.len().cmp(&1) {
                                    Ordering::Greater => {
                                        Some(GotoDefinitionResponse::Array(references))
                                    }
                                    Ordering::Less => Some(GotoDefinitionResponse::Scalar(
                                        references.pop().unwrap(),
                                    )),
                                    Ordering::Equal => None,
                                };

                            let lsp_response = Response {
                                id,
                                result: Some(serde_json::to_value(&response)?),
                                error: None,
                            };
                            connection.sender.send(Message::Response(lsp_response))?;
                            continue;
                        }
                        Err(req) => req,
                    };
                }
                Message::Response(resp) => {
                    info!("got response: {:?}", resp);
                }
                Message::Notification(not) => {
                    info!("got notification: {:?}", not);
                }
            }
        }
        Ok(())
    }

    fn cast<R>(req: Request) -> std::result::Result<(RequestId, R::Params), Request>
    where
        R: lsp_types::request::Request,
        R::Params: serde::de::DeserializeOwned,
    {
        req.extract(R::METHOD)
    }
}
