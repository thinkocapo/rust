extern crate actix_web;
extern crate sentry;
extern crate sentry_actix;



use std::num::ParseIntError;


use std::env;
use std::io;
use std::collections::HashMap;
use std::process;
use sentry::integrations::failure::capture_error;
use actix_web::{http};


use sentry::integrations::panic::register_panic_handler;





use actix_web::{server, App, Error, HttpRequest, http::StatusCode, HttpResponse};
use sentry_actix::SentryMiddleware;

use actix_web::Responder;




// My version
fn multiply_new(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    let first_number: i32 = first_number_str.parse()?;
    let second_number: i32 = second_number_str.parse()?;
    Ok(first_number * second_number)
}

fn handled_new(_req: &HttpRequest) -> Result<String, Error> {
    let first = "t";
    let second = "2";
    let result = match multiply_new(first, second) {
        Ok(result) => result,
        Err(err) => {
            // Foo is the ParseIntError turned into a failure::Error.
            let foo = err.into();
            capture_error(&foo);
            return Ok("try again".to_string());
        }
    };
    // Not sure what the plan is for the 'result' variable.
    return Ok(format!("{} * {} => {}", first, second, result))
    

}

/* fn process_order(inventory):
    global Inventory
    tempInventory = Inventory
    for item in cart:
        if Inventory[item['id']] <= 0:
            raise Exception("Not enough inventory for " + item['id'])
        else:
            tempInventory[item['id']] -= 1
            print 'Success: ' + item['id'] + ' was purchased, remaining stock is ' + str(tempInventory[item['id']])
    Inventory = tempInventory  */



/* fn checkout(_req: &HttpRequest) -> Result<String, Error> {
    
    let mut inventory = HashMap::new();

    inventory.insert("wrench", "1");
    inventory.insert("nails", "1");
    inventory.insert("hammer", "1");

    Err(io::Error::new(io::ErrorKind::Other, "An error happens here").into())
    order = json.loads(request.data)
    print "Processing order for: " + order["email"]
    cart = order["cart"]
    
    process_order(cart)

    return "Success"
} */

fn main() {

    register_panic_handler();


    let _guard = sentry::init("https://ef73d8aa7ac643d2b6f1d1e604d607eb@o87286.ingest.sentry.io/5250920");
    env::set_var("RUST_BACKTRACE", "1");
    
          //|r| r.f(handled)).resource("/unhandled", |r| r.f(unhandled))
        //.resource("/checkout", |r| r.f(checkout))
    server::new(|| {
        App::new().middleware(SentryMiddleware::new())
        .resource("/handled_new", |r| r.f(handled_new))}).bind("127.0.0.1:3001")
        .unwrap()
        .run();

        sentry::integrations::panic::register_panic_handler();

        
}