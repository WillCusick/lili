# Generate the first 1000 primes in the format for a rust array

# 1000th prime is 7919, so we only have to go up to that
MAX_PRIME = 7919

def main():
    primes = [2]
    sieve = [True]*(MAX_PRIME+1)
    for i in range(3, MAX_PRIME+1, 2):
        if sieve[i]:
            primes.append(i)
        for j in range(i, MAX_PRIME, i):
            sieve[j] = False
    
    print(primes)



if __name__ == "__main__":
    main()