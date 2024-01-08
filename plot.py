#!/usr/bin/env python3

import numpy as np
import matplotlib.pyplot as plt
import glob
import argparse

parser = argparse.ArgumentParser()
parser.add_argument('-f', '--files', nargs='*', required=False, type=int, default=[-1], help="list of files to plot")
args = parser.parse_args()

filenames = glob.glob("./*.dat")
print(filenames)

if args.files != [-1]:
    filenames_temp = []
    for (i,filename) in enumerate(filenames):
        if i in args.files:
            filenames_temp += [filename]
    filenames = filenames_temp

files = [np.loadtxt(filename) for filename in filenames]

def pos2(ang1, ang2):
    return [cos(ang1), sin(ang1)]

def pos2(ang1, ang2):
    return [cos(ang1) + cos(ang2), sin(ang1) + sin(ang2)]


line_colors = ['#1f77b4', '#ff7f0e', '#2ca02c', '#d62728', '#9467bd', '#8c564b', '#e377c2', '#7f7f7f', '#bcbd22', '#17becf']

for (i,(name, file)) in enumerate(zip(filenames,files)):
    color = line_colors[i %  10]
    # name = name.replace()
    # plt.plot(file[:,0], np.log(file[:, 1]), label=name)
    plt.plot(file[:,0], (file[:, 1] + np.pi) % ( 2.0 * np.pi) - np.pi, "-", color=color, label="$\phi_1$ in {}".format(name))
    plt.plot(file[:,0], 7 + file[:, 2], "--", color=color, label="$p_1$ in {}".format(name))
    plt.plot(file[:,0], 21 + (file[:, 3] + np.pi) % ( 2.0 * np.pi) - np.pi, "-", color=color, label="$\phi_2$ in {}".format(name))
    plt.plot(file[:,0], 35 + file[:, 4],"--", color=color, label="$p_2$ in {}".format(name))

plt.title("different solutions $\mathbf{x}(t) = (\phi_1(t), p_1(t), \phi_2(t), p_2(t))$ sorry für die fehlende achsenbeschriftung usw.")

plt.legend()
plt.show()