use super::{data::UserInput, input::input};
use crate::{
    components::acceleration::Acceleration,
    entity::player::movement,
    settings::network::{FPS, NUMBER_PLAYERS, ROLLBACK_DEFAULT, START_PORT},
};
use bevy::prelude::*;
use bevy_ggrs::{GGRSPlugin, Session};
use bevy_rapier2d::prelude::*;
use ggrs::{Config, SessionBuilder, UdpNonBlockingSocket};
use std::net::SocketAddr;

pub struct GGRSConfig;
impl Config for GGRSConfig {
    type Input = UserInput;
    type State = u8;
    type Address = SocketAddr;
}

pub fn init_network(app: &mut App) {
    // ask for client number
    println!("Enter client number (1-{}): ", NUMBER_PLAYERS);
    let mut client_num = String::new();
    std::io::stdin().read_line(&mut client_num).unwrap();
    let mut client_num: usize = client_num.trim().parse().unwrap();
    client_num -= 1;

    // Add players
    let mut sess_build = SessionBuilder::<GGRSConfig>::new().with_num_players(NUMBER_PLAYERS);
    for i in 0..NUMBER_PLAYERS {
        if i == client_num {
            sess_build = sess_build.add_player(ggrs::PlayerType::Local, i).unwrap();
        } else {
            sess_build = sess_build
                .add_player(
                    ggrs::PlayerType::Remote(
                        format!("127.0.0.1:{}", START_PORT + i).parse().unwrap(),
                    ),
                    i,
                )
                .unwrap();
        }
    }

    // Start the GGRS session
    let socket = UdpNonBlockingSocket::bind_to_port((START_PORT + client_num) as u16).unwrap();
    let sess = sess_build.start_p2p_session(socket).unwrap();

    // Create the GGRS plugin
    GGRSPlugin::<GGRSConfig>::new()
        .with_update_frequency(FPS)
        .register_rollback_component::<Acceleration>()
        .register_rollback_component::<Transform>()
        .register_rollback_component::<Velocity>()
        .with_input_system(input)
        .with_rollback_schedule(Schedule::default().with_stage(
            ROLLBACK_DEFAULT,
            SystemStage::parallel().with_system(movement),
        ))
        .build(app);
    app.insert_resource(Session::P2PSession(sess));
}
