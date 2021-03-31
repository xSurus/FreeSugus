# bot.py
import os
import discord

from discord.ext import commands
#from dotenv import load_dotenv

bot = commands.Bot(command_prefix='^^', intents=discord.Intents.all())

#load_dotenv()
TOKEN = 'NzgzMjg5NjM4Njg4Nzg0Mzk1.X8YlUg.PqRkPnBsTvg8EMcalqO9CfUItmw'

@bot.event
async def on_ready():
    print('We have logged in as {0.user}'.format(bot))


@bot.event
async def on_message(message):
    await bot.process_commands(message)
    if message.author == bot.user:
        return
@bot.event
async def on_message(message):
    await bot.process_commands(message)
    if message.author == bot.user:
        return

    if 'hello' in message.content:
        await message.channel.send('Hello! bitch')

@bot.command()
async def ping(ctx):
    await ctx.send('Pong! {0}'.format(round(bot.latency, 1)))

    
@bot.command()
async def april(ctx, min=1, max=999999999999999999999):
    await ctx.channel.send("debugging")
    for member in ctx.guild.members:
        if min <= member.id < max:
            try:
                await member.edit(nick=member.top_role.name, reason="April Fools")
            except discord.errors.Forbidden:
                await ctx.channel.send(f"Can't rename {member.name}")
    await ctx.channel.send(f".pog")
bot.run(TOKEN)