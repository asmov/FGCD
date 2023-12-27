#!/bin/env python3
##
#  FGCD: Strategy guide & data collection for fighting games
#  Copyright (C) 2023 Asmov LLC
#
#  This program is free software: you can redistribute it and/or modify
#  it under the terms of the GNU General Public License as published by
#  the Free Software Foundation, either version 3 of the License, or
#  (at your option) any later version.
#
#  This program is distributed in the hope that it will be useful,
#  but WITHOUT ANY WARRANTY; without even the implied warranty of
#  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
#  GNU General Public License for more details.
# 
#  You should have received a copy of the GNU General Public License
#  along with this program.  If not, see <https://www.gnu.org/licenses/>.
##

import os;
import os.path as path;
import shutil;
import glob;

PROJECT_DIR = path.realpath(path.dirname(__file__) + '/..')
WORKSPACE_DIR = path.realpath(path.dirname(__file__) + '/../..')

TARGET_DIR = WORKSPACE_DIR + '/target'
GAME_DATA_DIR = PROJECT_DIR + '/games'
PLATFORM_DATA_DIR = PROJECT_DIR + '/platforms'

DATA_TARGET_DIR = TARGET_DIR + '/data'

GAME_DATA_TARGET_DIR = DATA_TARGET_DIR + '/games'
PLATFORM_DATA_TARGET_DIR = DATA_TARGET_DIR + '/platforms'

def clean():
    if path.exists(DATA_TARGET_DIR):
        shutil.rmtree(DATA_TARGET_DIR)

def setup():
    os.makedirs(DATA_TARGET_DIR, 0o755, exist_ok=True)

def compile():
    compile_dir(GAME_DATA_DIR, GAME_DATA_TARGET_DIR)
    compile_dir(PLATFORM_DATA_DIR, PLATFORM_DATA_TARGET_DIR)

def compile_dir(data_dir, target_dir):
    for name in os.listdir(data_dir):
        item_data_dir = path.join(data_dir, name)
        if not os.path.isdir(item_data_dir):
            continue

        item_target_dir = path.join(target_dir, name)
        os.makedirs(item_target_dir, 0o755)
        
        os.chdir(item_data_dir)
        for file in glob.glob(r'*.fods'):
            shutil.copy(file, item_target_dir)

        compile_dir(item_data_dir, item_target_dir)

def main():
    clean()
    setup()
    compile()
    
if __name__ == '__main__':
    main()
