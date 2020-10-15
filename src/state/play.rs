use crate::proto::play::{Clientbound, Gamemode, Serverbound};
use crate::proto::types::Uuid;
use crate::proto::{Peekable, TransportSession};
use std::io;
use std::net::TcpStream;

pub struct Play<R = TcpStream, W = TcpStream> {
    session: TransportSession<R, W>,
    uuid: Uuid,
    username: String,

    entity_id: i32,
    gamemode: Gamemode,
    view_distance: i32,
    enable_respawn_screen: bool,
    held_item: i8,
    x: f64,
    y: f64,
    z: f64,
    yaw: f32,
    pitch: f32,
    on_ground: bool,
}

impl<R, W> Play<R, W>
where
    R: io::Read,
    W: io::Write,
{
    pub fn new(session: TransportSession<R, W>, uuid: Uuid, username: String) -> Self {
        Self {
            session,
            uuid,
            username,

            entity_id: -1,
            gamemode: Gamemode::Survival,
            view_distance: -1,
            enable_respawn_screen: true,
            held_item: 0,
            x: 0.0,
            y: 0.0,
            z: 0.0,
            yaw: 0.0,
            pitch: 0.0,
            on_ground: true,
        }
    }

    fn handle_packet(&mut self, packet: &Clientbound) -> anyhow::Result<Option<Event>> {
        match packet {
            Clientbound::KeepAlive { keepalive_id } => {
                self.session.write_packet(&Serverbound::KeepAlive {
                    keepalive_id: *keepalive_id,
                })?;
            }
            Clientbound::JoinGame {
                entity_id,
                gamemode,
                view_distance,
                enable_respawn_screen,
                ..
            } => {
                self.entity_id = *entity_id;
                self.gamemode = *gamemode;
                self.view_distance = view_distance.0;
                self.enable_respawn_screen = *enable_respawn_screen;

                //TODO make these actual settings
                self.session.write_packet(&Serverbound::ClientSettings {
                    locale: "en_US".to_string().into(),
                    view_distance: 16,
                    chat_mode: 0.into(),
                    chat_colors: true,
                    displayed_skin_parts: 0x7f,
                    main_hand: 0.into(),
                })?;
            }
            Clientbound::HeldItemChange { slot } => {
                self.held_item = *slot;
            }
            Clientbound::DeclareRecipes { .. } => {
                //TODO registries
            }
            Clientbound::Tags { .. } => {
                //TODO registries
            }
            Clientbound::EntityStatus { .. } => {
                //TODO
            }
            Clientbound::DeclareCommands { .. } => {
                //TODO
            }
            Clientbound::UnlockRecipes { .. } => {
                //TODO
            }
            &Clientbound::PlayerPositionAndLook {
                x,
                y,
                z,
                yaw,
                pitch,
                flags,
                teleport_id,
            } => {
                if flags & 0x01 == 0 {
                    self.x = x;
                } else {
                    self.x += x;
                }
                if flags & 0x02 == 0 {
                    self.y = y;
                } else {
                    self.y += y;
                }
                if flags & 0x04 == 0 {
                    self.z = z;
                } else {
                    self.z += z;
                }
                if flags & 0x08 == 0 {
                    self.yaw = yaw;
                } else {
                    self.yaw += yaw;
                }
                if flags & 0x10 == 0 {
                    self.pitch = pitch;
                } else {
                    self.pitch += pitch;
                }
                self.session
                    .write_packet(&Serverbound::TeleportConfirm { teleport_id })?;
                self.session
                    .write_packet(&Serverbound::PlayerPositionAndRotation {
                        x: self.x,
                        y: self.y,
                        z: self.z,
                        yaw: self.yaw,
                        pitch: self.pitch,
                        on_ground: self.on_ground,
                    })?;
            }
            Clientbound::PlayerInfo { .. } => {
                //TODO
            }
            Clientbound::UpdateLight { .. } => {
                //TODO
            }
            Clientbound::ChunkData { .. } => {
                //TODO
            }
            Clientbound::WorldBorder { .. } => {
                //TODO
            }
            Clientbound::SpawnPosition { .. } => {
                //TODO
            }
            _ => {}
        }
        Ok(None)
    }

    pub fn try_poll(&mut self) -> anyhow::Result<Option<Event>>
    where
        R: Peekable,
    {
        while let Some(packet) = self.session.try_read_packet()? {
            if let Some(event) = self.handle_packet(&packet)? {
                return Ok(Some(event));
            }
        }
        Ok(None)
    }

    pub fn poll(&mut self) -> anyhow::Result<Event> {
        loop {
            let packet = self.session.read_packet()?;
            if let Some(event) = self.handle_packet(&packet)? {
                return Ok(event);
            }
        }
    }
}

pub enum Event {}
