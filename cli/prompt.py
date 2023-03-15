import sys
from cli.getch import _Getch


def yes_no(s, default=0):
    if default == 1:
        print(s + " yes (no)", end="")
    else:
        print(s + " (yes) no", end="")
    sys.stdout.flush()
    cursor_move = False
    current = 1
    while True:
        g = _Getch().__call__()
        if cursor_move:
            if (g == b"M") or (g == b"P"):
                current = 1
            elif (g == b"K") or (g == b"H"):
                current = 0
            cursor_move = False
        else:
            if g == b"\x03":
                exit(0)
            elif g == g == b"\r":
                break
            elif (g == b"a") or (g == b"1"):
                current = 0
            elif g == b"b" or (g == b"2"):
                current = 1
            elif g == b"\xe0":
                cursor_move = True
        sys.stdout.write("\u001b[1000D")  # Move left
        if current == 0:
            print(s + " (yes) no", end="")
        else:
            print(s + " yes (no)", end="")
        sys.stdout.flush()
    bool_result = [True, False][current]
    return bool_result


def multi_choice(options, default=0):
    current = default
    cursor_move = False
    num_options = len(options)
    for i in range(0, 4):
        if current != i:
            print("  " + options[i])
        else:
            print("> " + options[i])
    while True:
        g = _Getch().__call__()
        if cursor_move:
            if (g == b"M") or (g == b"P"):
                if current != len(options) - 1:
                    current += 1
            elif (g == b"K") or (g == b"H"):
                if current != 0:
                    current -= 1
            cursor_move = False
        else:
            if g == b"\x03":
                raise KeyboardInterrupt
            elif g == g == b"\r":
                break
            elif num_options > 0 and ((g == b"a") or (g == b"1")):
                current = 0
            elif num_options > 1 and (g == b"b" or (g == b"2")):
                current = 1
            elif num_options > 2 and (g == b"c" or (g == b"3")):
                current = 2
            elif num_options > 3 and (g == b"d" or (g == b"4")):
                current = 3
            elif g == b"\xe0":
                cursor_move = True
        sys.stdout.write("\u001b[1000D")  # Move left
        sys.stdout.write("\u001b[" + str(4) + "A")  # Move up
        for i in range(0, 4):
            if current != i:
                print("  " + options[i])
            else:
                print("> " + options[i])
        sys.stdout.flush()
    return current
