import os
import sys
import platform
import subprocess
import re

from rust.autocompiler import compile_rust

# Getting the base details
current_path = os.path.abspath((os.getcwd()))
OS = platform.system()

# The base dirs
godot_dir = current_path + '\\godot'
rust_dir = current_path + '\\rust'

# Setting concrete folders
'''#! Caution. This folder does not exists if the Rust project has never been compiled.
#! Necesary to handle this when it's called. 
# Could be handled here, but it's just a semi-hardoded string, so this should be fixed at runtime when this
# no existent path can broke the program.'''
rust_compiling_folder = rust_dir + '\\target\\debug'
rust_project_name = 'learn_programming_with_godot'

# Setting actions based on what OS is currently running
if OS == 'Windows':
    desired_extension = '.dll'
    command = 'copy'
    silently_override_file_switch = '/Y'
elif OS == 'Linux':
    desired_extension = '.dylib'
    command = 'cp'
else:
    pass # Not interested in working with Mac nowadays...

# This will give us back a list with all files inside
try:
    rust_files = os.listdir(rust_compiling_folder) 
    #! Let's gonna handle the posible 'path already does not exists
except Exception:
    print('The path isn\'t already created. Please, run "cargo build" for the first time.')
    compile_rust(current_path, rust_dir)
    rust_files = os.listdir(rust_compiling_folder)

# Finding out target dynamic library
candidates = [file for file in rust_files if re.search(rf'\{desired_extension}$', file)]
library = candidates[0]

# The final path of the desired dynamic lib
rust_dll_path = rust_compiling_folder + '\\' + library

# Setting our base commands, flags and paths
subprocess_args = [command, rust_dll_path, godot_dir]

def replace_rust_dll():
    if OS == 'Windows':
        '''First we need to declare a new thread to make Python
        wait for the compilation. That fn starts a new process separated from the
        interpreter, so Python just calls that function and keep his Python job, due to 
        for him does isn't happening inside the interpreter.'''

        # Call fn to autocompile the Rust .dll
        compile_rust(current_path, rust_dir)

        subprocess_args.insert(1, silently_override_file_switch)
        process = subprocess.Popen(
            subprocess_args, 
            shell=True, 
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
        )
        
        process_stderr = process.stderr.read(100)
        # //TODO: Still have to write the code to handle stderr
        
        if not process_stderr:
            process_stdout = process.stdout.read(100)
            execution_result = process_stdout.decode('UTF-8').strip()
            print('Coping files, wait...')
            print('\t' + execution_result + f'\n\tFile: {library} to {godot_dir}')

        '''To avoid use PIPES on stdout and stderr...
        and in W10, prompt gets waiting if no PIPE de process... so...'''
        # subprocess.call(['exit'], shell=True)

    else:
        pass
    
if __name__ == '__main__':
    replace_rust_dll()