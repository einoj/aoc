def corrolate(image,kernel):
    # like convolution but without rotating the kernel 180 degrees
    l = len(kernel[0])
    h = len(kernel)
    li = len(image)
    kernel_sum = sum([kernel[i][j]**2 for i in range(l) for j in range(h)])
    result = 0
    for n in range(li-l+1):
        for m in range(li-h+1):
            corrolation = 0
            for i in range(l):
                for j in range(l):
                    corrolation += image[n+i][m+j]*kernel[i][j]
            if corrolation == kernel_sum:
                      result += 1
    return result 


def rot90(kern):
   return list(zip(*kern[::-1]))

import sys
image = []
for line in sys.stdin:                  
    line = line.rstrip()
    image.append([ord(i) for i in line])

kernel_part2 = [[ord('M'), ord('\0'), ord('M')], [ord('\0'), ord('A'), ord('\0')], [ord('S'), ord('\0'), ord('S')]]
result_part2 = 0
for i in range(4):
    result_part2 += corrolate(image,kernel_part2)
    kernel_part2 = rot90(kernel_part2)
          
print(f"Result part 2: {result_part2}")
