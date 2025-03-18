import discord
from discord import app_commands
import socket
import struct
import json
import traceback
from mcstatus import JavaServer


client = discord.Client(intents=discord.Intents.default())
tree = app_commands.CommandTree(client)


@client.event
async def on_ready():
    print("Logged in as {0.user}".format(client))
    await tree.sync()  # Sync the command tree


@client.event
async def on_message(message):
    if message.author == client.user:
        return
    if message.content.startswith("$server"):
        try:
            server_info = get_info("141.148.218.223", 25566)
            await message.channel.send(
                f"Server info: ```json\n{json.dumps(server_info, indent=2)}```"
            )
        except Exception as e:
            await message.channel.send(f"Error fetching server info: {str(e)}")


@tree.command(
    name="server",
    description="Get information about the Minecraft server",
)
async def server_command(interaction):
    try:
        await interaction.response.send_message("Fetching server info...")
        server_info = get_info("141.148.218.223", 25566)
        server = JavaServer.lookup("141.148.218.223:25566")

        # 'status' is supported by all Minecraft servers that are version 1.7 or higher.
        # Don't expect the player list to always be complete, because many servers run
        # plugins that hide this information or limit the number of players returned or even
        # alter this list to contain fake players for purposes of having a custom message here.
        # 'ping' is supported by all Minecraft servers that are version 1.7 or higher.
        # It is included in a 'status' call, but is also exposed separate if you do not require the additional info.

        # Format the server info in a nicer way
        formatted_info = "# Server Status\n"

        latency = server.ping()
        formatted_info += f"The server replied in {latency} ms\n"

        # 'query' has to be enabled in a server's server.properties file!
        # It may give more information than a ping, such as a full player list or mod information.
        query = server.query()
        player_usernames = query.players.names
        formatted_info += f"## Metadata\n"
        if "description" in server_info:
            if (
                isinstance(server_info["description"], dict)
                and "text" in server_info["description"]
            ):
                formatted_info += f"{server_info['description']['text']}\n"
            elif isinstance(server_info["description"], str):
                formatted_info += f"{server_info['description']}\n"
        if "version" in server_info:
            formatted_info += (
                f"Version: {server_info['version'].get('name', 'Unknown')}\n"
            )
        formatted_info += "## Player Info\n\n"

        if "players" in server_info:
            players = server_info["players"]
            formatted_info += (
                f"\n**Players**: {players.get('online', 0)}/{players.get('max', 0)}\n"
            )
            for player in player_usernames:
                formatted_info += f"- {player}\n"

        await interaction.followup.send(formatted_info)
        print(formatted_info)

        # Send the full JSON as well
        #  await interaction.followup.send(
        #      f"```json\n{json.dumps(server_info, indent=2)[:1900]}```"
        #  )
    except Exception as e:
        error_details = traceback.format_exc()
        print(f"Error fetching server info: {e}\n{error_details}")
        await interaction.followup.send(f"Error fetching server info: {str(e)}")


def unpack_varint(s):
    d = 0
    for i in range(5):
        b = ord(s.recv(1))
        d |= (b & 0x7F) << 7 * i
        if not b & 0x80:
            break
    return d


def pack_varint(d):
    o = bytearray()
    while True:
        b = d & 0x7F
        d >>= 7
        o.append(b | (0x80 if d > 0 else 0))
        if d == 0:
            break
    return bytes(o)


def pack_data(d):
    if isinstance(d, str):
        d = d.encode("utf-8")
    return pack_varint(len(d)) + d


def pack_port(i):
    return struct.pack(">H", i)


def get_info(host="localhost", port=25565):
    print(f"Connecting to {host}:{port}")
    # Connect
    s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    s.settimeout(10)  # Set a timeout to avoid hanging
    s.connect((host, port))
    print("Connected to server")

    try:
        # Send handshake + status request
        packet_id = b"\x00"
        protocol = pack_varint(0)
        host_data = pack_data(host)
        port_data = pack_port(port)
        next_state = b"\x01"

        handshake = packet_id + protocol + host_data + port_data + next_state
        s.send(pack_data(handshake))
        print("Handshake sent")

        # Send status request
        s.send(pack_data(b"\x00"))
        print("Status request sent")

        # Read response
        packet_length = unpack_varint(s)
        print(f"Packet length: {packet_length}")
        packet_id = unpack_varint(s)
        print(f"Packet ID: {packet_id}")
        string_length = unpack_varint(s)
        print(f"String length: {string_length}")

        # Read the response data
        d = b""
        while len(d) < string_length:
            chunk = s.recv(min(1024, string_length - len(d)))
            if not chunk:
                break
            d += chunk

        print(f"Received {len(d)} bytes of data")

        # Close our socket
        s.close()

        # Load json and return
        return json.loads(d.decode("utf8"))
    except Exception as e:
        print(f"Error in get_info: {e}")
        s.close()
        raise


client.run("xxx")
