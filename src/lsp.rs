use lsp_server::{Connection, Message, Request, RequestId, Response};
use lsp_types::{request::GotoDefinition, GotoDefinitionResponse, Location, Range};

use log::info;

pub fn run() -> anyhow::Result<()> {
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

pub fn main_loop(connection: Connection) -> anyhow::Result<()> {
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

                        let uri = params.text_document_position_params.text_document.uri;
                        let mut position = params.text_document_position_params.position;

                        position.line = 0;

                        let location = Location {
                            uri,
                            range: Range {
                                start: position,
                                end: position,
                            },
                        };

                        let result = Some(GotoDefinitionResponse::Scalar(location));
                        let result = serde_json::to_value(&result).unwrap();
                        let resp = Response {
                            id,
                            result: Some(result),
                            error: None,
                        };
                        connection.sender.send(Message::Response(resp))?;
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
