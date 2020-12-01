# bot.py
import os
from discord.ext import commands
import discord
from dotenv import load_dotenv

load_dotenv()
TOKEN = os.getenv('DISCORD_TOKEN')

client = commands.Bot(command_prefix="^^")
@client.event
async def on_ready():
    print(f'{client.user} has connected to Discord!')

@client.event
async def on_message(message):
    if message.author.bot:
        return
    await client.process_commands(message)

@client.command()
async def test(ctx):
    await ctx.send("reply")


client.run(TOKEN)
