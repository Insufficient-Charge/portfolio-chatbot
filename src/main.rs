use rocket::{
    form::{Contextual, Form},
    response::Redirect,
    tokio::spawn,
};
use rocket_dyn_templates::{context, Template};

use routes::date::{date_plus_month, get_current_date};
mod routes;
mod services;

#[macro_use]
extern crate rocket;

#[get("/")]
async fn index() -> Template {

    // We must wait on the request handles to finish
    let mut request_handles = services::gemini::REQUEST_HANDLE_MUTEX.lock().await;
    for handle in request_handles.iter() {
        println!("Waiting on handle to finish...");
        while !handle.is_finished() {}
    }
    request_handles.clear();

    println!("Index rendered");

    // I wonder if we could get away with not cloning the entire conversation
    let conversation = services::gemini::CONVERSATION_MUTEX.lock().await.clone();
    Template::render(
        "index",
        context! {
            conversation: conversation,
        },
    )
}

#[post("/submit", data = "<form>")]
#[allow(dead_code)]
fn submit(form: Form<Contextual<'_, String>>) -> Redirect {
    let gemini_task = spawn(services::gemini::generate_response("Give me a summary about JP"));

    spawn(async {
        services::gemini::REQUEST_HANDLE_MUTEX.lock().await.push(
            gemini_task
        );
    });

    return Redirect::to(uri!(index));
}

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount(
            "/",
            routes![index, get_current_date, date_plus_month, submit,],
        )
}
