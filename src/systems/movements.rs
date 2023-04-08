use crate::prelude::*;

pub fn movement(
    mut commands: Commands,
    mb: ResMut<MapBuilder>,
    move_messages: Query<(Entity, &WantsToMove)>,
    mut movers: Query<(Entity, &mut Position, With<Player>)>,
    our_clock : ResMut<OurClock>,
) {
    // for every message to move
    for (message_ent, move_signal) in move_messages.iter() {
        if our_clock.state == MyTimeState::Playing {
            if mb.map.can_enter_tile(move_signal.destination) {                
                if let Ok((_, mut position, _)) = movers.get_mut(move_signal.entity) {
                    position.x = move_signal.destination.x;
                    position.y = move_signal.destination.y;
                }
            }
        }
        // delete the message
        commands.entity(message_ent).despawn();
    }
}
 pub fn camera_movement(
    player_query: Query<&Position, (Changed<Position>, With<Player>)>,
    mut camera_query: Query<&mut Transform, (With<MyGameCamera>, Without<Player>)>,
) {

    for player_position in player_query.iter() {
        let mut camera_transform = camera_query.single_mut();
        let cam_x = convert_pos(player_position.x as f32,  SCREEN_WIDTH as f32);
        let cam_y = convert_pos(player_position.y as f32,  SCREEN_HEIGHT as f32);
        camera_transform.translation = Vec3::new(cam_x, cam_y, 999.0);
    }

}