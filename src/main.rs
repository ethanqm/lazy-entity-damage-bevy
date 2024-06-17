use bevy::prelude::*;

#[derive(Event)]
struct Damage(Entity);

#[derive(Event)]
struct Death(Entity);

fn main() {
    println!("Hello, world!");

    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<Death>()
        .add_event::<Damage>()
        .add_systems(PreStartup, setup)
        .add_systems(Update, (health_regen_update, remove_healed).chain())
        .add_systems(Update, emit_random_damage.before(health_regen_update))
        .add_systems(Update, emit_deaths.after(health_regen_update))
        .add_systems(Update, remove_dead.after(emit_deaths))
        .run();
}

fn setup(mut commands: Commands) {
    // spawn "player"
    commands.spawn(EntityTag {});
    // spawn 99 "enemies"
    for _i in 1..98 {
        commands.spawn(EntityTag {});
    }
}

#[derive(Component, Debug)]
struct EntityTag();

#[derive(Component)]
struct Health {
    value: i32,
    max: i32,
}

fn health_regen_update(
    mut hs: Query<(Entity, Option<&mut Health>)>,
    mut er: EventReader<Damage>,
    mut commands: Commands,
) {
    // all old values are incremented for health regen
    // events either update a value or append to the list.
    let es: Vec<Entity> = er.read().into_iter().map(|d| d.0).collect();

    for q in &mut hs {
        match q {
            (e, Some(mut h)) => {
                h.value += 1;
                // damage for this entity reported
                if es.contains(&e) {
                    h.value -= 10;
                    println!("{:?} was hurt again!\t{} HP", e, h.value);
                }
            }
            (e, None) => {
                if es.contains(&e) {
                    commands.entity(e).insert(Health {
                        value: 90,
                        max: 100,
                    });
                    println!("{:?} was hurt!\t{} HP", e, 90);
                }
            }
        }
    }
}

fn emit_random_damage(mut ew: EventWriter<Damage>, entities: Query<Entity>) {
    for e in entities.iter() {
        if rand::random::<u8>() < 20 {
            ew.send(Damage(e));
            // println!("Entity #{:?} was hurt!",e);
        }
    }
}

fn remove_healed(hs: Query<(Entity, &Health)>, mut commands: Commands) {
    for (e, h) in &hs {
        if h.value >= h.max {
            commands.entity(e).remove::<Health>();
        }
    }
}

fn emit_deaths(hs: Query<(Entity, &Health)>, mut ew: EventWriter<Death>) {
    hs.iter().for_each(|(e, h)| {
        if h.value <= 0 {
            ew.send(Death(e));
            println!("{:?} died!", e);
        }
    });
}

fn remove_dead(mut er: EventReader<Death>, mut commands: Commands) {
    for ev in er.read() {
        commands.entity(ev.0).despawn();
    }
}
