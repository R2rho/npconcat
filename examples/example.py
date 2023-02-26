import npconcat
import numpy as np
import time


a = np.linspace(1,100,10000000)

# a=a.astype('str')
b = np.full(np.shape(a),'X ',dtype='U8').astype('str')
start = time.time()
a = npconcat.format(a,5)
print(f'time: {time.time()-start}')
start = time.time()
result = npconcat.concat2(a,b)
# # result = npconcat.concat3(bytearray(a),bytearray(b))
# # result = npconcat.sum_as_string(1,2)
print(f'time: {time.time()-start}')

start = time.time()
result = [f'{a}{b}' for a,b in zip(a,b)]
print(f'time: {time.time()-start}')

start = time.time()
result = np.char.add(a,b)
print(f'time: {time.time()-start}')
