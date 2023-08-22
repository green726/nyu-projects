import random

#example energy function

def square_func(x):
    return x**2

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

test_result = nested_sampling(square_func, 1000, [random.uniform(0, 100) for i in range(100)])

print("nested sampling ran on square root function with 1000 iterations and 100 prior points:\n whole arrray: " + str(test_result) + "\n\n\n\n\n first: " + str(test_result[0]) + "\n last: " + str(test_result[-1]))
