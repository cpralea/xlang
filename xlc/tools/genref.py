#!/usr/bin/python3

"""Generate .xlc and .out from .xl.

Usage:
    genref.py [--root ROOT] [FILE [FILE ...]]

For each FILE.xl, either directly specified or found under ROOT/xl/,
execute:
    xlc --verbose --no-output ROOT/xl/FILE.xl > ROOT/xlc/FILE.xlc
    xl.py ROOT/xl/FILE.xl > ROOT/out/FILE.out
"""

import argparse
import os
import subprocess


def parse_args():
    """Parse command line parameters."""
    parser = argparse.ArgumentParser(description='Generate .xlc from .xl.')
    parser.add_argument('--root', metavar='ROOT', type=str, dest='root_dir', \
                        required=False, default='tests', \
                        help='root directory for xl/*.xl and xlc/*.xlc')
    parser.add_argument('files', metavar='FILE', type=str, nargs='*', \
                        help='specific file to process')
    return parser.parse_args()


def get_dirs(root_dir):
    """Get '/xl', '/xlc' and '/out' directoryes relative to 'root_dir'."""
    return ('{0}/xl'.format(root_dir), '{0}/xlc'.format(root_dir), '{0}/out'.format(root_dir))


def get_files(args_xl_files, xl_dir):
    """Gets the .xl files to process. Does so either from command line 'xl_files'
       or by listing all .xl files under 'xl_dir'."""
    files = args_xl_files
    if not files:
        files = [file.rpartition('.')[0] for file in os.listdir(xl_dir) if file.endswith('.xl')]
    return files


def genxlc(file, xl_dir, xlc_dir):
    """Generate .xlc from .xl."""
    cmd = 'cargo run -q -- --verbose --no-output {}/{}.xl > {}/{}.xlc'\
                .format(xl_dir, file, xlc_dir, file)
    process = subprocess.run(cmd, shell=True)
    return process.returncode


def genout(file, xl_dir, out_dir):
    """Generate .out from .xl."""
    cmd = '{} --quiet {}/{}.xl > {}/{}.out'\
                .format(os.path.normpath('tools/xl.py'), xl_dir, file, out_dir, file)
    process = subprocess.run(cmd, shell=True)
    return process.returncode


def genref():
    """Generate .xlc and .out from .xl."""
    args = parse_args()

    files = args.files
    (xl_dir, xlc_dir, out_dir) = get_dirs(args.root_dir)

    files = get_files(files, xl_dir)
    for file in files:
        print('{}...'.format(file), flush=True)
        if genxlc(file, xl_dir, xlc_dir) == 0:
            genout(file, xl_dir, out_dir)


genref()
