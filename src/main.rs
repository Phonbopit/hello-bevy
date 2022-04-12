use bevy::prelude::*;

#[derive(Component)]
struct Position {
    x: f32,
    y: f32
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands.spawn().insert(Person).insert(Name("Chuck Norris".to_string()));
    commands.spawn().insert(Person).insert(Name("John Doe".to_string()));
    commands.spawn().insert(Person).insert(Name("Prayuzz".to_string()));
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in query.iter() {
        println!("hello {}", name.0);
    }
}

fn hello_world() {
    println!("hello world");
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        println!("HelloPlugin!");

        app.add_startup_system(add_people)
            .add_system(hello_world)
            .add_system(greet_people);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .run();
}
