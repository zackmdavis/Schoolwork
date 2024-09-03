from math import factorial, exp

def binomial(n, p, k):
    return (factorial(n)/(factorial(k)*factorial(n-k)))*(p**k*(1-p)**(n-k))

def poisson(λ, k):
    return λ**k * exp(-λ) / factorial(k)

scenarios = [
    (2, 8, 0.1),
    (9, 10, 0.95),
    (0, 10, 0.1),
    (4, 9, 0.2),
]

for x, n, p in scenarios:
    print("x={}, n={}, p={}. Binomial: {:.5}, Poisson: {:.5}".format(x, n, p, binomial(n, p, x), poisson(n*p, x)))

# for k in range(7):
#     print(k, binomial(6, 0.5, k))
