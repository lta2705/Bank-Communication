mod config;
mod api;
mod adapters;
// #[actix::main]
fn main(){
    let system = actix::System::new();

    system.run().expect("TODO: panic message");
}
