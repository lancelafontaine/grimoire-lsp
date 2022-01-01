use lsp_server::{Connection, Message, Request, RequestId, Response};
use lsp_types::{request::GotoDefinition, GotoDefinitionResponse};

use log::info;

#[derive(clap::Args, Debug)]
#[clap(about, author, version)]
pub struct Lsp {}

pub fn run(_args: Lsp) -> crate::Result<()> {
    crate::logger::initialize_logger()?;

    info!("starting up lsp server");
    let (connection, io_threads) = Connection::stdio();

    let server_capabilities =
        serde_json::to_value(crate::server_capabilities::server_capabilities()).unwrap();
    info!("server capabilities: {:?}", &server_capabilities);

    connection.initialize(server_capabilities)?;
    main_loop(connection)?;
    io_threads.join()?;

    Ok(())
}

pub fn main_loop(connection: Connection) -> crate::Result<()> {
    info!("starting main loop");

    for msg in &connection.receiver {
        match msg {
            Message::Request(req) => {
                info!("got request: {:?}", req);

                if connection.handle_shutdown(&req)? {
                    info!("shutting down...");
                    return Ok(());
                }

                match cast::<GotoDefinition>(req) {
                    Ok((id, params)) => {
                        info!("got gotoDefinition request #{}: {:?}", id, params);

                        let url = params.text_document_position_params.text_document.uri;
                        let position = params.text_document_position_params.position;
                        let response = crate::reference::find_markdown_reference(url, position).map(|l| GotoDefinitionResponse::Scalar(l));
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
