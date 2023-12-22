import sys


def solve(data):
    print(data)


def main():
    for line in sys.stdin:
        data = int(line)
        solve(data)


if __name__ == "__main__":
    main()
