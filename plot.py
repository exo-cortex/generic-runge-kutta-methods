#!/usr/bin/env python3

import numpy as np
import matplotlib.pyplot as plt
import glob

filenames = glob.glob("./*.dat")
print(filenames)

files = [np.loadtxt(filename) for filename in filenames]

for (name, file) in zip(filenames,files):
    # plt.plot(file[:,0], np.log(file[:, 1]), label=name)
    plt.plot(file[:,0], file[:, 1], label=name)

plt.legend()
plt.show()