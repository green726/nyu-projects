import random
import scipy.constants
import numpy as np
import matplotlib as mpl
import matplotlib.pyplot as plt

plt.style.use('classic')

#example energy function

def square_func(x):
    return x**2

def multi_minima_func(x):
    return (2 * pow(x, 4)) - (3 * pow(x, 3)) - (7 * pow(x, 2)) + (2 * x) + 1



#monte-carlo random walk with 1d constraint
def mc_walk_constrained(pos, max_step_size, e_max, energy_f):

    e = energy_f
    while (True):
        #random step
        step_dist = random.uniform(-max_step_size, max_step_size)
        new_pos = pos + step_dist

        #check step's validity
        if (e(new_pos) <= e_max):
            return new_pos

#1-dimensional nested sampling

def nested_sampling(energy_function, iterations, prior_points):

    #local vars
    e = energy_function
    max_energies = []
    replicas = prior_points

    #main loop (over configurations)
    for n in range (0, iterations):
        #max energy values
        max_energy = 0
        max_energy_idx = -1

        #inner loop to find max energy with current replicas
        for i in range(len(replicas)):

            replica_energy = e(replicas[i])
            if replica_energy > max_energy:
                max_energy = replica_energy
                max_energy_idx = i

        #remove the max energy replica and replace it with a new one mc-walked from a random remaining replica
        max_energies.append(max_energy)
        del replicas[max_energy_idx]
        random_replica = random.choice(replicas)
        replicas.append(mc_walk_constrained(random_replica, 1, max_energy, e))
    return max_energies

# test_result = nested_sampling(square_func, 1000, [random.uniform(-50, 50) for i in range(100)])

# print("nested sampling ran on square root function with 1000 iterations and 100 prior points:\n whole arrray: " + str(test_result) + "\n\n\n\n\n first: " + str(test_result[0]) + "\n last: " + str(test_result[-1]))
#

def free_energy():
    iterations = 1000
    k = 100
    sample = [random.uniform(-k/2, k/2) for i in range(k)]
    
    # fig, ax = plt.subplots()
    # ax.plot(sorted(sample), [square_func(x) for x in sorted(sample)])
    # plt.show()


    # print(sample)
    ns_result = nested_sampling(multi_minima_func, iterations, sample)

    density_of_states = []
    free_energies = []
    
    for i in range(0, iterations):
        density = (1/(k + 1)) * pow((k/(k + 1)), i)
        density_of_states.append(density)
        free_energies.append(-np.log(density))

    print("density of states: " + str(density_of_states))

    x_axis = []

    for i in range(0, len(ns_result)):
        if i != len(ns_result) - 1:
            x_axis.append((ns_result[i] + ns_result[i + 1])/2)
        else:
            x_axis.append(ns_result[i])

    fig, ax = plt.subplots()

    ax.plot(x_axis, free_energies)
    plt.show()

    return free_energies

free_energy()
# print("free energy: " + str(free_energy()))
