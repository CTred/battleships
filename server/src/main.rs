use log::{info, trace, warn, LevelFilter};
use renet::{
    RenetConnectionConfig, RenetServer, ServerAuthentication, ServerConfig, ServerEvent,
    NETCODE_USER_DATA_BYTES,
};
use std::net::{SocketAddr, UdpSocket};
use std::time::{Duration, Instant, SystemTime};

use store::{EndGameReason, Player};

// Only clients that can provide the same PROTOCOL_ID that the server is using will be able to connect.
// This can be used to make sure players use the most recent version of the client for instance.
pub const PROTOCOL_ID: u64 = 1208;

fn main() {
    let target = env_logger::Target::Stdout;
    let mut builder = env_logger::Builder::from_default_env();
    builder
        .target(target)
        .filter(None, LevelFilter::Trace)
        .init();

    let server_addr: SocketAddr = "127.0.0.1:5000".parse().unwrap();
    let mut server: RenetServer = RenetServer::new(
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap(),
        ServerConfig {
            max_clients: 10,
            protocol_id: PROTOCOL_ID,
            public_addr: server_addr,
            authentication: ServerAuthentication::Unsecure,
        },
        RenetConnectionConfig::default(),
        UdpSocket::bind(server_addr).unwrap(),
    )
    .unwrap();

    trace!("GW server listening on {}", server_addr);

    let mut last_updated = Instant::now();
    let mut game_state = store::GameState::default();

    loop {
        // Update server time
        let now = Instant::now();
        server.update(now - last_updated).unwrap();
        last_updated = now;

        // Receive connection events from clients
        while let Some(event) = server.get_event() {
            match event {
                ServerEvent::ClientConnected(id, user_data) => {
                    // Tell the recently joined player about the other player
                    for (player_id, player) in game_state.players.iter() {
                        let event = store::GameEvent::PlayerJoined {
                            player_id: *player_id,
                            player_details: player.clone(),
                        };
                        server.send_message(id, 0, bincode::serialize(&event).unwrap());
                    }

                    // Add the new player to the game
                    let event = store::GameEvent::PlayerJoined {
                        player_id: id,
                        player_details: name_from_user_data(&user_data),
                    };
                    game_state.consume(&event);

                    // Tell all players that a new player has joined
                    server.broadcast_message(0, bincode::serialize(&event).unwrap());

                    info!("Client {} connected.", id);

                    // Begin game with two players
                    // TODO: implement "start button in lobby"
                    // TODO: players may join in the middle of a game
                    if game_state.players.len() == 2 {
                        let event = store::GameEvent::SetupBoard;
                        game_state.consume(&event);
                        server.broadcast_message(0, bincode::serialize(&event).unwrap());
                        trace!("Player setup ship positions");
                    }
                }
                ServerEvent::ClientDisconnected(id) => {
                    // First consume a disconnect event
                    let event = store::GameEvent::PlayerDisconnected { player_id: id };
                    game_state.consume(&event);
                    server.broadcast_message(0, bincode::serialize(&event).unwrap());
                    info!("Client {} disconnected", id);

                    // Then end the game
                    let event = store::GameEvent::EndGame {
                        reason: EndGameReason::PlayerLeft { player_id: id },
                    };
                    game_state.consume(&event);
                    server.broadcast_message(0, bincode::serialize(&event).unwrap());

                    // NOTE: Since we don't authenticate users we can't do any reconnection attempts.
                    // We simply have no way to know if the next user is the same as the one that disconnected.
                }
            }
        }

        // Receive GameEvents from clients. Broadcast valid events.
        for client_id in server.clients_id().into_iter() {
            while let Some(message) = server.receive_message(client_id, 0) {
                if let Ok(event) = bincode::deserialize::<store::GameEvent>(&message) {
                    if game_state.validade(&event) {
                        game_state.consume(&event);
                        trace!("Player {} sent: \n\t{:#?}", client_id, event);
                        server.broadcast_message(0, bincode::serialize(&event).unwrap());

                        // Determine if a player has won the game
                        // if let Some(winner) = game_state.determine_winner() {
                        //     let event = store::GameEvent::EndGame {
                        //         reason: store::EndGameReason::PlayerWon { winner },
                        //     };
                        //     server.broadcast_message(0, bincode::serialize(&event).unwrap());
                        // }
                    } else {
                        warn!("Player {} sent invalid event:\n\t{:#?}", client_id, event);
                    }
                }
            }
        }

        server.send_packets().unwrap();
        std::thread::sleep(Duration::from_millis(50));
    }
}

fn name_from_user_data(user_data: &[u8; NETCODE_USER_DATA_BYTES]) -> Player {
    let mut buffer = [0u8; 8];
    buffer.copy_from_slice(&user_data[0..8]);
    let mut len = u64::from_le_bytes(buffer) as usize;
    len = len.min(NETCODE_USER_DATA_BYTES - 8);
    let data = user_data[8..len + 8].to_vec();
    Player {
        name: String::from_utf8(data).unwrap(),
    }
}
