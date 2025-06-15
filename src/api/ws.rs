use actix_web::{HttpRequest, Responder, web};
use actix_ws::Message;

use crate::api::dto::EventFromFrontend;

pub async fn ws_handler(req: HttpRequest, body: web::Payload) -> actix_web::Result<impl Responder> {
    let (response, mut session, mut msg_stream) = actix_ws::handle(&req, body)?;

    actix_web::rt::spawn(async move {
        while let Some(Ok(msg)) = msg_stream.recv().await {
            match msg {
                Message::Text(msg) => {
                    log::info!(
                        "Got text: {:#?}",
                        serde_json::from_str::<EventFromFrontend>(&msg.to_string()).unwrap()
                    )
                }
                Message::Close(_) => break,
                _ => {}
            }
        }

        let _ = session.close(None).await;
    });

    Ok(response)
}
