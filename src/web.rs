// Copyright 2017-2018 Daniel P. Clark & other abrute Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use tiny_http::{Response, Server};
extern crate serde;
extern crate serde_json;
use model::report_data::*;

pub fn host_data(web_reporter: &ReportData) {
    let server = Server::http("0.0.0.0:3838").unwrap();

    for request in server.incoming_requests() {
        let response = Response::from_string({
            serde_json::to_string(&web_reporter).unwrap_or("".to_string())
        });
        let _ = request.respond(response);
    }
}
