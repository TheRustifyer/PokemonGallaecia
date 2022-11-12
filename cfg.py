""" Performns tasks to autocompile the project, and replace the latest
    version of the Rust gdnative library with the new generated one
"""

import os
import platform
import subprocess
import re

from rust.autocompiler import compile_rust

# Getting the base details
current_path = os.path.abspath((os.getcwd()))
OS = platform.system()

# Setting concrete folders
'''#! Caution. This folder does not exists if the Rust project has never been compiled.
#! Necesary to handle this when it's called. 
# Could be handled here, but it's just a semi-hardoded string, so this should be fixed at runtime when this
# no existent path can broke the program.'''
rust_project_name = 'pokemon_gallaecia'

# Setting actions based on what OS is currently running
if OS == 'Windows':
    from win10toast import ToastNotifier

    godot_dir = current_path + '\\godot'
    rust_dir = current_path + '\\rust'
    rust_compiling_folder = rust_dir + '\\target\\debug'

    desired_extension = '.dll'
    command = 'copy'
    silently_override_file_switch = '/Y'
else:
    godot_dir = current_path + '/godot'
    rust_dir = current_path + '/rust'
    rust_compiling_folder = rust_dir + '/target/debug'

    desired_extension = '.so'
    command = 'cp'
    silently_override_file_switch = '-f'

# This will give us back a list with all files inside
try:
    rust_files = os.listdir(rust_compiling_folder)
    #! Let's gonna handle the posible 'path already does not exists
except FileNotFoundError:
    print('The path isn\'t already created. Please, run "cargo build" for the first time.')
    # compile_rust(current_path, rust_dir)
    rust_files = os.listdir(rust_compiling_folder)

# Finding out target dynamic library
candidates = [file for file in rust_files if re.search(rf'\{desired_extension}$', file)]
library = candidates[0]

# The final path of the desired dynamic lib
rust_dll_path = rust_compiling_folder + '\\' + library if OS == 'Windows' \
    else rust_compiling_folder + '/' + library

# Setting our base commands, flags and paths
subprocess_args = [command, rust_dll_path, godot_dir]

# Fn that makes appear a pop-up desktop notification
def desktop_result_notification(sucess):
    """ Launches a desktop pop-up showing the result of the process """
    toast = ToastNotifier()
    icon = "ferris.ico"
    dur = 10
    if sucess:
        title = "Build completed!"
        message = "Godot-Rust has been compiled succesfully.\nGo ahead and play with your new toy."
        toast.show_toast(title, message, icon_path=icon, duration=dur)

    else:
        title = "Build failed!"
        message = "Something has went wrong. Ckeck stderr for more info."
        toast.show_toast(title, message, icon_path=icon, duration=dur)

def replace_rust_dll():
    """ replaces the last compiled version of the native library
        for the new compiled one
    """
    '''First we need to declare a new thread to make Python
    wait for the compilation. That fn starts a new process separated from the
    interpreter, so Python just calls that function and keep his Python job, due to 
    for him does isn't happening inside the interpreter.'''

    # Call fn to autocompile the Rust shared library for gdnative
    compile_rust(current_path, rust_dir)

    subprocess_args.insert(1, silently_override_file_switch)
    print(subprocess_args)
    process = subprocess.Popen(
        subprocess_args,
        # shell=True,
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

        if OS == 'Windows':
            desktop_result_notification(True)
    else:
        if OS == 'Windows':
            desktop_result_notification(False)

if __name__ == '__main__':
    replace_rust_dll()
