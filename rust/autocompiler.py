import subprocess
import os

def compile_rust(path):
    os.chdir(path)
    print('Path of __init__.py file ' + os.path.abspath(os.getcwd()))
    process = subprocess.Popen(
            'cargo build', 
            shell=True, 
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
        )

    process_stderr = process.stderr.read(100)
    # //TODO: Still have to write the code to handle stderr
    
    if not process_stderr:
        process_stdout = process.stdout.read(1000)
        execution_result = process_stdout.decode('UTF-8').strip()
        print(execution_result)

    subprocess.call(['exit'], shell=True)

    os.chdir(path)
    print('Path of __init__.py file' + os.path.abspath(os.getcwd()))