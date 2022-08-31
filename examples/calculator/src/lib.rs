/*
 * Copyright 2022 Comcast Cable Communications Management, LLC
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 * SPDX-License-Identifier: Apache-2.0
 */
struct Calculator { 
}

impl Calculator {
  fn dispatch_request(&mut self, req: json::JsonValue, ctx: thunder_rs::RequestContext) {
    if let Some(method) = req["method"].as_str() {
      match method {
        "add" => { self.add(req, ctx); }
        "mul" => { self.mul(req, ctx); }
        _ => {
          println!("method {} not handled here", method);
        }
      }
    }
  }

  fn add(&mut self, req: json::JsonValue, ctx: thunder_rs::RequestContext) {
    let mut sum = 0;
    for val in req["params"].members() {
      if let Some(n) = val.as_u32() {
        sum += n;
      }
    }

    let res = json::object!{
      "jsonrpc": "2.0",
      "id": req["id"].as_u32(),
      "result": sum
    };

    self.send_response(res, ctx);
  }

  fn mul(&mut self, req: json::JsonValue, ctx: thunder_rs::RequestContext) {
    let mut product = 0;
    for val in req["params"].members() {
      if let Some(n) = val.as_u32() {
        product *= n
      }
    }

    let res = json::object!{
      "jsonrpc": "2.0",
      "id": req["id"].as_u32(),
      "result": product
    };

    self.send_response(res, ctx);
  }

  fn send_response(&mut self, res: json::JsonValue, ctx: thunder_rs::RequestContext) {
    let s = json::stringify(res);
    ctx.send(s);
  }
}

impl thunder_rs::Plugin for Calculator {
  fn on_message(&mut self, json: String, ctx: thunder_rs::RequestContext) {
    let req: json::JsonValue = json::parse(json.as_str())
      .unwrap();
    self.dispatch_request(req, ctx);
  }
  fn on_client_connect(&mut self, _channel_id: u32) { }
  fn on_client_disconnect(&mut self, _channel_id: u32) { }
}

fn sample_plugin_init(_conf: thunder_rs::PluginConfig) -> Box<dyn thunder_rs::Plugin> {
  Box::new(Calculator{ })
}

thunder_rs::export_plugin!("Calculator", (1,0,0), sample_plugin_init);
