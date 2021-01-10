# bot.py
import os

import discord

from discord.ext import commands
"""from dotenv import load_dotenv

load_dotenv()
TOKEN = os.getenv('DISCORD_TOKEN')
"""
TOKEN = 'NzgzMjg5NjM4Njg4Nzg0Mzk1.X8YlUg.aq92poPvs2OAla7Ngn5goeYOiWw'
my_pings = []
bot = commands.Bot(command_prefix='er')
client = discord.Client()

@bot.command()
async def test(ctx, arg):
    await ctx.send(arg)
    
@bot.command()
async def ping(ctx):
    await ctx.send('Pong! {0}'.format(round(bot.latency, 1)))

@client.event
async def on_message(message):
    if message.author == client.user:
        return
    if message.author.bot: 
        return
    mention = f'<@!{190550937264324608}>'
    if mention in message.content:
        await message.channel.send("How dare you mention my creator!")
        my_pings.append(str(message.author), message.timestamp, message.content)

async def on_message(message):
    mypings = f'mypings'
    if 'mypings' in message.content:
        await message.channel.send(my_pings)
@client.event
async def on_ready():
    print(f'{client.user} has connected Discord!')

client.run(TOKEN)
