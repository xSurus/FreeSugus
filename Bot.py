# bot.py
import os
import discord
from PIL import Image
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


""" await ctx.send("so far so good")
    for x in range(w):
        for y in range(h):
            pixList = [x,y]
            currPix = im.getpixel((x,y))
            await place.send(f".place setpixel {x}, {y}, {currPix}") 
"""
def rgb2hex(r, g, b):
    return '#{:02x}{:02x}{:02x}'.format(r, g, b)

@bot.command()
async def draw(ctx, image_name, x, y):
    place = bot.get_channel(819966095070330950)
    await ctx.message.delete()
    if ctx.message.author.id != 190550937264324608:
        return
    im = Image.open(f"{image_name}.png")
    # datalist = list(im.getdata(im.getbands())) # might work not sure yet
    pixels = im.convert("RGBA").load()
    h, w = im.size  # height, width, channel
    pixels_to_draw = []

    for x in range(w):
        for y in range(h):
            hex_color = '#%02x%02x%02x' % pixels[x,y][:3]
            await place.send(f".place setpixel {x} {y} {hex_color}")
    await ctx.send("Done?")
    

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