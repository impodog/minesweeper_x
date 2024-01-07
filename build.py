import subprocess
import os
import sys
import re
import shutil
import argparse

def touch_dir(path):
    if os.path.exists(path):
        assert os.path.isdir(path)
    else:
        print("Creating directory: " + path)
        os.mkdir(path)
def copy(src, dst):
    print("Copying: " + src + " -> " + dst)
    if os.path.isdir(src):
        shutil.copytree(src, dst, dirs_exist_ok=True)
    else:
        shutil.copy(src, dst)

def build(target):
    is_wasm = False
    if target.startswith("wasm"):
        suffix = ".wasm"
        is_wasm = True
    elif sys.platform == "win32":
        suffix = ".exe"
    else:
        suffix = ""

    if target is None:
        toolchains = subprocess.run(["rustup", "toolchain", "list"], stdout=subprocess.PIPE).stdout.decode("utf-8")
        target = re.search(r"([a-z0-9\-_]+)\s+\(default\)", toolchains).group(1)
    target_list = target.split('-')
    if re.match(r"(stable|beta|nightly|\d+\.\d+(\.\d+)?)", target_list[0]):
        target_list.pop(0)
    target = "-".join(target_list)

    target_info = target_list[0] + "-" + target_list[2]

    package_name = "minesweeper_x-" + target_info
    package_dir = "package/" + package_name

    subprocess.run(["cargo", "build", "--release", "--target", target])

    executable = "target/%s/release/minesweeper_x%s" % (target, suffix)
    assert os.path.exists(executable)

    touch_dir("package")
    touch_dir(package_dir)
    copy(executable, package_dir)
    copy("README.md", package_dir)
    copy("LICENSE", package_dir)
    copy("assets", package_dir + "/assets")



parser = argparse.ArgumentParser(description="Tool to build minesweeper_x release packages")
parser.add_argument("--target", "-t", default=None, help="Target platform to build for")
args = parser.parse_args()

curdir = os.path.dirname(os.path.realpath(__file__))
os.chdir(curdir)

build(args.target)
