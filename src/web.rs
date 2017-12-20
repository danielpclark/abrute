use tiny_http::{Server, Response};
extern crate serde;
extern crate serde_json;
use ::model::report_data::*;

pub fn host_data(web_reporter: &ReportData) {
  let server = Server::http("0.0.0.0:3838").unwrap();

  for request in server.incoming_requests() {
    let response = Response::from_string({
      serde_json::to_string(&web_reporter).unwrap_or("".to_string())
    });
    let _ = request.respond(response);
  }
}


