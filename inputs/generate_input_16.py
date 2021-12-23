import sys

def generate(n):
    if n == 0:
        return "1201"
    else:
        return "0A01040080" + 2*generate(n-1)

n=int(sys.argv[1])
print(generate(n))