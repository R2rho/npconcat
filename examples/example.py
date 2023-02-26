import npconcat
import numpy as np
import time
import itertools


a = np.linspace(1,100,10000000).astype('str').tolist()

# a=a.astype('str')
b = np.full(np.shape(a),'X ',dtype='U8').astype('str').tolist()
start = time.time()
# a = npconcat.format(a,5)
print(f'time: {time.time()-start}')
start = time.time()
result = npconcat.pass_through(a,b)
print(f'pass through time: {time.time()-start}')

# start = time.time()
# result = npconcat.pass_through_vec(a,b)
# print(f'pass through vec: {time.time()-start}')

# start = time.time()
# result = npconcat.concat6(a,b)
# print(f'concat time: {time.time()-start}')

start = time.time()
result = [f'{a}{b}' for a,b in zip(a,b)]
print(f'list comp time: {time.time()-start}')

# start = time.time()
# result = np.char.add(a,b)
# print(f'np char time: {time.time()-start}')

start = time.time()
result = list(map(''.join, itertools.zip_longest(a, b)))
print(f'itertools time: {time.time()-start}')
