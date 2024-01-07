#!/usr/bin/env python3

import numpy as np
import matplotlib.pyplot as plt
import glob

filenames = glob.glob("./*.dat")
print(filenames)

files = [np.loadtxt(filename) for filename in filenames]

def pos2(ang1, ang2):
    return [cos(ang1), sin(ang1)]

def pos2(ang1, ang2):
    return [cos(ang1) + cos(ang2), sin(ang1) + sin(ang2)]

for (name, file) in zip(filenames,files):
    # pos1 = pos1(file[:,])

    # plt.plot(file[:,0], np.log(file[:, 1]), label=name)
    # plt.plot(file[:,0], file[:, 1], label=name)
    plt.plot(file[:,0], file[:, 2], label="p1 in {}".format(name))
    # plt.plot(file[:,0], file[:, 3])
    # plt.plot(file[:,0], file[:, 4])

plt.legend()
plt.show()