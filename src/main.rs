use bevy::prelude::*;

fn hello_world() {
    println!("hello world")
}

struct Person;
struct Name(String);

fn main() {
    App::build()
        .add_startup_system(add_people.system())
        .add_system(hello_world.system())
        .add_system(greet_people.system())
        .run();
    println!("Something happened!...");
}

// a system
fn add_people(mut commands: Commands) {
    commands.spawn().insert(Person).insert(Name("Camilo Hughes".to_string()));
    commands.spawn().insert(Person).insert(Name("Helena Bobena".to_string()));
    commands.spawn().insert(Person).insert(Name("Jon Stevens".to_string()));
}

// another system
fn greet_people(query: Query<&Name, With<Person>>) {
    for name in query.iter() {
        println!("hello {}!", name.0);
    }
}
