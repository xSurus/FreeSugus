# bot.py
import os
import discord

from discord.ext import commands
from dotenv import load_dotenv

bot = commands.Bot(command_prefix='^^', intents=discord.Intents.all())

load_dotenv()
TOKEN = os.getenv('DISCORD_TOKEN')

@bot.event
async def on_ready():
    print('We have logged in as {0.user}'.format(bot))


@bot.event
async def on_message(message):
    await bot.process_commands(message)
    if message.author == bot.user:
        return

@bot.command()
async def hello(message):
    if message.author.id != 190550937264324608:
        return
    await message.send('Hello! bitch')

@bot.command()
async def ping(ctx):
    await ctx.send('Pong! {0}'.format(round(bot.latency, 1)))
spam = bot.get_channel(768600365602963496)
place = 819966095070330950
@bot.command()
async def draw(ctx, x, y, w, h, random, link):
    await ctx.message.delete()
    if ctx.author.id != 190550937264324608:
        return
    # if (x < 1000 and x >= 0 and y < 1000 and y >= 0 and x + w < 1000 and w > 0 and h+y < 1000 and h >= 0)
    await spam.send(f'<convert {x} {y} {w} {h} {random}')

@bot.command()
async def speak(ctx, *, text):
    if ctx.message.author.id == 190550937264324608:
        message = ctx.message
        await message.delete()
        await ctx.send(f"{text}")
    else:
        await ctx.send('lol mate what u doin that isn\'t a real command')

# @bot.command()
# async def april(ctx, min=1, max=999999999999999999999):
#     await ctx.channel.send("debugging")
#     for member in ctx.guild.members:
#         if min <= member.id < max:
#             try:
#                 await member.edit(nick=member.top_role.name, reason="April Fools")
#             except discord.errors.Forbidden:
#                 await ctx.channel.send(f"Can't rename {member.name}")
#     await ctx.channel.send(f".pog")

bot.run(TOKEN)