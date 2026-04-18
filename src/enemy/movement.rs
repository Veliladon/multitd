use crate::prelude::*;

pub fn move_enemy(
    mut commands: Commands,
    mut move_query: Query<(&mut Mobile, &mut Transform, Entity)>,
    // lane_query: Query<&FollowsLane>,
    time: Res<Time>,
    entity_map: Res<MazeEntityMap>,
    route: Res<MazeRoute>,
) {
    for (mut mobile, mut transform, entity) in move_query.iter_mut() {
        let mut new_translation = transform.translation.xz()
            + (mobile.direction.as_vec2() * (mobile.speed * time.delta_secs()));
        /* info!(
            "{:?} {:?}",
            new_translation.y.signum() != transform.translation.z.signum(),
            (new_translation.x == 0.0 && new_translation.y == 0.0)
        );*/

        if (new_translation.x.signum() != transform.translation.x.signum())
            || (new_translation.y.signum() != transform.translation.z.signum())
        {
            //info!("{:?}", route);
            info!("Mob crossed a middle - {:?}", new_translation);

            debug!("Old Destination {}", route.nodes[mobile.index]);
            mobile.index += 1;

            info!("Mobile Index: {}", mobile.index);
            let new_destination = route.nodes[mobile.index];
            info!("New Destination {}", route.nodes[mobile.index]);
            debug!(
                "new desgination: {}, old destination: {}",
                new_destination, mobile.destination
            );

            let difference = new_destination as i32 - mobile.destination as i32;
            match difference {
                6 => {
                    // north
                    debug!("Moving North");
                    mobile.direction = DIRECTIONS[0];
                    new_translation.y = 0. // need to clamp it to correct sign
                }

                1 => {
                    // east
                    info!("Moving East");
                    mobile.direction = DIRECTIONS[1];
                    new_translation.x = 0. // need to clamp it to correct sign
                }
                -6 => {
                    // south
                    debug!("Moving South");
                    mobile.direction = DIRECTIONS[2];
                    new_translation.y = -0. // need to clamp it to correct sign
                }
                -1 => {
                    // west
                    debug!("Moving West");
                    mobile.direction = DIRECTIONS[3];
                    new_translation.x = -0. // need to clamp it to correct sign
                }
                _ => panic!(),
            }
            info!("{:?}", mobile.direction);
            mobile.destination = new_destination;
        }

        if new_translation.x > 3.0
            || new_translation.y > 3.0
            || new_translation.x < -3.0
            || new_translation.y < -3.0
        {
            // info!("hit the tile edge");
            commands
                .entity(entity)
                .insert(ChildOf(entity_map[mobile.destination]));
            info!("{:?}", entity_map[mobile.destination]);
            match mobile.direction {
                IVec2 { x: 0, y: 1 } => {
                    new_translation.y -= 6.0;
                    info!("{:?}", new_translation);
                }
                IVec2 { x: 1, y: 0 } => {
                    new_translation.x -= 6.0;
                    info!("{:?}", new_translation);
                }
                IVec2 { x: 0, y: -1 } => {
                    new_translation.y += 6.0;
                    info!("{:?}", new_translation);
                }
                IVec2 { x: -1, y: 0 } => {
                    new_translation.x += 6.0;
                    info!("{:?}", new_translation);
                }

                _ => panic!(),
            }
        }

        transform.translation = Vec3::new(
            new_translation.x,
            transform.translation.y,
            new_translation.y,
        );

        info!("{:?}", transform.translation.xz());
    }
}
