import random

bunny = 0
bunnyMin = 0
bunnyMax = 99

def startBunny():
    global bunny
    global bunnyMin
    global bunnyMax
    bunny = random.randrange(bunnyMin, bunnyMax+1)
    print(f"Bunny starting at: {bunny}")

def moveBunnyMove():
    global bunny
    global bunnyMin
    global bunnyMax

    if bunny == bunnyMin:
        bunny += 1
    elif bunny == bunnyMax:
        bunny -= 1
    elif random.random() > .5:
        bunny += 1
    else:
        bunny -= 1

def guess_location(in_guess):
    global bunny
    if bunny == in_guess:
        return True
    else:
        moveBunnyMove()
        return False

def main():

    print(f"Welcome to Bunny Land")

    startBunny()

    bunnyFound = False
    spot = 0
    total_repeat = 2
    current_repeat = 0

    while not bunnyFound:
 
        bunnyFound = guess_location(spot)
        
        if not bunnyFound:
            current_repeat += 1
            if current_repeat >= total_repeat:
                spot += 1
                current_repeat = 0
                total_repeat += 1

    global bunny
    print(f"Bunny found at {spot} {bunny}")
