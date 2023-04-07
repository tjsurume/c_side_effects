use crate::prelude::*;

pub fn movement(
    mut commands: Commands,
    mut mb: ResMut<MapBuilder>,
    move_messages: Query<(Entity, &WantsToMove)>,
    mut movers: Query<(Entity, &mut Position)>
) {
    // for every message to move
    for (message_ent, move_signal) in move_messages.iter() {
        if mb.map.can_enter_tile(move_signal.destination) {
            if let Ok((mov_ent, mut position)) = movers.get_mut(move_signal.entity) {
            
                position.x = move_signal.destination.x;
                position.y = move_signal.destination.y;
            }
        }
        // delete the message
        commands.entity(message_ent).despawn();
    }
}