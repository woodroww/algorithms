

def move(f, t):
    print(f"Move disc from {f} to {t}")

move("A", "C")

def moveVia(f, v, t):
    move(f, v)
    move(v, t)

moveVia("A", "B", "C")

# n-number of disks
# f-from position
# h-helper position
# t-target position
def hanoi(n, f, h, t):
    if n == 0:
        return
    hanoi(n-1, f, t, h)
    move(f, t)
    hanoi(n-1, h, f, t)

hanoi(4, "A", "B", "C")

