import shutil
import csv
import os
import re
import ctypes
from subprocess import call, check_output, STDOUT

def main():
    if shutil.which('junction') is None:
        print("Junction command not found! Check the README.")

    for old_path, new_path in save_paths():
        move_and_junction(old_path, new_path)
    print('Done')

def save_paths():
    with open('list.csv', newline='') as csvfile:
        paths = csv.reader(csvfile, delimiter=',', quotechar='"',
                           skipinitialspace=True, strict=True)
        for old_path, new_path in paths:
            old_path = os.path.expanduser("~\\{}".format(old_path))
            new_path = os.path.expanduser("~\\Saved Games\\{}".format(new_path))
            yield old_path, new_path

def move_and_junction(old_path, new_path):
    if os.path.exists(old_path):
        if not is_junction(old_path):
            if os.path.exists(new_path):
                print("Can't move {}".format(old_path))
                print("Conflicting folder already exists: {}".format(new_path))
                return
            move_save(old_path, new_path)
            link_save(old_path, new_path)
            hide_directory(old_path)
        else:
            print("Already junctioned: {}".format(old_path))

def is_junction(path):
    '''
    Determine if the folder at the specified path is a junction.
    Unfortunately, if the junction command ever changes it's output,
    this will break.
    '''
    output = check_output('junction "{}"'.format(path), universal_newlines=True)
    result = re.search('No reparse points found', output)
    return result is None

def move_save(old_path, new_path):
    print("Moving...  {} -> {}".format(old_path, new_path))
    shutil.move(old_path, new_path)

def link_save(old_path, new_path):
    print("Linking... {} -> {}".format(old_path, new_path))
    FNULL = open(os.devnull, 'w')
    call('junction "{}" "{}"'.format(old_path, new_path), stdout=FNULL, stderr=STDOUT)

def hide_directory(path):
    ctypes.windll.kernel32.SetFileAttributesW(path, 2)

if __name__ == "__main__":
    main()

