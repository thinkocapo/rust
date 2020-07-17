extern crate actix_web;
extern crate sentry;
extern crate sentry_actix;
#[macro_use]
extern crate failure;

#[macro_use]
extern crate lazy_static;


use std::num::ParseIntError;


use std::env;
use std::collections::HashMap;
use sentry::integrations::failure::capture_error;
use sentry::{configure_scope, User};
use actix_web::{http};
use actix_web::Json;
use actix_web::Result;
use serde::Deserialize;
use serde::Serialize;
use sentry::protocol::value::to_value;






use std::sync::Mutex;

use sentry::integrations::panic::register_panic_handler;



lazy_static! {

    #[derive(Copy, Deserialize, Clone, Debug)]
    static ref HASHMAP: Mutex<HashMap<&'static str, u32>> = {
        let mut Inventory = HashMap::new();
        Inventory.insert("wrench", 1);
        Inventory.insert("nails", 1);
        Inventory.insert("hammer", 1);
        Mutex::new(Inventory)
    };
}


use actix_web::{server, App, HttpRequest, HttpResponse};
use sentry_actix::SentryMiddleware;


fn multiply_new(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    let first_number: i32 = first_number_str.parse()?;
    let second_number: i32 = second_number_str.parse()?;
    Ok(first_number * second_number)
}

fn handled_new(_req: &HttpRequest) -> HttpResponse {
    let first = "t";
    let second = "2";
    let result = match multiply_new(first, second) {
        Ok(result) => result,
        Err(err) => {
            let foo = err.into();
            capture_error(&foo);
            let result: HttpResponse = "try again".to_string().into();

            

            return result;
        }
    };
    
    let result: HttpResponse = (format!("{} * {} => {}", first, second, result)).into();

    return result;
  


}

fn fakedatabseapp(_req: &HttpRequest) -> HttpResponse{

    panic!("Unhandled request!");
 
}


#[derive(Deserialize, Clone, Debug)]
struct CardSubmittedPayload {
    card_id: i64,
}


#[derive(Serialize, Deserialize, Clone, Debug)]
struct Item {
    id: String,
    name: String,
    price: f64,
    img: String,
}




#[derive(Serialize, Clone, Debug, Deserialize)]
struct CheckoutPayload {

    email: String,
    cart: Vec<Item>,

}


fn process_order(cart: &Vec<Item>) -> HttpResponse {


    let mut map = HASHMAP.lock().unwrap();
    println!("The entry for `0` is \"{:?}\".", map.get("foo"));

  

    for cartitem in cart.iter() {

        
        if map.get(cartitem.id.as_str()).map(|id| id <= &0).unwrap_or(false) {


            let mut string = String::new();
            string.push_str("Not enough inventory for ");
            string.push_str(&cartitem.id);

            configure_scope(|scope| {
        
        
                scope.set_extra("inventory", to_value(map.clone()).unwrap());
        
            });

            capture_error(&format_err!("Error: {}", string));
            
            let result: HttpResponse = string.to_string().into();

            
            
            return result;
            

        } else if map.get(cartitem.id.as_str()).map(|id| id > &0).unwrap_or(false) {
        
            
                if let Some (id) = map.get_mut(cartitem.id.as_str()) {
                    *id -= 1;
                    println!("Success: {:?} was purchased, remaining stock is {:?}", cartitem.id, cartitem.id.as_str());
                } else {
                    false;
                }
            
            

        }
        
    }

    let result: HttpResponse = (format!("Everything ok")).into();

    return result;
      

}



fn checkout(req: HttpRequest, body: Json<CheckoutPayload>) -> HttpResponse { 

   
 
    configure_scope(|scope| {
        scope.set_user(Some(User {
            email: Some((*body.email).to_string()),
            ..Default::default()
        }));

        let mut string = String::new();

   
        string.push_str(req.headers().get("X-Transaction-ID").unwrap().to_str().unwrap());

        scope.set_tag("transaction_id", string);

        string = String::new();
        string.push_str(req.headers().get("X-Session-ID").unwrap().to_str().unwrap());

        scope.set_tag("session_id", string);

        string = String::new();

        

    });
    
    return process_order(&body.cart);

}



fn main() {

    register_panic_handler();


    let _guard = sentry::init("https://ef73d8aa7ac643d2b6f1d1e604d607eb@o87286.ingest.sentry.io/5250920");
    env::set_var("RUST_BACKTRACE", "1");
    
    server::new(|| {
        App::new().middleware(SentryMiddleware::new())
        .resource("/handled_new",|r| r.method(http::Method::GET).f(handled_new))
        .resource("/unhandled",|r| r.method(http::Method::GET).f(fakedatabseapp))
        .resource("/checkout", |r| r.method(http::Method::POST).with(checkout))}).bind("127.0.0.1:3001")
        .unwrap()
        .run();

  
        sentry::integrations::panic::register_panic_handler();

        
}