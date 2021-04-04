import subprocess
import os

def compile_rust(prev_path, desired_path):
    os.chdir(desired_path)

    process = subprocess.Popen(
            'cargo build', 
            shell=True, 
            stdout=subprocess.PIPE,
        )

    process_stdout = process.stdout.read(1000)
    execution_result_out = process_stdout.decode('UTF-8').strip()
    if not process_stdout:
        print('\t*** Rust code compilation successfully ***')
    else:
        print(f'stdout -> {execution_result_out}')

    subprocess.call(['exit'], shell=True)

    os.chdir(prev_path)

