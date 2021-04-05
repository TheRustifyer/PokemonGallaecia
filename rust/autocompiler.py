import subprocess
import os

def compile_rust(prev_path, desired_path):
    os.chdir(desired_path)

    process = subprocess.Popen(
            'cargo build', 
            shell=True, 
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE
        )

    '''NOTE: Cargo build command gives his output always on stderr,
    regardless of whether the program compiled successfully or not, 
    which is a little bit weird. BTW, we are capturing stdout in order to make the program
    wait for the complete execution of the process, otherwise Python process will running without
    wait Rust compilation to ends. (When subprocess with cargo build spawns, it's independent of the
    Python interpreter thread.'''

    process_stdout = process.stdout.read(1000)
    process_stderr = process.stderr.read(1000)
    execution_result_out = process_stdout.decode('UTF-8').strip()
    if not process_stdout:
        print('\t*** Rust code compilation successfully ***')
    else:
        print(f'stdout -> {execution_result_out}')

    subprocess.call(['exit'], shell=True)

    os.chdir(prev_path)

