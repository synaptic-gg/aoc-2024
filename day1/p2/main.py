list1 = []
list2 = []
with open('input.txt','r') as file:
    i = 0
    for line in file:
        list1_elements = line.split(" ")
        list1.append(int(list1_elements[0]))
        list2.append(int(list1_elements[3].replace('\n','')))
        i += 1

with open('input_san.txt','w') as file:
    L = [f"{list1} \n",f" length is {len(list1)} \n",f"{list2} \n",f" length is {len(list2)} \n"]
    file.writelines(L)
    file.close()
        
