from random import uniform
import matplotlib as mpl
import matplotlib.pyplot as plt
from numpy.lib import math

# def energy(x) :
#     return x
#
# def sample():
#     return uniform(0, 10)
#
#
# x = []
# y = []
# n = 10000
#
#
# for i in range(n):
#     this_x = sample()
#     x.append(this_x)
#     y.append(energy(this_x))
#
# x.sort()
# y.sort()
#
# # print(len(x))
# # print(len(y))
#
#
# plt.scatter(x, y)
# plt.show()


y = []
x =[]


inputXFile = open('inputX.txt', 'r')
# linesX = inputXFile.readlines()
linesX = [5, 4, 3, 2, 1]

for line in linesX:
    x.append(float(line))

inputYFile = open('inputY.txt', 'r')
# linesY = inputYFile.readlines()
linesY = [4, 3, 2, 1, 0]
 
for line in linesY:
    y.append(float(line))


cm_y = []
y_sum = 0

for i in range(len(y)):
    cm_y.append(y_sum)
    y_sum += (y[len(y) - i - 1])
cm_y.reverse()

plt.plot(x, cm_y)
plt.show()
