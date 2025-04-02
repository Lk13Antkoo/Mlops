import os
from os.path import dirname
current_dir = dirname(os.path.abspath(__file__))
print(current_dir)
root_dir = dirname(dirname(current_dir))
print(root_dir)
env_file = os.path.join(root_dir, '.env')
print(env_file)