import random
import scipy.constants
import numpy as np
import matplotlib as mpl
import matplotlib.pyplot as plt
import time

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

def nested_sampling(energy_function, iterations, prior_points, debug):

    #local vars
    e = energy_function
    max_energies = []
    replicas = prior_points

    y_lim_max = [e(r) for r in sorted(replicas)][-1]


    if debug:
        fig, ax = plt.subplots()

        # ax.plot(x_axis, density_of_states)
        
    #main loop (over configurations)
    for n in range (0, iterations):
        #max energy values
        max_energy = 0
        max_energy_idx = 0

        if debug:
            ax.clear()
            ax.plot(sorted(replicas), [e(r) for r in sorted(replicas)])
            # ax.scatter(range(0, n), max_energies)
            plt.xlim(-10, 10)
            plt.ylim(-20, (y_lim_max * (1/((n + 1)**2)) - 15))
            # plt.ylim(-30, 30)
            plt.pause(.000001)
    
        #inner loop to find max energy with current replicas
        for i in range(len(replicas)):
            
            if i == 0:
                max_energy = e(replicas[i])
                max_energy_idx = 0
                continue

            replica_energy = e(replicas[i])
            if replica_energy > max_energy:
                max_energy = replica_energy
                max_energy_idx = i

        #remove the max energy replica and replace it with a new one mc-walked from a random remaining replica
        max_energies.append(max_energy)
        del replicas[max_energy_idx]
        random_replica = random.choice(replicas)
        replicas.append(mc_walk_constrained(random_replica, 1, max_energy, e))


    plt.show()
    return max_energies

def free_energy():
    iterations = 1000
    k = 100
    sample = [random.uniform(0.0, 1.0) for i in range(k)]
    
    # ns_result = nested_sampling(multi_minima_func, iterations, sample, True)
    ns_result = nested_sampling(square_func, iterations, sample, False)

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

    ax.plot(x_axis, density_of_states)
    plt.show()

    return free_energies

free_energy()
