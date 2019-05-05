#![feature(proc_macro_hygiene, decl_macro)]

extern crate fakedata_generator;
#[macro_use]
extern crate rocket;

use fakedata_generator::*;
use fakedata_generator::corpora::gen_corpora_switch;

#[get("/")]
fn index() -> &'static str {
    return r#"
Welcome to the fakedata_generator example implementation as a web server

Available routes:

    /gen/email
    /gen/username
    /gen/domain
    /gen/http_method
    /gen/ipv4
    /gen/enum_r/<input>
    /gen/corpora/<input>


Inputs:

/gen/enum_r/<input>
Specify input as comma-seperated strings, e.g.
/gen/enum_r/hello,world,this,is,a,tes
/gen/enum/horse,cat,dog
/gen/enum/active,inactive,unknown


/gen/corpora/<input>
Specify a corpora dataset, e.g.
/gen/corpora/horse
/gen/corpora/cat
/gen/corpora/fabric



"#;
}

#[get("/gen/email")]
fn email() -> String {
return gen_email();
}

#[get("/gen/username")]
fn username() -> String {
    return gen_username();
}

#[get("/gen/domain")]
fn domain() -> String {
    return gen_domain();
}

#[get("/gen/http_method")]
fn http_method() -> String {
    return gen_http_method();
}

#[get("/gen/ipv4")]
fn ipv4() -> String {
    return gen_ipv4();
}

#[get("/gen/enum/<input>")]
fn enum_r(input: String) -> String {
    return gen_enum(input.to_owned());
}

#[get("/gen/enum")]
fn enum_index() -> &'static str {
    return r#"
/gen/enum/<input>

Example Routes:
/gen/enum/available,unavailable
/gen/enum/horse,cat,dog
/gen/enum/active,inactive,unknown
"#;
}

#[get("/gen/corpora/<input>")]
fn corpora(input: String) -> String {
    return gen_corpora_switch(input.to_owned());
}

#[get("/gen/corpora")]
fn corpora_index() -> &'static str {
    return r#"
/gen/corpora/<input>

Example Routes:
/gen/corpora/horse
/gen/corpora/cat
/gen/corpora/fabric
"#;
}


/*
#[get("/gen/")]
fn () -> String {
    return gen_();
}
*/

fn main() {
    rocket::ignite()
        .mount("/", routes![index,email,username,domain,http_method,ipv4,enum_r,corpora_index,corpora])
        .launch();
}