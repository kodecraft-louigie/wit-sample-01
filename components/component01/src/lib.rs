wit_bindgen::generate!();

use exports::wasi::http::incoming_handler::Guest;
use serde::{Deserialize, Serialize};
use wasi::http::types::*;

struct HttpServer;

impl Guest for HttpServer {
    fn handle(_request: IncomingRequest, response_out: ResponseOutparam) {       
        let pong = local::greeter_demo::greet::greetings("args");
        let ret = Response {
            instrument: pong.instrument_name,
            ask_iv: pong.ask_iv,
            best_ask_amount: pong.best_ask_amount,
            best_ask_price: pong.best_ask_price,
            bid_iv: pong.bid_iv
        };
        let json_string = serde_json::to_string(&ret).unwrap();
        
        let h: Headers = Fields::new();
        let _ = h.set(&"content-Type".to_string(), &["application/json".to_string().into_bytes()]);
        let response = OutgoingResponse::new(h);
        response.set_status_code(200).unwrap();       
        let response_body = response.body().unwrap();
        ResponseOutparam::set(response_out, Ok(response));
        response_body
            .write()
            .unwrap()
            .blocking_write_and_flush(json_string.as_bytes())
            .unwrap();
        OutgoingBody::finish(response_body, None).expect("failed to finish response body");
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub instrument: String,
    pub ask_iv: f64,
    pub best_ask_amount: f64,
    pub best_ask_price: f64,
    pub bid_iv: f64
}

export!(HttpServer);
