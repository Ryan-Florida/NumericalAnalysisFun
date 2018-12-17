#!/usr/bin/env python

#This file accepts as input the heat grid output by the DisplayResults function
#of main.rs
from sys import stdin
import matplotlib.pyplot as plt
from numpy import meshgrid, linspace

heat  = [line.strip('\n').split() for line in stdin.readlines()]
heat2 = []
for h in heat:
    heat2.append([float(_) for _ in h])

#These will have to be changed if you manipulate the H/K values in the example
#files. k = K and h = H
h = 1e-1
k = 1e-2
x    = [_*h for _ in range(len(heat[0]) + 1)]
y    = [_*k for _ in range(len(heat))]

x, y = meshgrid(x,y)

plt.pcolormesh(x, y, heat2)
plt.colorbar()
plt.title('Numerical Solution to $\\frac{\partial{u}}{\partial{t}} = \\frac{\partial^2{u}}{\partial{x^2}}$')
plt.xlabel('x (meters)')
plt.ylabel('t (seconds)')
plt.show()
