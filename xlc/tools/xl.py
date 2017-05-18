#!/usr/bin/python3

"""Frontend for the xlc X language compiler.

Compiles and executes the specified source FILE.

Usage:
    xl.py FILE

Runs the following series of steps:
    xlc FILE.xl
    clang -Wno-override-module FILE.ll -o FILE
    rm -f FILE.ll FILE
"""

import argparse
import os
import shutil
import subprocess
import sys
import tempfile


def parse_args():
    """Parse command line parameters."""
    parser = argparse.ArgumentParser(description='Frontend for the xlc X language compiler.')
    parser.add_argument('-q', '--quiet', action='store_true', dest='quiet', required=False, \
                        help='do not print the commands being executed')
    parser.add_argument('file', metavar='FILE', type=str, \
                        help='specific X source file to compile and execute')
    return parser.parse_args()


def create_tmpdir():
    """Create temporary directory where all intermediate files will be placed."""
    return tempfile.mkdtemp(prefix='xl-')


def remove_dir(directory):
    """Remove the specified directory and all of its contents."""
    shutil.rmtree(directory, ignore_errors=True)


def get_xlc_dir():
    """Best effort attempt to return the path to xlc compiler and xlrt runtime."""
    return '../target/debug'


def get_file(xl_file, old_sufix, new_sufix, tmp):
    """Return the .ll file name corresponding to the given .xl."""
    if xl_file.endswith(old_sufix):
        base = xl_file.rsplit(old_sufix, 2)[0]
    else:
        base = xl_file
    return '{}/{}{}'.format(tmp, base, new_sufix)


def constrained_path(path):
    """Remove all '..' occurrences from specified path.
       It is used to ensure all artifact paths are constrained to remain under
       temporary artifact directory."""
    return path.replace('..', '')


def get_exe_sufix():
    """Return platform-specific executable sufix (like .exe for Windows)."""
    if sys.platform.find('linux') != -1:
        return ''
    elif sys.platform.find('win') != -1:
        return '.exe'
    else:
        sys.exit('Unsupported platform \'{}\'.'.format(sys.platform))


def get_exe_cmd(xlc_dir, exe_file):
    """Return platform-specific execution command."""
    if sys.platform.find('linux') != -1:
        return 'export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:{} && {}'.format(xlc_dir, exe_file)
    elif sys.platform.find('win') != -1:
        # Windows-specific workaround to locate the runtime.
        xlc_dir = os.path.abspath('{}/deps'.format(xlc_dir))
        return 'cmd /V /C "set \"PATH=%PATH%;{}\" && {}"'.format(xlc_dir, exe_file)
    else:
        sys.exit('Unsupported platform \'{}\'.'.format(sys.platform))


def get_link_opts(xlc_dir):
    """Return platform-specific linker options."""
    if sys.platform.find('linux') != -1:
        return '-L{} -lxlrt'.format(xlc_dir)
    elif sys.platform.find('win') != -1:
        # Windows-specific workaround to identify the import library.
        xlc_dir = os.path.abspath('{}/deps'.format(xlc_dir))
        xlrt_lib = [file for file in os.listdir(xlc_dir) \
            if file.startswith('xlrt-') and file.endswith('.dll.lib')][0]
        return '-L{} -l{}'.format(xlc_dir, xlrt_lib)
    else:
        sys.exit('Unsupported platform \'{}\'.'.format(sys.platform))


def mk_file_dir(file):
    """Ensures parent directory for the given file does exist."""
    directory = os.path.dirname(file)
    os.makedirs(directory, mode=0o700, exist_ok=True)


def xlc_compile(xlc, xl_file, ll_file, run):
    """Compile X source code into LLVM IR."""
    run('Compile...', '{} {} --output {}'\
            .format(xlc, xl_file, ll_file))


def clang_build(xlc_dir, ll_file, exe_file, run):
    """Build the executable by using clang."""
    run('Build...', 'clang -Wno-override-module {} {} -o {}'\
            .format(get_link_opts(xlc_dir), ll_file, exe_file))


def execute(exe_cmd, run):
    """Execute the compiled binary."""
    run('Execute...', '{}'\
            .format(exe_cmd))


def xlc_frontend():
    """Frontend for the xlc X language compiler."""
    args = parse_args()

    def run(msg, cmd):
        """Print 'msg', 'cmd' and execute 'cmd' taking '--quiet' into account.
           Stops if 'cmd' execution failed."""
        def display(msg):
            """Print 'msg' taking '--quiet' into account."""
            if not args.quiet:
                print('{}'.format(msg), flush=True)
        display('{}'.format(msg))
        display('  {}'.format(cmd))
        process = subprocess.run(cmd, shell=True)
        if process.returncode != 0:
            sys.exit(process.returncode)

    tmp = create_tmpdir()

    xlc_dir = get_xlc_dir()
    xlc = '{}/xlc'.format(xlc_dir)
    xl_file = args.file
    ll_file = get_file(constrained_path(xl_file), '.xl', '.ll', tmp)
    exe_file = get_file(constrained_path(xl_file), '.xl', get_exe_sufix(), tmp)
    (xlc_dir, xlc, xl_file, ll_file, exe_file) = map(os.path.abspath, \
        (xlc_dir, xlc, xl_file, ll_file, exe_file))

    mk_file_dir(ll_file)

    xlc_compile(xlc, xl_file, ll_file, run)
    clang_build(xlc_dir, ll_file, exe_file, run)
    execute(get_exe_cmd(xlc_dir, exe_file), run)

    remove_dir(tmp)


xlc_frontend()
