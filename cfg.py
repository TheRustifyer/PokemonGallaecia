import os
import subprocess

user_path = os.path.expanduser("~")
current_path = os.path.abspath((os.getcwd()))

# for root, dirs, files in os.walk(current_path):
#     print(root, dirs, files)
print(os.listdir(current_path))
# print(subprocess.Popen(['copy']))
print(current_path)
print(user_path)