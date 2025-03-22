use anyhow::{Result, anyhow};
use bytes::{Buf, BufMut, Bytes, BytesMut};
use rand::random;
use std::collections::HashMap;
use std::time::Duration;
use tokio::io;
use tokio::net::UdpSocket;
use tokio::time::timeout;

const QUERY_MAGIC: u16 = 0xfe_fd;
const SESSION_ID_MASK: u32 = 0x0f_0f_0f_0f;

/// The code is taken from https://github.com/ariscript/mc-query and was modified to better suit the proejct.

#[derive(Debug)]
pub struct FullStatResponse {
    /// The "motd" - message shown in the server list by the client.
    pub motd: String,

    /// The server's game type.
    /// Vanilla servers hardcode this to "SMP".
    pub game_type: String,

    /// The server's game ID.
    /// Vanilla servers hardcode this to "MINECRAFT".
    pub game_id: String,

    /// The server's game version.
    pub version: String,

    /// The plugins the server has installed.
    /// Vanilla servers return an empty string.
    /// Other server platforms may have their own format for this field.
    pub plugins: String,

    /// The server's world/map name.
    pub map: String,

    /// The current number of online players.
    pub num_players: usize,

    /// Maximum players online this server allows.
    pub max_players: usize,

    /// The port the server is running on.
    pub host_port: u16,

    /// The server's IP address.
    pub host_ip: String,

    /// The current list of online players.
    pub players: Vec<String>,
}

pub async fn stat_send(sock: &UdpSocket, bytes: &[u8]) -> io::Result<Bytes> {
    sock.send(bytes).await?;
    Box::pin(timeout(Duration::from_millis(250), recv_packet(sock))).await?
}
pub async fn handshake(socket: &UdpSocket) -> Result<(i32, i32), anyhow::Error> {
    // generate new token per interaction to avoid reset problems
    #[allow(clippy::cast_possible_wrap)] // this is fine, we don't care about the value
    let session_id = (random::<u32>() & SESSION_ID_MASK) as i32;

    let mut req = BytesMut::with_capacity(7);
    req.put_u16(QUERY_MAGIC);
    req.put_u8(9); // packet type 9 - handshake
    req.put_i32(session_id);
    // no payload for handshake requests

    socket.send(&req).await?;

    let mut response = Box::pin(recv_packet(socket)).await?;
    validate_packet(&mut response, 9, session_id)?;

    let token_str = get_string(&mut response)?;

    token_str
        .parse()
        .map(|t| (t, session_id))
        .map_err(|_| anyhow::anyhow!("Cannot parse int"))
}

async fn recv_packet(socket: &UdpSocket) -> io::Result<Bytes> {
    let mut buf = [0u8; 65536];
    socket.recv(&mut buf).await?;

    Ok(Bytes::copy_from_slice(&buf))
}

fn validate_packet(
    packet: &mut Bytes,
    expected_type: u8,
    expected_session: i32,
) -> Result<(), anyhow::Error> {
    let recv_type = packet.get_u8();
    if recv_type != expected_type {
        return Err(anyhow::anyhow!("Invalid packet type"));
    }

    let recv_session = packet.get_i32();
    if recv_session != expected_session {
        return Err(anyhow::anyhow!("Session ID mismatch"));
    }

    Ok(())
}

fn get_string(bytes: &mut Bytes) -> Result<String, anyhow::Error> {
    let mut buf = vec![];
    loop {
        let byte = bytes.get_u8();
        if byte == 0 {
            break;
        }
        buf.push(byte);
    }

    Ok(String::from_utf8(buf)?)
}

pub async fn stat_full(host: &str, port: u16) -> Result<FullStatResponse, anyhow::Error> {
    let socket = UdpSocket::bind("0.0.0.0:0").await?;
    socket.connect(format!("{host}:{port}")).await?;

    let (token, session) = Box::pin(handshake(&socket)).await?;

    let mut bytes = BytesMut::new();
    bytes.put_u16(QUERY_MAGIC);
    bytes.put_u8(0); // packet type 0 - stat
    bytes.put_i32(session);
    bytes.put_i32(token);
    bytes.put_u32(0); // 4 extra bytes required for full stat vs. basic

    let mut res = match stat_send(&socket, &bytes).await {
        Ok(v) => v,
        Err(_) => stat_send(&socket, &bytes).await?,
    };

    validate_packet(&mut res, 0, session)?;

    // skip 11 meaningless padding bytes
    res.advance(11);

    // K,V section
    let mut kv = HashMap::new();
    loop {
        let key = get_string(&mut res)?;
        if key.is_empty() {
            break;
        }
        let value = get_string(&mut res)?;
        kv.insert(key, value);
    }

    // excuse this horrendous code, I don't know of a better way
    let motd = kv
        .remove("hostname")
        .ok_or_else(|| anyhow!("Missing 'hostname' field"))?;
    let game_type = kv
        .remove("gametype")
        .ok_or_else(|| anyhow!("Missing 'gametype' field"))?;
    let game_id = kv
        .remove("game_id")
        .ok_or_else(|| anyhow!("Missing 'game_id' field"))?;
    let version = kv
        .remove("version")
        .ok_or_else(|| anyhow!("Missing 'version' field"))?;
    let plugins = kv
        .remove("plugins")
        .ok_or_else(|| anyhow!("Missing 'plugins' field"))?;
    let map = kv
        .remove("map")
        .ok_or_else(|| anyhow!("Missing 'map' field"))?;
    let num_players = kv
        .remove("numplayers")
        .ok_or_else(|| anyhow!("Missing 'numplayers' field"))?
        .parse()
        .map_err(|_| anyhow!("Failed to parse 'numplayers' as integer"))?;
    let max_players = kv
        .remove("maxplayers")
        .ok_or_else(|| anyhow!("Missing 'maxplayers' field"))?
        .parse()
        .map_err(|_| anyhow!("Failed to parse 'maxplayers' as integer"))?;
    let host_port = kv
        .remove("hostport")
        .ok_or_else(|| anyhow!("Missing 'hostport' field"))?
        .parse()
        .map_err(|_| anyhow!("Failed to parse 'hostport' as integer"))?;
    let host_ip = kv
        .remove("hostip")
        .ok_or_else(|| anyhow!("Missing 'hostip' field"))?;

    // skip 10 meaningless padding bytes
    for _ in 0..10 {
        res.get_u8();
    }

    // players section
    let mut players = vec![];
    loop {
        let username = get_string(&mut res)?;
        if username.is_empty() {
            break;
        }
        players.push(username);
    }

    Ok(FullStatResponse {
        motd,
        game_type,
        game_id,
        version,
        plugins,
        map,
        num_players,
        max_players,
        host_port,
        host_ip,
        players,
    })
}
