use std::ops::{Deref, DerefMut};

use bevy::prelude::*;

use crate::components::Name;

pub struct LogPlugin;

impl Plugin for LogPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LogMessage>()
            .insert_resource(LogBuffer::default())
            .add_system(log_messages);
    }
}

#[derive(Debug, Clone, Copy)]
pub enum GameEvent {
    WelcomePlayer,
    PushEnemies { num_pushed: u32 },
    FoundTreasure,
}

#[derive(Debug, Clone, Copy)]
pub struct LogMessage {
    pub actor: Entity,
    pub event: GameEvent,
}

impl ToString for GameEvent {
    fn to_string(&self) -> String {
        match self {
            Self::WelcomePlayer => "started to explore a new dungeon...".to_string(),
            Self::PushEnemies { num_pushed } => {
                if *num_pushed > 1 {
                    format!("pushed {num_pushed} creatures out of the way")
                } else {
                    format!("pushed {num_pushed} creature out of the way")
                }
            }
            Self::FoundTreasure => "found the treasure in this level!".to_string(),
        }
    }
}

#[derive(Debug, Default)]
pub struct LogBuffer(Vec<String>);

impl Deref for LogBuffer {
    type Target = Vec<String>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LogBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn log_messages(
    mut events: EventReader<LogMessage>,
    names: Query<&Name>,
    mut buffer: ResMut<LogBuffer>,
) {
    for e in events.iter() {
        if let Ok(name) = names.get(e.actor) {
            // Turning into a string here to get the fixed name in case the entity might be despawned
            buffer.push(format!("{} {}", name, e.event.to_string()));
        } else {
            warn!("Could not find name for event {:?}", e);
        }
    }
}
