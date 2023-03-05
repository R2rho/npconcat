import numpy as np
import time
import sys
import npconcat

def fastconcat(arrays:list, precision:int=5, spaces:bool=True):

    primitives: list = ['int','float','bool','str']

    #VALIDATE inputs
    if arrays.__class__.__name__ !=  'list':
        raise TypeError('arrays argument must be a list of ndarrays or list of primitives (int, float, bool, str)')
    if not all([a.__class__.__name__ in ['list','ndarray'] for a in arrays]):
        raise Exception(f'All input arrays must be of type "list" or "ndarray"')

    for arr in arrays:
        if arr.__class__.__name__ == 'list':
            if not all([a.__class__.__name__ in primitives for a in arr]):
                raise TypeError(f'All input arrays must be 1-D ndarrays or lists of primitives: {primitives}')

    if precision.__class__.__name__ != 'int':
        raise TypeError('precision argument must be of type int')

    if spaces.__class__.__name__ != 'bool':
        raise TypeError('spaces argument must be of type bool')

    separator: bool = ' ' if spaces else ''
    maxLen: int =0
    minLen: int =0
    maxWidth: int=0
    threshold: int = sys.maxsize
    nl = np.full(np.shape(arrays[0]),'\n')

    for idx,arr in enumerate(arrays):
        if arr.__class__.__name__ == 'list':
            arrays[idx] = np.array(arr)
        if len(np.shape(arrays[idx])) != 1 and np.shape(arrays[idx])[1] != 0:
            raise Exception('Input arrays must be one-dimensional')

        if len(arrays[idx])>maxLen: maxLen = len(arr)
        if len(arrays[idx])<minLen or minLen == 0: minLen = len(arr)

        #SUPPRESS newline characters with maxwidth --> no opt-out options in np.array2string for max_line_width :(
        strWidth = len(np.array2string(arrays[idx],threshold=threshold))
        if strWidth > maxWidth: maxWidth = strWidth*len(arrays)*2
        
    if minLen != maxLen:
        raise Exception('All input arrays must be of the same length to be concatenated')
  
    #SET formatters
    precision_zero = float(f"0.{'0'*precision}1")
    if spaces:
        formatter = {'float_kind':lambda pos: f"{pos:.{precision}f}" if pos < precision_zero else f" {pos:.{precision}f}"}
    else:
        formatter = {'float_kind':lambda pos: f"{pos:.{precision}f}"}

    #INITIALIZE numpy array of concatenated arrays
    concat_arrays=np.empty((0,1))
    stacked_arrays=[]
    for a in arrays:
        stacked_arrays.append(np.array2string(a,formatter=formatter,max_line_width = maxWidth,separator=separator,threshold=threshold).replace('[','').replace(']','').replace("'","").replace('"',''))
        concat_arrays = np.append(concat_arrays,np.array2string(a,formatter=formatter,max_line_width = maxWidth,separator=separator,threshold=threshold).replace('[','').replace(']','').replace("'","").replace('"',''))
    
    concat_arrays = np.append(concat_arrays,np.array2string(nl,max_line_width = maxWidth))

    concat_arrays = np.array2string(concat_arrays.T,max_line_width=maxWidth, separator=separator).replace('[','').replace(']','').replace("'","").replace('"','')
    bt = np.array(bytearray(concat_arrays,'utf-8'))
    bt = np.reshape(bt,(1,len(bt))).T
    bt = bytes(bt).decode().split('|')
    return concat_arrays


def fastconcat2(arrays:list, precision:int=5, spaces:bool=True):

    primitives: list = ['int','float','bool','str']

    #VALIDATE inputs
    if arrays.__class__.__name__ !=  'list':
        raise TypeError('arrays argument must be a list of ndarrays or list of primitives (int, float, bool, str)')
    if not all([a.__class__.__name__ in ['list','ndarray'] for a in arrays]):
        raise Exception(f'All input arrays must be of type "list" or "ndarray"')

    for idx, arr in enumerate(arrays):
        if arr.__class__.__name__ == 'list':
            if not all([a.__class__.__name__ in primitives for a in arr]):
                raise TypeError(f'All input arrays must be 1-D ndarrays or lists of primitives: {primitives}')
        if arr.__class__.__name__ == 'ndarray':
            arrays[idx] = list(arr)

    if precision.__class__.__name__ != 'int':
        raise TypeError('precision argument must be of type int')

    if spaces.__class__.__name__ != 'bool':
        raise TypeError('spaces argument must be of type bool')
    separator: bool = ' ' if spaces else ''
    result = npconcat.concat_pylists3(arrays,5,separator)
    # result = npconcat.concat_pylists(*arrays[0:3],5,separator)
    return result

class Axis:
    def __init__(self,name,positions):
        self.name=name
        self.positions=positions

size = 1000000
x = np.linspace(-100,0,size)
y = np.linspace(100,200,size)
z = np.linspace(200,300,size)

shape = np.shape(x)

X = 'X'
Y = 'Y'
Z = 'Z'
nl = np.full(shape,'\n')

axes=[Axis(X,x),
      Axis(Y,y),
      Axis(Z,z)]

start = time.time()
positions=[]
for axis in axes:
    positions.append([f'{axis.name} {pos:.5f}' for pos in axis.positions])

axis_positions = list(map(lambda x : ' '.join(x),zip(*positions)))

print(f'listcomp: {time.time()-start}')

axis_name=''
width = len(x)*2000
precision=5
precision_zero = float(f"0.{'0'*precision}1")
formatter = {'float_kind':lambda pos: f"{axis_name}{pos:.5f}" if pos < precision_zero else f"{axis_name} {pos:.5f}"}

start = time.time()
positions=np.empty((0,0))
# for a in axes:
#     axis_name = a.name
#     positions=np.append(positions,np.array2string(a.positions,formatter=formatter,max_line_width = width*20).replace('[','').replace(']',''))

# positions = np.append(positions,np.array2string(nl,max_line_width = width))
# p = np.array2string(positions.T,max_line_width=len(positions[0])*len(axes)*2).replace('[','').replace(']','').replace("'","")
print(f'numpy_append: {time.time()-start}')

X = np.full(shape,'X')
Y = np.full(shape,'Y')
Z = np.full(shape,'Z')

start = time.time()
arrays = [X,x,Y,y,Z,z]
p = fastconcat2(arrays)
print(f'fastconcat: {time.time()-start}')

print('done')


