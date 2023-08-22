import random;

def square_func(x):
    return x**2


#1-dimensional global minima direct sampling monte carlo

min_sample = -100
max_sample = 100
iterations = 1000000

def direct_sampling_minima(f, start_min, min_sample, max_sample, iterations):
    min = start_min
    for i in range(0, iterations):
        x = random.uniform(min_sample, max_sample)
        val = f(x)
        if val < min:
            min = val
    return min

print("direct sampling minima" + str(direct_sampling_minima(square_func, 999999999999, min_sample, max_sample, iterations)))

#1-d markov chain monte carlo local minima finding
def mcmc_minima(f, start_min, min_sample, max_sample, iterations, walk_dist):
    min = start_min
    x = random.uniform(min_sample, max_sample)
    for i in range(0, iterations):
        val = f(x)
        if val < min:
            min = val
        move = random.uniform(-walk_dist, walk_dist)
        x = x + move
    return min

print("mcmc: minima" + str(mcmc_minima(square_func, 999999999999, min_sample, max_sample, iterations, 1)))


#univariate monte carlo integration
def direct_sampling_integral(f, min_sample, max_sample, iterations):
    sum_vals = 0
    for i in range(0, iterations):
        x = random.uniform(min_sample, max_sample)
        sum_vals += f(x)
    return (sum_vals / iterations) * (max_sample - min_sample)

print("direct sampling integral: " + str(direct_sampling_integral(square_func, 0, 2, iterations)))

#univariate markov chain monte carlo integration

def mcmc_integral(f, min_sample, max_sample, iterations, walk_dist):
    sum_vals = 0
    x = random.uniform(min_sample, max_sample)
    for i in range(0, iterations):
        sum_vals += f(x)
        x = gen_walk(x, walk_dist, min_sample, max_sample)
    return (sum_vals / iterations) * (max_sample - min_sample)

def gen_walk(x, walk_dist, min_sample, max_sample):
    move = random.uniform(-walk_dist, walk_dist)
    if x + move < min_sample or x + move > max_sample:
        return x
    else:
        return x + move

print("mcmc integral: " + str(mcmc_integral(square_func, 0, 2, iterations, 1)))
