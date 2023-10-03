use actix_web::{ web, App, HttpResponse, HttpServer, Responder };
use serde::Deserialize;

#[derive(Deserialize)]
struct GcdParameters {
    m: u64,
    n: u64
}

#[actix_web::main]
async fn main() {
    let server = HttpServer::new( || {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/gcd", web::post().to(post_gcd))
    });

    println!("Servering on http://0.0.0.0:30005...");
    server
        .bind("0.0.0.0:30005").expect("error binding server to address")
        .run().await.expect("error running server");
}

async fn get_index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r#"
                <title>GCD Calculator</title>
                <form action="/gcd" method="post">
                <input type="text" name="m"/>
                <input type="text" name="n"/>
                <button type="submit">Compute GCD</button>
                </form>
            "#
        )
}

async fn post_gcd(form: web::Form<GcdParameters>) -> impl Responder {
    if form.m == 0 || form.n == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Computing the GCD with zero is boring");
    }

    let response = 
        format!("The GCD of the numbers {} and {} \
                is <b>{}</b>\n",
            form.m, form.n, gcd(form.m, form.n));

    HttpResponse::Ok()
        .content_type("text/html")
        .body(response)
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    let t = a % b;
    return gcd(b, t);
}