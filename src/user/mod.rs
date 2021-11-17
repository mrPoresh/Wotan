mod handler;

pub mod model;
pub mod util;

pub(crate) mod service;
pub use util::has_role;

use crate::user::handler::register;
use actix_web::web;


// Create a user scope for requests                                                                                     //
//                                                                                                                      //
// * Example                                                                                                          * //
//                                                                                                                      //
// * curl --request POST \ --url http://.../user/register \ --header... :                                             * //
// *    ---> fn route : registrate routes with handler (fn register)                                                  * //
// *        ---> fn register (src/user/handler) : get data from request, pool from appData and call in fn create_user * //
// *                                                                                                                  * //
pub fn route(cfg: &mut web::ServiceConfig) {

    cfg.service(
        web::scope("/user")
            .service(web::resource("/register").route(web::post().to(register)))
            //.service -> login
            //.servise -> logout
            //.service -> get me
    );

}