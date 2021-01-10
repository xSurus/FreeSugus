# bot.py
import os
import discord

from discord.ext import commands
from dotenv import load_dotenv

bot = commands.Bot(command_prefix='^^')

load_dotenv()
TOKEN = os.getenv('DISCORD_TOKEN')

@bot.event
async def on_ready():
    print('We have logged in as {0.user}'.format(bot))


@bot.event
async def on_message(message):
    if message.author == bot.user:
        return
@bot.event
async def on_message(message):
    if message.author == bot.user:
        return

    if 'hello' in message.content:
        await message.channel.send('Hello!')

        
@bot.command()
async def ping(ctx):
    await ctx.send('Pong! {0}'.format(round(bot.latency, 1)))        



my_pings = []


@bot.event
async def ping(ctx):
    await ctx.send('Pong! {0}'.format(round(bot.latency, 1)))
"""
@bot.event
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
"""
bot.run(TOKEN)