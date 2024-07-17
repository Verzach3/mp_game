use bevy::prelude::*;
use bevy_tokio_tasks::TokioTasksRuntime;
use tokio::time::sleep;

#[derive(Component)]
pub struct Person;

#[derive(Component)]
pub struct Name(String);

#[derive(Resource)]
pub struct GreetingTimer(pub Timer);

pub fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Alice".to_string())));
    commands.spawn((Person, Name("Bob".to_string())));
    commands.spawn((Person, Name("Carol".to_string())));
}

pub fn greet_people_timer(
    time: Res<Time>,
    mut timer: ResMut<GreetingTimer>,
    runtime: ResMut<TokioTasksRuntime>,
    query: Query<&Name, With<Person>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in query.iter() {
            println!("Hello {}!", name.0);
        }
        runtime.spawn_background_task(|mut _ctx| async move {
           println!("Iniciando tarea");
           sleep(std::time::Duration::from_secs(2)).await;
           println!("Tarea finalizada");
        });
    }
}
