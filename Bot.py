# bot.py
import os

import discord
from dotenv import load_dotenv

load_dotenv()
TOKEN = os.getenv('DISCORD_TOKEN')

client = discord.Client()
@client.event
async def on_message(message):
    mention = f'<@!{190550937264324608}>'
    if mention in message.content:
        await message.channel.send("You mentioned me")

@client.event
async def on_ready():
    print(f'{client.user} has connected to Discord!')

client.run(TOKEN)
