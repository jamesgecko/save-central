import shutil
import csv
import os
import sys
import re
import ctypes
from subprocess import call, check_output, STDOUT
from ntfsutils  import junction
import options

def main():
    if (sys.version_info[0] < 3) or (sys.version_info[0] == 3 and sys.version_info[1] < 2):
        sys.exit("This script must be run with Python 3.2 or greater")

    create_junctions()
    print('Done')

def create_junctions():
    for old_path, new_path in save_paths():
        move_and_junction(old_path, new_path)

def save_paths():
    with open('list.csv', newline='') as csvfile:
        paths = csv.reader(csvfile, delimiter=',', quotechar='"',
                           skipinitialspace=True, strict=True)
        for old_path, new_path in paths:
            old_path = os.path.expanduser("~\\{}".format(old_path))
            new_path = os.path.expanduser("~\\Saved Games\\{}".format(new_path))
            yield old_path, new_path

    with open('unqualified_list.csv', newline='') as csvfile:
        paths = csv.reader(csvfile, delimiter=',', quotechar='"',
                           skipinitialspace=True, strict=True)
        for old_path, new_path in paths:
            old_path = os.path.expandvars(old_path)
            old_path = re.sub("%steamlibrary%", options.steamlibrary, old_path)
            new_path = os.path.expanduser("~\\Saved Games\\{}".format(new_path))
            yield old_path, new_path

def move_and_junction(old_path, new_path):
    if os.path.exists(old_path):
        if not junction.isjunction(old_path):
            if os.path.exists(new_path):
                print("Can't move {}".format(old_path))
                print("Conflicting folder already exists: {}".format(new_path))
                return
            move_save(old_path, new_path)
            link_save(old_path, new_path)
            hide_directory(old_path)
        else:
            print("Already junctioned: {}".format(old_path))

def restore_junctions():
    for old_path, new_path in save_paths():
        restore_junction(old_path, new_path)

def restore_junction(old_path, new_path):
    if os.path_exists(new_path):
        if os.path_exists(old_path):
            if junction.isjunction(old_path):
                print("Already junctioned: {}".format(old_path))
            else:
                print("Cannot junction; conflicting folder already exists: {}".format(old_path))
        else:
            link_save(old_path, new_path)
            hide_directory(old_path)

def move_save(old_path, new_path):
    print("Moving...  {} -> {}".format(old_path, new_path))
    shutil.move(old_path, new_path)

def link_save(old_path, new_path):
    print("Linking... {} -> {}".format(old_path, new_path))
    junction.create(new_path, old_path)

def hide_directory(path):
    ctypes.windll.kernel32.SetFileAttributesW(path, 2)

if __name__ == "__main__":
    main()

