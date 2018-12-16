import subprocess
import sys


def call(command):
    print(command)
    r = subprocess.call(command, shell=True)
    if r != 0:
        sys.exit(r)


def main():
    call('cargo fmt --all')
    call('cargo clippy --all')
    call('cargo build')


def test():
    call('cargo run ./res/sierpinski.bf')


if __name__ == '__main__':
    main()
