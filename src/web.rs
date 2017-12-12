use tiny_http::{Server, Response};
use ::reporter::ReportData;
extern crate serde_json;

pub fn host_data(web_reporter: &ReportData) {
  let server = Server::http("0.0.0.0:8000").unwrap();

  for request in server.incoming_requests() {
    let response = Response::from_string({
      serde_json::to_string((&web_reporter)).unwrap_or("".to_string())
    });
    let _a = request.respond(response);
  }
}
