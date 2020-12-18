from os import path
import os
import sys
import requests

dir_override = ''


def make_main_py_template(day):
    template = """from aocd.models import Puzzle
puzzle = Puzzle(year=2020, day={})
data = puzzle.input_data



""".format(day)
    return template


def write_string_to_file(file, s):
    f = open(file, 'w')
    f.write(s)
    f.close()


def day_dir(day):
    return "day_{}".format(day)


def check_day_folder_exists(day):
    return path.exists(day_dir(day))


def make_day_folder(day):
    os.mkdir(day_dir(day))


def download_input(day, token):
    res = requests.get('https://adventofcode.com/2020/day/{}/input'.format(day), cookies={'session': token})
    write_string_to_file('{}/input.txt'.format(day_dir(day)), res.text)


def create_main_py(day):
    write_string_to_file('{}/main.py'.format(day_dir(day)), make_main_py_template(day))


def get_auth_token():
    f = open('.auth', 'r')
    token = f.read()
    f.close()
    return token


d = sys.argv[1]
if len(sys.argv) > 2:
    dir_override = sys.argv[2]
if check_day_folder_exists(d):
    print('day exists')
    exit(1)

make_day_folder(d)
download_input(d, get_auth_token())
create_main_py(d)
