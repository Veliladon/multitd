use crate::prelude::*;

pub fn move_enemy(mut move_query: Query<(&mut Mobile, &mut Transform)>, time: Res<Time>) {
    for (mobile, mut transform) in move_query.iter_mut() {
        let new_translation = transform.translation.xz()
            + (mobile.direction.as_vec2() * (mobile.speed * time.delta_secs()));
        transform.translation = Vec3::new(
            new_translation.x,
            transform.translation.y,
            new_translation.y,
        );
    }
}
